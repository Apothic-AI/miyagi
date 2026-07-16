use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Wwama(#[from] wwama::Error),
    #[error("unsupported model architecture: {0}")]
    UnsupportedArchitecture(String),
    #[error("unsupported tensor for {coordinate}: {reason}")]
    UnsupportedTensor { coordinate: String, reason: String },
    #[error("missing tensor mapping for {0}")]
    MissingTensor(String),
    #[error("duplicate tensor mapping for {0}")]
    DuplicateTensor(String),
    #[error("invalid projection: {0}")]
    InvalidProjection(String),
    #[error("unsupported patch version {0}")]
    UnsupportedPatchVersion(u32),
    #[error("unsupported patch format: {0}")]
    UnsupportedPatchFormat(String),
    #[error("invalid patch: {0}")]
    InvalidPatch(String),
    #[error("duplicate patch flip: {0}")]
    DuplicateFlip(String),
    #[error("patch model signature {patch} does not match loaded model signature {model}")]
    ModelSignatureMismatch { patch: String, model: String },
    #[error("probe set is empty")]
    EmptyProbeSet,
    #[error("duplicate probe name: {0}")]
    DuplicateProbe(String),
    #[error("probe {probe} has an empty {field}")]
    EmptyProbeField { probe: String, field: &'static str },
    #[error("probe {probe} answer tokenizes to {count} tokens in strict mode")]
    AmbiguousAnswerToken { probe: String, count: usize },
    #[error("probe {probe} answer tokenized to no tokens")]
    EmptyAnswerToken { probe: String },
    #[error("measurement mismatch: {0}")]
    MeasurementMismatch(String),
    #[error("fitness input is invalid: {0}")]
    InvalidFitness(String),
    #[error("search configuration is invalid: {0}")]
    InvalidSearch(String),
    #[error("search was cancelled")]
    SearchCancelled,
    #[error("search checkpoint is incompatible: {0}")]
    IncompatibleCheckpoint(String),
    #[error("failed to restore model state after {operation}: {source}")]
    RestorationFailed {
        operation: String,
        #[source]
        source: Box<Error>,
    },
    #[error("dataset record {index} is invalid: {reason}")]
    InvalidDatasetRecord { index: usize, reason: String },
    #[error("regular expression did not contain capture group 1: {0}")]
    MissingRegexCapture(String),
    #[error("path has no file name: {0}")]
    MissingFileName(PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidProjection(_)
            | Self::UnsupportedPatchVersion(_)
            | Self::UnsupportedPatchFormat(_)
            | Self::InvalidPatch(_)
            | Self::DuplicateFlip(_)
            | Self::EmptyProbeSet
            | Self::DuplicateProbe(_)
            | Self::EmptyProbeField { .. }
            | Self::AmbiguousAnswerToken { .. }
            | Self::EmptyAnswerToken { .. }
            | Self::InvalidFitness(_)
            | Self::InvalidSearch(_)
            | Self::IncompatibleCheckpoint(_)
            | Self::InvalidDatasetRecord { .. }
            | Self::MissingRegexCapture(_) => 2,
            Self::UnsupportedArchitecture(_)
            | Self::UnsupportedTensor { .. }
            | Self::MissingTensor(_)
            | Self::DuplicateTensor(_)
            | Self::ModelSignatureMismatch { .. }
            | Self::Wwama(_) => 3,
            Self::MeasurementMismatch(_) | Self::RestorationFailed { .. } => 4,
            Self::SearchCancelled => 130,
            Self::Io(_) | Self::Json(_) | Self::MissingFileName(_) => 5,
        }
    }
}
