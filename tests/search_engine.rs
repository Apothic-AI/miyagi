use std::collections::BTreeSet;
use std::sync::atomic::AtomicBool;

use miyagi::architecture::ArchitectureMap;
use miyagi::backend::{GenerateConfig, MiyagiBackend};
use miyagi::probe::{CompiledProbe, Probe};
use miyagi::search::{SearchConfig, run_search};
use miyagi::{Projection, Result};

struct FakeBackend {
    architecture: ArchitectureMap,
    flipped: BTreeSet<(usize, Projection, usize)>,
}

impl FakeBackend {
    fn new() -> Self {
        let descriptor = wwama::TensorDescriptor {
            name: "blk.0.ffn_gate.weight".to_owned(),
            type_id: 41,
            type_name: "Q1_0".to_owned(),
            dimensions: [128, 3, 1, 1],
            strides: [18, 18, 0, 0],
            n_dims: 2,
            nbytes: 54,
            backend: "fake".to_owned(),
        };
        let mut descriptors = vec![descriptor.clone()];
        descriptors.push(wwama::TensorDescriptor {
            name: "blk.0.ffn_up.weight".to_owned(),
            ..descriptor.clone()
        });
        descriptors.push(wwama::TensorDescriptor {
            name: "blk.0.ffn_down.weight".to_owned(),
            dimensions: [128, 3, 1, 1],
            ..descriptor
        });
        Self {
            architecture: ArchitectureMap::discover(&descriptors).unwrap(),
            flipped: BTreeSet::new(),
        }
    }
}

impl MiyagiBackend for FakeBackend {
    fn architecture(&self) -> &ArchitectureMap {
        &self.architecture
    }

    fn model_label(&self) -> &str {
        "fake"
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i32>> {
        Ok(vec![if text == "target" { 1 } else { 0 }])
    }

    fn row_scales(&mut self, _layer: usize, _projection: Projection) -> Result<Vec<f32>> {
        Ok(vec![1.0, 1.0, 1.0])
    }

    fn flip_row(&mut self, layer: usize, projection: Projection, row: usize) -> Result<()> {
        let key = (layer, projection, row);
        if !self.flipped.insert(key) {
            self.flipped.remove(&key);
        }
        Ok(())
    }

    fn logit_gap(&mut self, prompt: &[i32], _correct: i32, _wrong: i32) -> Result<f32> {
        let target = prompt.first() == Some(&1);
        let row0 = self.flipped.contains(&(0, Projection::Gate, 0));
        let row1 = self.flipped.contains(&(0, Projection::Gate, 1));
        if target {
            Ok(-1.0
                + if row0 {
                    2.0
                } else if row1 {
                    -2.0
                } else {
                    0.0
                })
        } else {
            Ok(1.0 - if row1 { 3.0 } else { 0.0 })
        }
    }

    fn generate(&mut self, _prompt: &str, _config: &GenerateConfig) -> Result<String> {
        Ok(String::new())
    }
}

fn compiled(name: &str, prompt: &str) -> CompiledProbe {
    CompiledProbe {
        probe: Probe::new(prompt, "yes", "no", name, "test"),
        prompt_tokens: vec![if prompt == "target" { 1 } else { 0 }],
        correct_id: 1,
        wrong_id: 2,
    }
}

#[test]
fn search_keeps_only_improving_flips_and_reverts_rejections() {
    let mut backend = FakeBackend::new();
    let config = SearchConfig {
        search_layers: vec![0],
        search_projections: vec![Projection::Gate],
        max_iters: 3,
        screen_probe_count: 1,
        patch_name: "test".to_owned(),
        base_model: "fake".to_owned(),
        ..SearchConfig::default()
    };
    let result = run_search(
        &mut backend,
        &[compiled("target", "target")],
        &[compiled("control", "control")],
        config,
        None,
        None,
        None,
        |_| {},
    )
    .unwrap();
    assert!(result.patch.flips.iter().any(|flip| flip.row == 0));
    assert_eq!(
        result.model_state,
        miyagi::search::ModelPatchState::AcceptedPatchApplied
    );
    assert!(backend.flipped.contains(&(0, Projection::Gate, 0)));
    assert!(!backend.flipped.contains(&(0, Projection::Gate, 1)));
}

#[test]
fn cancellation_is_reported_without_losing_current_state() {
    let mut backend = FakeBackend::new();
    let cancelled = AtomicBool::new(true);
    let result = run_search(
        &mut backend,
        &[compiled("target", "target")],
        &[compiled("control", "control")],
        SearchConfig {
            search_layers: vec![0],
            search_projections: vec![Projection::Gate],
            max_iters: 3,
            ..SearchConfig::default()
        },
        None,
        None,
        Some(&cancelled),
        |_| {},
    );
    assert!(matches!(result, Err(miyagi::Error::SearchCancelled)));
    assert!(backend.flipped.is_empty());
}
