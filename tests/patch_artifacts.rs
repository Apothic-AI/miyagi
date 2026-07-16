use std::path::PathBuf;

use miyagi::{ArchitectureMap, Patch, PatchValidation, Projection};

fn synthetic_architecture() -> ArchitectureMap {
    let mut descriptors = Vec::new();
    for layer in 0..36 {
        for (projection, width, rows) in [
            ("ffn_gate", 4096_u64, 12288_u64),
            ("ffn_up", 4096_u64, 12288_u64),
            ("ffn_down", 12288_u64, 4096_u64),
        ] {
            descriptors.push(wwama::TensorDescriptor {
                name: format!("blk.{layer}.{projection}.weight"),
                type_id: 41,
                type_name: "Q1_0".to_owned(),
                dimensions: [width, rows, 1, 1],
                strides: [18, (width as usize / 128) * 18, 0, 0],
                n_dims: 2,
                nbytes: (width as usize / 128) * 18 * rows as usize,
                backend: "CPU".to_owned(),
            });
        }
    }
    ArchitectureMap::discover(&descriptors).unwrap()
}

#[test]
fn all_checked_in_bankai_patches_parse_and_validate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let architecture = synthetic_architecture();
    for name in [
        "patch_math_v1.json",
        "calculus_v1.json",
        "calculus_generalized_v1.json",
    ] {
        let path = root.join("../../python/bankai/patches").join(name);
        let patch = Patch::load(path).unwrap();
        let validated = patch
            .validate(
                &architecture,
                PatchValidation {
                    allow_model_signature_mismatch: true,
                },
            )
            .unwrap();
        assert_eq!(validated.patch().format, "bankai_row_xor_v1");
        assert!(!validated.patch().flips.is_empty());
    }
}

#[test]
fn legacy_projection_names_are_normalized() {
    let patch = Patch::from_json_str(
        r#"{"version":1,"type":"row_flip","base_model":"x","flips":[{"layer":0,"proj":"ffn_down","row":1}]}"#,
    )
    .unwrap();
    assert_eq!(patch.flips[0].proj, Projection::Down);
}
