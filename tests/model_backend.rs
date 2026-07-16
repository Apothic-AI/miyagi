use std::env;

use miyagi::TokenMode;
use miyagi::backend::{BackendConfig, MiyagiBackend, WwamaBackend};
use miyagi::probe::{compile_probes, math_probes, measure_probes};

#[test]
fn bonsai_q1_model_mutation_restores_logits() {
    let Some(path) = env::var_os("MIYAGI_TEST_MODEL") else {
        return;
    };
    let mut backend = WwamaBackend::load(
        path.to_str().unwrap(),
        BackendConfig {
            n_ctx: 256,
            n_batch: 64,
            n_ubatch: 64,
            n_gpu_layers: env::var("MIYAGI_TEST_GPU_LAYERS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(0),
            ..BackendConfig::default()
        },
    )
    .unwrap();
    assert!(backend.architecture().layer_count() >= 1);
    let probes =
        compile_probes(&backend, &math_probes(), TokenMode::LastTokenCompatibility).unwrap();
    let baseline = measure_probes(&mut backend, &probes).unwrap();
    let scales = backend.row_scales(0, miyagi::Projection::Gate).unwrap();
    assert_eq!(scales.len(), 12288);
    let tensor = backend
        .architecture()
        .tensor(0, miyagi::Projection::Gate)
        .unwrap()
        .clone();
    let row_offset = 0;
    let row_bytes = tensor.strides[0] * (tensor.width / 128);
    let bytes_before = backend
        .session_mut()
        .read_tensor_range(&tensor.name, row_offset, row_bytes)
        .unwrap();
    backend.flip_row(0, miyagi::Projection::Gate, 0).unwrap();
    let scales_after = backend.row_scales(0, miyagi::Projection::Gate).unwrap();
    let bytes_mutated = backend
        .session_mut()
        .read_tensor_range(&tensor.name, row_offset, row_bytes)
        .unwrap();
    let mutated = measure_probes(&mut backend, &probes).unwrap();
    backend.flip_row(0, miyagi::Projection::Gate, 0).unwrap();
    let bytes_restored = backend
        .session_mut()
        .read_tensor_range(&tensor.name, row_offset, row_bytes)
        .unwrap();
    let restored = measure_probes(&mut backend, &probes).unwrap();
    assert_eq!(scales_after, scales);
    assert_ne!(bytes_mutated, bytes_before);
    assert_eq!(bytes_restored, bytes_before);
    assert_ne!(mutated[0].gap, baseline[0].gap);
    for (before, after) in baseline.iter().zip(restored) {
        assert_eq!(before.gap, after.gap, "probe {} drifted", before.name);
    }
}

#[test]
fn bonsai_q1_architecture_is_not_assumed_to_be_8b() {
    let Some(path) = env::var_os("MIYAGI_TEST_27B_MODEL") else {
        return;
    };
    let backend = WwamaBackend::load(
        path.to_str().unwrap(),
        BackendConfig {
            mutable_tensors: false,
            n_gpu_layers: 0,
            ..BackendConfig::default()
        },
    )
    .unwrap();
    assert!(backend.architecture().layer_count() > 36);
}
