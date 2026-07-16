pub mod architecture;
pub mod backend;
pub mod cli;
pub mod dataset;
pub mod error;
pub mod evaluation;
pub mod fitness;
pub mod patch;
pub mod probe;
pub mod search;

pub use architecture::{ArchitectureMap, Projection, TensorInfo};
pub use backend::{BackendConfig, GenerateConfig, MiyagiBackend, TokenMode, WwamaBackend};
pub use error::{Error, Result};
pub use fitness::FitnessMode;
pub use patch::{Patch, PatchFlip, PatchValidation, ValidatedPatch};
pub use probe::{CompiledProbe, Probe, ProbeMeasurement};
pub use search::{ModelPatchState, SearchCheckpoint, SearchConfig, SearchResult};
