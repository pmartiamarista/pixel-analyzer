use wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum AnalyzerError {
    EmptyBuffer,

    UnsupportedFormat(String),

    DecodingFailed(String),

    InvalidConfig(String),

    InsufficientPixels,
}

impl From<AnalyzerError> for JsValue {
    fn from(e: AnalyzerError) -> JsValue {
        let msg = match e {
            AnalyzerError::EmptyBuffer => {
                "AnalyzerError [EmptyBuffer]: The provided buffer is empty.".to_string()
            }
            AnalyzerError::UnsupportedFormat(detail) => {
                format!(
                    "AnalyzerError [UnsupportedFormat]: {}. Supported formats: PNG, JPEG, WebP.",
                    detail
                )
            }
            AnalyzerError::DecodingFailed(detail) => {
                format!(
                    "AnalyzerError [DecodingFailed]: The image could not be decoded — {}.",
                    detail
                )
            }
            AnalyzerError::InvalidConfig(detail) => {
                format!("AnalyzerError [InvalidConfig]: {}.", detail)
            }
            AnalyzerError::InsufficientPixels => {
                "AnalyzerError [InsufficientPixels]: Not enough pixels after sampling \
                 to form the requested number of clusters. \
                 Try reducing max_colors or using Quality::Precise."
                    .to_string()
            }
        };
        JsValue::from_str(&msg)
    }
}
