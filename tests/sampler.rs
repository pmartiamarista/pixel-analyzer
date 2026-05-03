use pixel_analyzer::config::Quality;
use pixel_analyzer::sampler;

#[test]
fn sampler_skips_transparent_pixels() {
    let pixels = vec![255, 0, 0, 0];
    let sampled = sampler::sample_pixels(&pixels, 1, 1, Quality::Precise);
    assert!(sampled.is_empty());
}

#[test]
fn sampler_all_pixels_returned_on_small_image() {
    let pixels = vec![255, 0, 0, 255];
    let sampled = sampler::sample_pixels(&pixels, 1, 1, Quality::Precise);
    assert_eq!(sampled.len(), 1);
}

#[test]
fn sample_returns_red_pixels() {
    let pixels: Vec<u8> = (0..16).flat_map(|_| vec![255, 0, 0, 255]).collect();
    let result = sampler::sample_pixels(&pixels, 4, 4, Quality::Precise);
    assert!(!result.is_empty());
    for px in &result {
        assert_eq!(px.r, 255);
        assert_eq!(px.g, 0);
        assert_eq!(px.b, 0);
    }
}

#[test]
fn downsample_reduces_pixel_count_for_oversized_image() {
    let w = 200;
    let h = 200;
    let pixels: Vec<u8> = vec![255; w * h * 4];
    let result = sampler::sample_pixels(&pixels, w as u32, h as u32, Quality::Draft);
    assert!(result.len() < w * h);
}

#[test]
fn sampler_returns_empty_for_fully_transparent_image() {
    let pixels: Vec<u8> = vec![0; 4 * 4 * 4];
    let result = sampler::sample_pixels(&pixels, 4, 4, Quality::Precise);
    assert!(result.is_empty());
}

#[test]
fn sampler_precise_returns_all_pixels_from_small_image() {
    let w = 10;
    let h = 10;
    let pixels: Vec<u8> = vec![255; w * h * 4];
    let result = sampler::sample_pixels(&pixels, w as u32, h as u32, Quality::Precise);
    assert_eq!(result.len(), w * h);
}
