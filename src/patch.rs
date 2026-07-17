use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::architecture::{ArchitectureMap, Projection};
use crate::backend::MiyagiBackend;
use crate::error::{Error, Result};

pub const PATCH_VERSION: u32 = 1;
pub const PATCH_FORMAT: &str = "miyagi_row_xor_v1";

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PatchFlip {
    pub layer: usize,
    pub proj: Projection,
    pub row: usize,
}

impl PatchFlip {
    pub fn coordinate(&self) -> String {
        format!("L{}.{}[{}]", self.layer, self.proj, self.row)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PatchStats {
    #[serde(default)]
    pub n_flips: usize,
    #[serde(default, alias = "bits_flipped")]
    pub logical_bits_flipped: u64,
    #[serde(default, alias = "size_bytes")]
    pub compact_binary_estimate_bytes: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Patch {
    pub version: u32,
    pub format: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub base_model: String,
    pub flips: Vec<PatchFlip>,
    pub stats: PatchStats,
    #[serde(default)]
    pub metadata: BTreeMap<String, Value>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PatchValidation {
    pub allow_model_signature_mismatch: bool,
}

#[derive(Clone, Debug)]
pub struct ValidatedPatch {
    patch: Patch,
    logical_bits_flipped: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct RawPatch {
    #[serde(default = "default_patch_version")]
    version: u32,
    format: Option<String>,
    #[serde(rename = "type")]
    legacy_type: Option<String>,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default = "default_base_model")]
    base_model: String,
    flips: Vec<PatchFlip>,
    stats: Option<PatchStats>,
    #[serde(default)]
    metadata: BTreeMap<String, Value>,
}

fn default_patch_version() -> u32 {
    PATCH_VERSION
}

fn default_base_model() -> String {
    "unknown".to_owned()
}

impl Patch {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        base_model: impl Into<String>,
        flips: Vec<PatchFlip>,
    ) -> Self {
        Self {
            version: PATCH_VERSION,
            format: PATCH_FORMAT.to_owned(),
            name: name.into(),
            description: description.into(),
            base_model: base_model.into(),
            stats: PatchStats {
                n_flips: flips.len(),
                logical_bits_flipped: 0,
                compact_binary_estimate_bytes: flips.len() * 12,
            },
            flips,
            metadata: BTreeMap::new(),
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = fs::read_to_string(path)?;
        let fallback_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unnamed");
        Self::from_json_str_with_name(&text, fallback_name)
    }

    pub fn from_json_str(text: &str) -> Result<Self> {
        Self::from_json_str_with_name(text, "unnamed")
    }

    fn from_json_str_with_name(text: &str, fallback_name: &str) -> Result<Self> {
        let raw: RawPatch = serde_json::from_str(text)?;
        if raw.version != PATCH_VERSION {
            return Err(Error::UnsupportedPatchVersion(raw.version));
        }
        let format = match (raw.format.as_deref(), raw.legacy_type.as_deref()) {
            (Some(PATCH_FORMAT), _) => PATCH_FORMAT,
            (None, Some("row_flip")) => PATCH_FORMAT,
            (Some(other), _) => return Err(Error::UnsupportedPatchFormat(other.to_owned())),
            (None, Some(other)) => return Err(Error::UnsupportedPatchFormat(other.to_owned())),
            (None, None) => {
                return Err(Error::InvalidPatch(
                    "patch requires format or legacy type".to_owned(),
                ));
            }
        };
        let name = if raw.name.trim().is_empty() {
            fallback_name.to_owned()
        } else {
            raw.name
        };
        let mut patch = Self::new(name, raw.description, raw.base_model, raw.flips);
        patch.format = format.to_owned();
        if let Some(stats) = raw.stats {
            patch.stats = stats;
        }
        patch.metadata = raw.metadata;
        Ok(patch)
    }

    pub fn validate(
        &self,
        architecture: &ArchitectureMap,
        options: PatchValidation,
    ) -> Result<ValidatedPatch> {
        if self.version != PATCH_VERSION {
            return Err(Error::UnsupportedPatchVersion(self.version));
        }
        if self.format != PATCH_FORMAT {
            return Err(Error::UnsupportedPatchFormat(self.format.clone()));
        }
        if self.name.trim().is_empty() {
            return Err(Error::InvalidPatch("patch name is empty".to_owned()));
        }
        if let Some(Value::String(signature)) = self.metadata.get("architecture_signature")
            && signature != architecture.signature()
            && !options.allow_model_signature_mismatch
        {
            return Err(Error::ModelSignatureMismatch {
                patch: signature.clone(),
                model: architecture.signature().to_owned(),
            });
        }

        let mut seen = BTreeSet::new();
        let mut logical_bits_flipped = 0_u64;
        for flip in &self.flips {
            if !seen.insert(flip.clone()) {
                return Err(Error::DuplicateFlip(flip.coordinate()));
            }
            let tensor = architecture.tensor(flip.layer, flip.proj)?;
            if flip.row >= tensor.rows {
                return Err(Error::InvalidPatch(format!(
                    "{} exceeds row count {}",
                    flip.coordinate(),
                    tensor.rows
                )));
            }
            logical_bits_flipped = logical_bits_flipped
                .checked_add(tensor.width as u64)
                .ok_or_else(|| Error::InvalidPatch("logical bit count overflow".to_owned()))?;
        }

        let mut patch = self.clone();
        patch.stats = PatchStats {
            n_flips: patch.flips.len(),
            logical_bits_flipped,
            compact_binary_estimate_bytes: patch.flips.len() * 12,
        };
        Ok(ValidatedPatch {
            patch,
            logical_bits_flipped,
        })
    }

    pub fn compose(name: impl Into<String>, patches: &[Patch]) -> Result<Self> {
        if patches.is_empty() {
            return Err(Error::InvalidPatch(
                "at least one patch is required for composition".to_owned(),
            ));
        }
        let mut flips = BTreeSet::new();
        for patch in patches {
            for flip in &patch.flips {
                if !flips.insert(flip.clone()) {
                    flips.remove(flip);
                }
            }
        }
        if flips.is_empty() {
            return Err(Error::InvalidPatch(
                "composed patch cancels to an empty XOR mask".to_owned(),
            ));
        }
        let base_models = patches
            .iter()
            .map(|patch| patch.base_model.as_str())
            .collect::<BTreeSet<_>>();
        if base_models.len() != 1 {
            return Err(Error::InvalidPatch(
                "cannot compose patches with different base_model values".to_owned(),
            ));
        }
        let mut patch = Self::new(
            name,
            format!("XOR composition of {} patches", patches.len()),
            patches[0].base_model.clone(),
            flips.into_iter().collect(),
        );
        patch.metadata.insert(
            "composed_from".to_owned(),
            Value::Array(
                patches
                    .iter()
                    .map(|patch| Value::String(patch.name.clone()))
                    .collect(),
            ),
        );
        Ok(patch)
    }

    pub fn to_pretty_json(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn save_atomic(&self, path: impl AsRef<Path>) -> Result<usize> {
        let path = path.as_ref();
        let bytes = self.to_pretty_json()?;
        let temporary = temporary_path(path)?;
        let write_result = (|| -> Result<()> {
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temporary)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
            fs::rename(&temporary, path)?;
            Ok(())
        })();
        if write_result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        write_result?;
        Ok(bytes.len())
    }
}

impl ValidatedPatch {
    pub fn patch(&self) -> &Patch {
        &self.patch
    }

    pub fn into_patch(self) -> Patch {
        self.patch
    }

    pub fn logical_bits_flipped(&self) -> u64 {
        self.logical_bits_flipped
    }

    pub fn apply<B: MiyagiBackend>(&self, backend: &mut B) -> Result<()> {
        apply_flips(backend, &self.patch.flips, "patch application")
    }

    pub fn remove<B: MiyagiBackend>(&self, backend: &mut B) -> Result<()> {
        apply_flips(backend, &self.patch.flips, "patch removal")
    }
}

fn apply_flips<B: MiyagiBackend>(
    backend: &mut B,
    flips: &[PatchFlip],
    operation: &str,
) -> Result<()> {
    let mut applied: Vec<PatchFlip> = Vec::new();
    for flip in flips {
        if let Err(error) = backend.flip_row(flip.layer, flip.proj, flip.row) {
            for applied_flip in applied.iter().rev() {
                if let Err(source) =
                    backend.flip_row(applied_flip.layer, applied_flip.proj, applied_flip.row)
                {
                    return Err(Error::RestorationFailed {
                        operation: operation.to_owned(),
                        source: Box::new(source),
                    });
                }
            }
            return Err(error);
        }
        applied.push(flip.clone());
    }
    Ok(())
}

fn temporary_path(path: &Path) -> Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or_else(|| Error::MissingFileName(path.to_owned()))?
        .to_string_lossy();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    for attempt in 0..100_u32 {
        let candidate = parent.join(format!(".{name}.{}.{}.tmp", std::process::id(), attempt));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(Error::InvalidPatch(
        "could not allocate an atomic output path".to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_alternate_schema_and_emits_canonical_format() {
        let patch = Patch::from_json_str(
            r#"{
                "version": 1,
                "type": "row_flip",
                "base_model": "bonsai",
                "flips": [{"layer": 1, "proj": "gate_proj", "row": 7}]
            }"#,
        )
        .unwrap();
        assert_eq!(patch.format, PATCH_FORMAT);
        assert_eq!(patch.name, "unnamed");
        let output = String::from_utf8(patch.to_pretty_json().unwrap()).unwrap();
        assert!(output.contains("\"format\": \"miyagi_row_xor_v1\""));
        assert!(!output.contains("\"type\""));
    }

    #[test]
    fn composition_uses_xor_symmetric_difference() {
        let shared = PatchFlip {
            layer: 1,
            proj: Projection::Gate,
            row: 2,
        };
        let unique = PatchFlip {
            layer: 2,
            proj: Projection::Up,
            row: 3,
        };
        let first = Patch::new("a", "", "model", vec![shared.clone()]);
        let second = Patch::new("b", "", "model", vec![shared, unique.clone()]);
        let composed = Patch::compose("combined", &[first, second]).unwrap();
        assert_eq!(composed.flips, vec![unique]);
    }
}
