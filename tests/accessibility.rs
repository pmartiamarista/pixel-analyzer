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
