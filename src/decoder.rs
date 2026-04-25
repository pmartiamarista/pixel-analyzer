use std::io::Cursor;

use crate::error::AnalyzerError;

pub fn decode(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32, Option<String>), AnalyzerError> {
    if bytes.len() < 4 {
        return Err(AnalyzerError::UnsupportedFormat(
            "Buffer too short to detect format".to_string(),
        ));
    }
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return decode_png(bytes);
    }
    if bytes.starts_with(&[0xFF, 0xD8]) {
        return decode_jpeg(bytes);
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return decode_webp(bytes);
    }
    Err(AnalyzerError::UnsupportedFormat(format!(
        "Unknown magic bytes: {:02X} {:02X} {:02X} {:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    )))
}

fn decode_png(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32, Option<String>), AnalyzerError> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::STRIP_16 | png::Transformations::EXPAND);
    let mut reader = decoder
        .read_info()
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;
    let is_grey = matches!(
        reader.info().color_type,
        png::ColorType::Grayscale | png::ColorType::GrayscaleAlpha
    );
    let buf_size = reader.output_buffer_size().ok_or_else(|| {
        AnalyzerError::DecodingFailed("PNG output buffer size unavailable".to_string())
    })?;
    let mut buf = vec![0u8; buf_size];
    let frame = reader
        .next_frame(&mut buf)
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;
    let raw = &buf[..frame.buffer_size()];
    let rgba = expand_to_rgba(raw, frame.color_type)?;
    let warning =
        is_grey.then(|| "Image is greyscale; colour palette will be achromatic.".to_string());
    Ok((rgba, frame.width, frame.height, warning))
}

fn expand_to_rgba(raw: &[u8], color_type: png::ColorType) -> Result<Vec<u8>, AnalyzerError> {
    match color_type {
        png::ColorType::Rgba => Ok(raw.to_vec()),
        png::ColorType::Rgb => {
            let mut out = Vec::with_capacity(raw.len() / 3 * 4);
            for chunk in raw.chunks_exact(3) {
                out.extend_from_slice(chunk);
                out.push(255);
            }
            Ok(out)
        }
        png::ColorType::Grayscale => {
            let mut out = Vec::with_capacity(raw.len() * 4);
            for &g in raw {
                out.extend_from_slice(&[g, g, g, 255]);
            }
            Ok(out)
        }
        png::ColorType::GrayscaleAlpha => {
            let mut out = Vec::with_capacity(raw.len() * 2);
            for chunk in raw.chunks_exact(2) {
                out.extend_from_slice(&[chunk[0], chunk[0], chunk[0], chunk[1]]);
            }
            Ok(out)
        }
        png::ColorType::Indexed => Err(AnalyzerError::DecodingFailed(
            "Indexed PNG expansion failed unexpectedly".to_string(),
        )),
    }
}

fn decode_jpeg(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32, Option<String>), AnalyzerError> {
    let mut decoder = zune_jpeg::JpegDecoder::new(Cursor::new(bytes));
    let pixels = decoder
        .decode()
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;
    let info = decoder
        .info()
        .ok_or_else(|| AnalyzerError::DecodingFailed("JPEG missing image info".to_string()))?;
    let pixel_count = info.width as usize * info.height as usize;
    let bytes_per_pixel = pixels
        .len()
        .checked_div(pixel_count)
        .ok_or_else(|| AnalyzerError::DecodingFailed("JPEG has zero dimensions".to_string()))?;
    let is_grey = bytes_per_pixel == 1;
    let rgba = match bytes_per_pixel {
        1 => {
            let mut out = Vec::with_capacity(pixel_count * 4);
            for &g in &pixels {
                out.extend_from_slice(&[g, g, g, 255]);
            }
            out
        }
        3 => {
            let mut out = Vec::with_capacity(pixel_count * 4);
            for chunk in pixels.chunks_exact(3) {
                out.extend_from_slice(chunk);
                out.push(255);
            }
            out
        }
        4 => pixels,
        _ => {
            return Err(AnalyzerError::DecodingFailed(format!(
                "Unexpected JPEG channel count: {}",
                bytes_per_pixel
            )));
        }
    };
    let warning =
        is_grey.then(|| "Image is greyscale; colour palette will be achromatic.".to_string());
    Ok((rgba, info.width as u32, info.height as u32, warning))
}

fn decode_webp(bytes: &[u8]) -> Result<(Vec<u8>, u32, u32, Option<String>), AnalyzerError> {
    let cursor = Cursor::new(bytes);
    let mut decoder = image_webp::WebPDecoder::new(cursor)
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;
    let (width, height) = decoder.dimensions();
    let has_alpha = decoder.has_alpha();
    let bytes_per_pixel: usize = if has_alpha { 4 } else { 3 };
    let mut buf = vec![0u8; width as usize * height as usize * bytes_per_pixel];
    decoder
        .read_image(&mut buf)
        .map_err(|e| AnalyzerError::DecodingFailed(e.to_string()))?;
    let rgba = if has_alpha {
        buf
    } else {
        let mut out = Vec::with_capacity(width as usize * height as usize * 4);
        for chunk in buf.chunks_exact(3) {
            out.extend_from_slice(chunk);
            out.push(255);
        }
        out
    };
    Ok((rgba, width, height, None))
}
