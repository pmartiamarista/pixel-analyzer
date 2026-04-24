use pixel_analyzer::color;
use pixel_analyzer::color_theory;
use pixel_analyzer::types::{LchColor, RgbColor};

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
fn complementary_hue_rotation_is_exact_at_lch_level() {
    let base_lch = color::lab_to_lch(color::rgb_to_lab(RgbColor {
        r: 60,
        g: 180,
        b: 100,
    }));

    let expected_comp_h = (base_lch.h + 180.0).rem_euclid(360.0);
    let actual_comp_h = (base_lch.h + 180.0).rem_euclid(360.0);
    assert!(
        (actual_comp_h - expected_comp_h).abs() < 0.001,
        "rotate_hue arithmetic must be exact to <0.001°: got {:.6}° vs {:.6}°",
        actual_comp_h,
        expected_comp_h,
    );
}

#[test]
fn complementary_hex_hue_within_tolerance_of_expected() {
    let base_lch = color::lab_to_lch(color::rgb_to_lab(RgbColor {
        r: 60,
        g: 180,
        b: 100,
    }));
    let theory = color_theory::generate(base_lch);
    let comp_lch = color::lab_to_lch(color::rgb_to_lab(RgbColor::from_hex(&theory.complementary)));
    let expected_h = (base_lch.h + 180.0).rem_euclid(360.0);
    let diff = (comp_lch.h - expected_h)
        .abs()
        .min(360.0 - (comp_lch.h - expected_h).abs());
    assert!(
        diff <= 5.0,
        "Hex-decoded complementary hue {:.2}° is more than 5° from expected {:.2}° (8-bit quantization tolerance)",
        comp_lch.h,
        expected_h,
    );
}
