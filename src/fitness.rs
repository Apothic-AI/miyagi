use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::probe::ProbeMeasurement;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessMode {
    #[default]
    Mean,
    Min,
}

pub fn compute_fitness(
    mode: FitnessMode,
    target: &[ProbeMeasurement],
    control: &[ProbeMeasurement],
    target_baseline: &[ProbeMeasurement],
    control_baseline: &[ProbeMeasurement],
    control_penalty: f32,
) -> Result<f32> {
    if !control_penalty.is_finite() || control_penalty < 0.0 {
        return Err(Error::InvalidFitness(
            "control penalty must be finite and non-negative".to_owned(),
        ));
    }
    let target_improvements = aligned_deltas(target, target_baseline, false)?;
    let control_degradation = aligned_deltas(control, control_baseline, true)?
        .into_iter()
        .sum::<f32>()
        / control.len() as f32;
    let target_score = match mode {
        FitnessMode::Mean => {
            target_improvements.iter().sum::<f32>() / target_improvements.len() as f32
        }
        FitnessMode::Min => target_improvements
            .into_iter()
            .reduce(f32::min)
            .ok_or_else(|| Error::InvalidFitness("target set is empty".to_owned()))?,
    };
    let fitness = target_score - control_penalty * control_degradation;
    if !fitness.is_finite() {
        return Err(Error::InvalidFitness(
            "fitness result is non-finite".to_owned(),
        ));
    }
    Ok(fitness)
}

fn aligned_deltas(
    current: &[ProbeMeasurement],
    baseline: &[ProbeMeasurement],
    degradation: bool,
) -> Result<Vec<f32>> {
    if current.is_empty() || baseline.is_empty() || current.len() != baseline.len() {
        return Err(Error::MeasurementMismatch(
            "measurement sets must be non-empty and have equal lengths".to_owned(),
        ));
    }
    current
        .iter()
        .zip(baseline)
        .map(|(current, baseline)| {
            if current.name != baseline.name {
                return Err(Error::MeasurementMismatch(format!(
                    "expected probe {}, got {}",
                    baseline.name, current.name
                )));
            }
            if !current.gap.is_finite() || !baseline.gap.is_finite() {
                return Err(Error::InvalidFitness(format!(
                    "probe {} has non-finite gap",
                    current.name
                )));
            }
            Ok(if degradation {
                (baseline.gap - current.gap).max(0.0)
            } else {
                current.gap - baseline.gap
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(name: &str, gap: f32) -> ProbeMeasurement {
        ProbeMeasurement {
            name: name.to_owned(),
            category: String::new(),
            prompt: String::new(),
            correct_token: String::new(),
            wrong_token: String::new(),
            correct_id: 1,
            wrong_id: 2,
            gap,
        }
    }

    #[test]
    fn mean_fitness_matches_bankai_formula() {
        let target_base = [measurement("a", 1.0), measurement("b", -1.0)];
        let target = [measurement("a", 2.0), measurement("b", 1.0)];
        let control_base = [measurement("c", 3.0), measurement("d", 2.0)];
        let control = [measurement("c", 2.0), measurement("d", 3.0)];
        let result = compute_fitness(
            FitnessMode::Mean,
            &target,
            &control,
            &target_base,
            &control_base,
            2.0,
        )
        .unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn min_mode_uses_worst_target_improvement() {
        let baseline = [measurement("a", 0.0), measurement("b", 0.0)];
        let current = [measurement("a", 3.0), measurement("b", -1.0)];
        let control = [measurement("c", 1.0)];
        assert_eq!(
            compute_fitness(
                FitnessMode::Min,
                &current,
                &control,
                &baseline,
                &control,
                2.0
            )
            .unwrap(),
            -1.0
        );
    }
}
