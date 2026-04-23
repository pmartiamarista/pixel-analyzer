use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

pub mod accessibility;
pub mod color;
pub mod color_theory;
pub mod config;
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

    let (img, warning) = decode(&bytes)?;
    let img_width = img.width();
    let img_height = img.height();

    let rgba_bytes = to_rgba(&img);

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

fn decode(bytes: &[u8]) -> Result<(image::DynamicImage, Option<String>), AnalyzerError> {
    let format = detect_format(bytes)?;
    let mut warning = None;

    let img = image::load_from_memory_with_format(bytes, format)
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;

    if img.color() == image::ColorType::L8 || img.color() == image::ColorType::La8 {
        warning = Some("Image is greyscale; colour palette will be achromatic.".to_string());
    }

    Ok((img, warning))
}

fn detect_format(bytes: &[u8]) -> Result<image::ImageFormat, AnalyzerError> {
    if bytes.len() < 4 {
        return Err(AnalyzerError::UnsupportedFormat(
            "Buffer too short to detect format".to_string(),
        ));
    }

    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return Ok(image::ImageFormat::Png);
    }
    if bytes.starts_with(&[0xFF, 0xD8]) {
        return Ok(image::ImageFormat::Jpeg);
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Ok(image::ImageFormat::WebP);
    }

    Err(AnalyzerError::UnsupportedFormat(format!(
        "Unknown magic bytes: {:02X} {:02X} {:02X} {:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    )))
}

fn to_rgba(img: &image::DynamicImage) -> Vec<u8> {
    img.to_rgba8().into_raw()
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
