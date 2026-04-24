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
