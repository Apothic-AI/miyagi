use serde::{Deserialize, Serialize};

use crate::architecture::{ArchitectureMap, Projection};
use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenMode {
    #[default]
    LastTokenCompatibility,
    StrictSingle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BackendConfig {
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_ubatch: u32,
    pub n_threads: i32,
    pub n_threads_batch: i32,
    pub n_gpu_layers: i32,
    pub mutable_tensors: bool,
    pub add_special: bool,
    pub parse_special: bool,
}

impl BackendConfig {
    pub fn session_options(&self) -> wwama::SessionOptions {
        wwama::SessionOptions {
            n_ctx: self.n_ctx,
            n_batch: self.n_batch,
            n_ubatch: self.n_ubatch,
            n_threads: self.n_threads,
            n_threads_batch: self.n_threads_batch,
            n_gpu_layers: self.n_gpu_layers,
            mutable_tensors: self.mutable_tensors,
            ..wwama::SessionOptions::default()
        }
    }
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            n_ctx: 2048,
            n_batch: 512,
            n_ubatch: 512,
            n_threads: 0,
            n_threads_batch: 0,
            n_gpu_layers: 999,
            mutable_tensors: true,
            add_special: false,
            parse_special: true,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GenerateConfig {
    pub max_new_tokens: usize,
    pub temperature: f32,
    pub top_k: i32,
    pub top_p: f32,
    pub seed: u32,
}

impl Default for GenerateConfig {
    fn default() -> Self {
        Self {
            max_new_tokens: 100,
            temperature: 0.0,
            top_k: 40,
            top_p: 0.95,
            seed: 42,
        }
    }
}

pub trait MiyagiBackend {
    fn architecture(&self) -> &ArchitectureMap;
    fn model_label(&self) -> &str;
    fn tokenize(&self, text: &str) -> Result<Vec<i32>>;
    fn row_scales(&mut self, layer: usize, projection: Projection) -> Result<Vec<f32>>;
    fn flip_row(&mut self, layer: usize, projection: Projection, row: usize) -> Result<()>;
    fn logit_gap(&mut self, prompt: &[i32], correct: i32, wrong: i32) -> Result<f32>;
    fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<String>;
}

pub struct WwamaBackend {
    session: wwama::Session,
    architecture: ArchitectureMap,
    model_label: String,
    config: BackendConfig,
}

impl WwamaBackend {
    pub fn load(path: &str, config: BackendConfig) -> Result<Self> {
        let options = config.session_options();
        let session = wwama::Session::load_from_path(path, options)?;
        let descriptors = session.model().tensors()?;
        let architecture = ArchitectureMap::discover(&descriptors)?;
        Ok(Self {
            session,
            architecture,
            model_label: path.to_owned(),
            config,
        })
    }

    pub fn resolve_answer_token(
        &self,
        probe_name: &str,
        text: &str,
        mode: TokenMode,
    ) -> Result<i32> {
        let tokens = self.tokenize(text)?;
        if tokens.is_empty() {
            return Err(Error::EmptyAnswerToken {
                probe: probe_name.to_owned(),
            });
        }
        if mode == TokenMode::StrictSingle && tokens.len() != 1 {
            return Err(Error::AmbiguousAnswerToken {
                probe: probe_name.to_owned(),
                count: tokens.len(),
            });
        }
        Ok(*tokens.last().expect("non-empty token list checked above"))
    }

    pub fn session(&self) -> &wwama::Session {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut wwama::Session {
        &mut self.session
    }

    pub fn config(&self) -> &BackendConfig {
        &self.config
    }
}

impl MiyagiBackend for WwamaBackend {
    fn architecture(&self) -> &ArchitectureMap {
        &self.architecture
    }

    fn model_label(&self) -> &str {
        &self.model_label
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i32>> {
        Ok(self
            .session
            .tokenize_text(text, self.config.add_special, self.config.parse_special)?)
    }

    fn row_scales(&mut self, layer: usize, projection: Projection) -> Result<Vec<f32>> {
        let name = self.architecture.tensor(layer, projection)?.name.clone();
        Ok(self.session.q1_0_row_scales(&name)?)
    }

    fn flip_row(&mut self, layer: usize, projection: Projection, row: usize) -> Result<()> {
        let name = self.architecture.tensor(layer, projection)?.name.clone();
        self.session.xor_q1_0_row(&name, row)?;
        Ok(())
    }

    fn logit_gap(&mut self, prompt: &[i32], correct: i32, wrong: i32) -> Result<f32> {
        Ok(self.session.logit_gap(prompt, correct, wrong)?)
    }

    fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<String> {
        let options = wwama::GenerationOptions {
            max_new_tokens: config.max_new_tokens,
            temperature: config.temperature,
            top_k: config.top_k,
            top_p: config.top_p,
            seed: config.seed,
            add_special: self.config.add_special,
            parse_special: self.config.parse_special,
            ..wwama::GenerationOptions::default()
        };
        Ok(self.session.generate_text(prompt, &options)?.text)
    }
}
