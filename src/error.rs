use std::fmt;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum AnalyzerError {
    EmptyBuffer,
    UnsupportedFormat(String),
    DecodingFailed(String),
    InvalidConfig(String),
    InsufficientPixels,
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalyzerError::EmptyBuffer => {
                write!(
                    f,
                    "AnalyzerError [EmptyBuffer]: The provided buffer is empty."
                )
            }
            AnalyzerError::UnsupportedFormat(detail) => {
                write!(
                    f,
                    "AnalyzerError [UnsupportedFormat]: {}. Supported formats: PNG, JPEG, WebP.",
                    detail
                )
            }
            AnalyzerError::DecodingFailed(detail) => {
                write!(
                    f,
                    "AnalyzerError [DecodingFailed]: The image could not be decoded — {}.",
                    detail
                )
            }
            AnalyzerError::InvalidConfig(detail) => {
                write!(f, "AnalyzerError [InvalidConfig]: {}.", detail)
            }
            AnalyzerError::InsufficientPixels => {
                write!(
                    f,
                    "AnalyzerError [InsufficientPixels]: Not enough pixels after sampling \
                     to form the requested number of clusters. \
                     Try reducing max_colors or using Quality::Precise."
                )
            }
        }
    }
}

impl From<AnalyzerError> for JsValue {
    fn from(e: AnalyzerError) -> JsValue {
        e.to_string().into()
    }
}
