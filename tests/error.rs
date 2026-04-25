use pixel_analyzer::error::AnalyzerError;

#[test]
fn error_empty_buffer_message() {
    let err = AnalyzerError::EmptyBuffer;
    let msg = err.to_string();
    assert!(msg.contains("[EmptyBuffer]"));
    assert!(msg.contains("is empty"));
}

#[test]
fn error_unsupported_format_message() {
    let err = AnalyzerError::UnsupportedFormat("Magic mismatch".to_string());
    let msg = err.to_string();
    assert!(msg.contains("[UnsupportedFormat]"));
    assert!(msg.contains("Magic mismatch"));
    assert!(msg.contains("Supported formats"));
}

#[test]
fn error_decoding_failed_message() {
    let err = AnalyzerError::DecodingFailed("CRC error".to_string());
    let msg = err.to_string();
    assert!(msg.contains("[DecodingFailed]"));
    assert!(msg.contains("CRC error"));
}

#[test]
fn error_invalid_config_message() {
    let err = AnalyzerError::InvalidConfig("max_colors=1".to_string());
    let msg = err.to_string();
    assert!(msg.contains("[InvalidConfig]"));
    assert!(msg.contains("max_colors=1"));
}

#[test]
fn error_insufficient_pixels_message() {
    let err = AnalyzerError::InsufficientPixels;
    let msg = err.to_string();
    assert!(msg.contains("[InsufficientPixels]"));
    assert!(msg.contains("Not enough pixels"));
}
