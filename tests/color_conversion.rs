use pixel_analyzer::accessibility;
use pixel_analyzer::color;
use pixel_analyzer::color_theory;
use pixel_analyzer::config::Quality;
use pixel_analyzer::kmeans;
use pixel_analyzer::metrics;
use pixel_analyzer::sampler;
use pixel_analyzer::types::{LabColor, LchColor, RgbColor};

#[test]
fn rgb_to_lab_black_is_zero_luminance() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let lab = color::rgb_to_lab(black);
    assert!(lab.l < 0.1);
}

#[test]
fn rgb_to_lab_white_is_100_luminance() {
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let lab = color::rgb_to_lab(white);
    assert!((lab.l - 100.0).abs() < 0.1);
}

#[test]
fn round_trip_lab_rgb_within_tolerance() {
    let original = RgbColor {
        r: 100,
        g: 150,
        b: 200,
    };
    let lab = color::rgb_to_lab(original);
    let recovered = color::lab_to_rgb(lab);
    assert!((original.r as i16 - recovered.r as i16).abs() <= 1);
    assert!((original.g as i16 - recovered.g as i16).abs() <= 1);
    assert!((original.b as i16 - recovered.b as i16).abs() <= 1);
}

#[test]
fn lch_hue_rotation_wraps_at_360() {
    let base = color::rgb_to_lab(RgbColor { r: 255, g: 0, b: 0 });
    let lch = color::lab_to_lch(base);
    let hex = color::lch_to_hex(LchColor {
        l: lch.l,
        c: lch.c,
        h: 370.0,
    });
    assert!(hex.starts_with('#'));
}

#[test]
fn complementary_produces_valid_hex() {
    let base = color::lab_to_lch(color::rgb_to_lab(RgbColor { r: 255, g: 0, b: 0 }));
    let theory = color_theory::generate(base);
    assert_eq!(theory.complementary.len(), 7);
    assert!(theory.complementary.starts_with('#'));
}

#[test]
fn triadic_colours_are_distinct() {
    let base = color::lab_to_lch(color::rgb_to_lab(RgbColor { r: 0, g: 255, b: 0 }));
    let theory = color_theory::generate(base);
    assert_ne!(theory.triadic[0], theory.triadic[1]);
}

#[test]
fn wcag_black_on_white_is_21_to_1() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, black);
    assert!((report.contrast_ratio - 21.0).abs() < 0.1);
}

#[test]
fn wcag_similar_colours_fails_aa() {
    let c1 = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let c2 = RgbColor {
        r: 250,
        g: 250,
        b: 250,
    };
    let report = accessibility::evaluate(c1, c2);
    assert!(!report.is_aa_normal);
}

#[test]
fn kmeans_clusters_two_separated_groups() {
    let mut pixels = Vec::new();
    for _ in 0..100 {
        pixels.push(LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        });
    }
    for _ in 0..100 {
        pixels.push(LabColor {
            l: 90.0,
            a: 0.0,
            b: 0.0,
        });
    }
    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 0.1, 100).unwrap();
    assert_eq!(clusters.len(), 2);
}

#[test]
fn kmeans_returns_error_on_insufficient_pixels() {
    let pixels = vec![LabColor {
        l: 50.0,
        a: 0.0,
        b: 0.0,
    }];
    let result = kmeans::kmeans_plus_plus(&pixels, 5, 1.0, 100);
    assert!(result.is_err());
}

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
fn entropy_zero_for_uniform_image() {
    let pixels = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        };
        100
    ];
    let stats = metrics::compute(
        &vec![
            RgbColor {
                r: 128,
                g: 128,
                b: 128,
            };
            100
        ],
        &pixels,
        0.0,
        10,
        10,
    );
    assert!(stats.entropy < 0.01);
}

#[test]
fn colorfulness_zero_for_grey_image() {
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128,
        };
        100
    ];
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        };
        100
    ];
    let stats = metrics::compute(&rgb, &lab, 0.0, 10, 10);
    assert!(stats.colorfulness < 0.01);
}
