use pixel_analyzer::accessibility;
use pixel_analyzer::types::RgbColor;

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
fn best_font_color_returns_black_for_mid_grey_background() {
    let mid_grey = RgbColor {
        r: 150,
        g: 150,
        b: 150,
    };
    let best = accessibility::best_font_color(mid_grey);
    assert_eq!(best, "#000000");
}

#[test]
fn best_font_color_returns_white_for_black_background() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let best = accessibility::best_font_color(black);
    assert_eq!(best, "#FFFFFF");
}

#[test]
fn wcag_high_contrast_pair_passes_aaa() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, black);
    assert!(report.is_aaa_normal);
}

#[test]
fn evaluate_contrast_ratio_is_symmetric() {
    let c1 = RgbColor {
        r: 10,
        g: 20,
        b: 30,
    };
    let c2 = RgbColor {
        r: 200,
        g: 210,
        b: 220,
    };
    let report1 = accessibility::evaluate(c1, c2);
    let report2 = accessibility::evaluate(c2, c1);
    assert!((report1.contrast_ratio - report2.contrast_ratio).abs() < 0.001);
}
