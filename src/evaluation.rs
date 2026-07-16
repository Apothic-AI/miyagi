use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::probe::ProbeMeasurement;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignTransition {
    Fixed,
    Broke,
    StayedRight,
    StayedWrong,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProbeDelta {
    pub name: String,
    pub category: String,
    pub baseline: f32,
    pub patched: f32,
    pub delta: f32,
    pub transition: SignTransition,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EvaluationSummary {
    pub probes: Vec<ProbeDelta>,
    pub fixed: usize,
    pub broke: usize,
    pub stayed_right: usize,
    pub stayed_wrong: usize,
    pub improved: usize,
    pub degraded: usize,
    pub unchanged: usize,
    pub by_category: BTreeMap<String, CategorySummary>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CategorySummary {
    pub fixed: usize,
    pub broke: usize,
    pub stayed_right: usize,
    pub stayed_wrong: usize,
}

pub fn compare_measurements(
    baseline: &[ProbeMeasurement],
    patched: &[ProbeMeasurement],
    change_threshold: f32,
) -> Result<EvaluationSummary> {
    if !change_threshold.is_finite() || change_threshold < 0.0 {
        return Err(Error::MeasurementMismatch(
            "change threshold must be finite and non-negative".to_owned(),
        ));
    }
    if baseline.len() != patched.len() || baseline.is_empty() {
        return Err(Error::MeasurementMismatch(
            "baseline and patched measurements must be non-empty and aligned".to_owned(),
        ));
    }
    let mut summary = EvaluationSummary::default();
    for (baseline, patched) in baseline.iter().zip(patched) {
        if baseline.name != patched.name {
            return Err(Error::MeasurementMismatch(format!(
                "expected probe {}, got {}",
                baseline.name, patched.name
            )));
        }
        let transition = match (baseline.gap > 0.0, patched.gap > 0.0) {
            (false, true) => SignTransition::Fixed,
            (true, false) => SignTransition::Broke,
            (true, true) => SignTransition::StayedRight,
            (false, false) => SignTransition::StayedWrong,
        };
        match transition {
            SignTransition::Fixed => summary.fixed += 1,
            SignTransition::Broke => summary.broke += 1,
            SignTransition::StayedRight => summary.stayed_right += 1,
            SignTransition::StayedWrong => summary.stayed_wrong += 1,
        }
        let category = summary
            .by_category
            .entry(baseline.category.clone())
            .or_default();
        match transition {
            SignTransition::Fixed => category.fixed += 1,
            SignTransition::Broke => category.broke += 1,
            SignTransition::StayedRight => category.stayed_right += 1,
            SignTransition::StayedWrong => category.stayed_wrong += 1,
        }
        let delta = patched.gap - baseline.gap;
        if delta > change_threshold {
            summary.improved += 1;
        } else if delta < -change_threshold {
            summary.degraded += 1;
        } else {
            summary.unchanged += 1;
        }
        summary.probes.push(ProbeDelta {
            name: baseline.name.clone(),
            category: baseline.category.clone(),
            baseline: baseline.gap,
            patched: patched.gap,
            delta,
            transition,
        });
    }
    Ok(summary)
}
