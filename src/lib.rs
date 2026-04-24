use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

pub mod accessibility;
pub mod color;
pub mod color_theory;
pub mod config;
pub mod decoder;
pub mod error;
pub mod kmeans;
pub mod metrics;
pub mod report;
pub mod sampler;
pub mod types;

use config::{AnalysisConfig, Quality};
use error::AnalyzerError;
use report::AnalysisReport;

#[wasm_bindgen(start)]
pub fn on_module_load() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init() -> Promise {
    future_to_promise(async { Ok(JsValue::UNDEFINED) })
}

#[wasm_bindgen]
pub fn terminate() {}

#[wasm_bindgen]
pub struct AnalysisOptions {
    max_colors: u8,
    quality: Quality,
    convergence: f32,
}

#[wasm_bindgen]
impl AnalysisOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(max_colors: u8, quality: Quality, convergence: f32) -> Self {
        Self {
            max_colors,
            quality,
            convergence,
        }
    }

    #[wasm_bindgen]
    pub fn defaults() -> Self {
        let d = AnalysisConfig::default();
        Self {
            max_colors: d.max_colors,
            quality: d.quality,
            convergence: d.convergence_threshold,
        }
    }
}

impl From<AnalysisOptions> for AnalysisConfig {
    fn from(o: AnalysisOptions) -> Self {
        AnalysisConfig {
            max_colors: o.max_colors,
            quality: o.quality,
            convergence_threshold: o.convergence,
            max_iterations: 100,
        }
    }
}

#[wasm_bindgen]
pub fn analyze(data: js_sys::Uint8Array, options: Option<AnalysisOptions>) -> Promise {
    let bytes: Vec<u8> = data.to_vec();

    let config: AnalysisConfig = options.map(AnalysisConfig::from).unwrap_or_default();

    future_to_promise(async move { run_pipeline(bytes, config).map_err(JsValue::from) })
}

fn run_pipeline(bytes: Vec<u8>, config: AnalysisConfig) -> Result<JsValue, AnalyzerError> {
    config.validate()?;
    validate_buffer(&bytes)?;

    let start_ms = now_ms();

    let (rgba_bytes, img_width, img_height, warning) = decoder::decode(&bytes)?;

    let rgb_pixels = sampler::sample_pixels(&rgba_bytes, img_width, img_height, config.quality);

    if rgb_pixels.is_empty() {
        return Err(AnalyzerError::InsufficientPixels);
    }

    let lab_pixels: Vec<types::LabColor> =
        rgb_pixels.iter().map(|px| color::rgb_to_lab(*px)).collect();

    let clusters = kmeans::kmeans_plus_plus(
        &lab_pixels,
        config.max_colors as usize,
        config.convergence_threshold,
        config.max_iterations,
    )?;

    let elapsed_ms = now_ms() - start_ms;

    let report = report::build(
        &clusters,
        &rgb_pixels,
        &lab_pixels,
        img_width,
        img_height,
        elapsed_ms,
        warning,
    );

    serialise(&report)
}

fn validate_buffer(bytes: &[u8]) -> Result<(), AnalyzerError> {
    if bytes.is_empty() {
        return Err(AnalyzerError::EmptyBuffer);
    }
    Ok(())
}

fn serialise(report: &AnalysisReport) -> Result<JsValue, AnalyzerError> {
    serde_wasm_bindgen::to_value(report)
        .map_err(|e| AnalyzerError::DecodingFailed(format!("Serialization error: {}", e)))
}

fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}
