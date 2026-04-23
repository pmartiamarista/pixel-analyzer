use crate::error::AnalyzerError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Quality {
    Draft = 0,
    #[default]
    Balanced = 1,
    Precise = 2,
}

impl Quality {
    pub fn sample_fraction(self) -> f32 {
        match self {
            Quality::Draft => 0.25,
            Quality::Balanced => 0.50,
            Quality::Precise => 1.00,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub max_colors: u8,

    pub quality: Quality,

    pub convergence_threshold: f32,

    pub max_iterations: u32,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_colors: 5,
            quality: Quality::default(),
            convergence_threshold: 1.0,
            max_iterations: 100,
        }
    }
}

impl AnalysisConfig {
    pub fn validate(&self) -> Result<(), AnalyzerError> {
        if !(2..=16).contains(&self.max_colors) {
            return Err(AnalyzerError::InvalidConfig(format!(
                "max_colors must be in the range [2, 16], got {}",
                self.max_colors
            )));
        }
        if self.convergence_threshold <= 0.0 {
            return Err(AnalyzerError::InvalidConfig(format!(
                "convergence_threshold must be > 0, got {}",
                self.convergence_threshold
            )));
        }
        if self.max_iterations == 0 {
            return Err(AnalyzerError::InvalidConfig(
                "max_iterations must be ≥ 1".to_string(),
            ));
        }
        Ok(())
    }
}
