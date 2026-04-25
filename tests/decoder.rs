use pixel_analyzer::decoder;

#[test]
fn decode_rejects_buffer_shorter_than_4_bytes() {
    let bytes = vec![0x89, 0x50, 0x4E];
    let result = decoder::decode(&bytes);
    assert!(result.is_err());
}

#[test]
fn decode_rejects_unknown_magic_bytes() {
    let bytes = vec![0x00, 0x00, 0x00, 0x00];
    let result = decoder::decode(&bytes);
    assert!(result.is_err());
}

#[test]
fn decode_rejects_webp_shorter_than_12_bytes() {
    let bytes = b"RIFFxxxxWE".to_vec();
    let result = decoder::decode(&bytes);
    assert!(result.is_err());
}

// Full format tests would require valid header data which is complex to mock without dependencies.
// However, we can test that the dispatcher identifies the headers before they fail in actual decoding.

#[test]
fn decode_identifies_png_header() {
    let bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x00, 0x00];
    let result = decoder::decode(&bytes);
    // It should try to decode as PNG and fail with a DecodingFailed error, not UnsupportedFormat.
    match result {
        Err(pixel_analyzer::error::AnalyzerError::DecodingFailed(_)) => {}
        _ => panic!(
            "Expected DecodingFailed for malformed PNG, got {:?}",
            result
        ),
    }
}
