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

#[test]
fn apca_black_on_white_passes_body_text() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(black, white);
    assert!(report.passes_body_text, "Lc={}", report.lc);
    assert!(report.passes_large_text);
    assert!(report.passes_ui_component);
    assert!(report.lc > 100.0, "Expected Lc > 100, got {}", report.lc);
}

#[test]
fn apca_white_on_black_is_negative_lc_and_passes_body_text() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(white, black);
    assert!(
        report.lc < 0.0,
        "Expected negative Lc for light-on-dark, got {}",
        report.lc
    );
    assert!(report.passes_body_text, "Lc={}", report.lc);
}

#[test]
fn apca_identical_colors_returns_zero_lc() {
    let grey = RgbColor {
        r: 128,
        g: 128,
        b: 128,
    };
    let report = accessibility::apca_evaluate(grey, grey);
    assert_eq!(report.lc, 0.0);
    assert!(!report.passes_ui_component);
}

#[test]
fn apca_similar_colors_fails_all_thresholds() {
    let c1 = RgbColor {
        r: 200,
        g: 200,
        b: 200,
    };
    let c2 = RgbColor {
        r: 210,
        g: 210,
        b: 210,
    };
    let report = accessibility::apca_evaluate(c1, c2);
    assert!(!report.passes_ui_component, "Lc={}", report.lc);
}

#[test]
fn apca_mid_grey_text_on_white_bg_passes_ui_only() {
    let grey = RgbColor {
        r: 119,
        g: 119,
        b: 119,
    };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(grey, white);
    assert!(report.passes_ui_component, "Lc={}", report.lc);
    assert!(
        !report.passes_body_text,
        "Expected fail body text, Lc={}",
        report.lc
    );
}

#[test]
fn evaluate_report_includes_apca_field() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, black);
    assert!(report.apca.passes_body_text, "Lc={}", report.apca.lc);
}

#[test]
fn apca_lc_is_directional_normal_vs_reverse() {
    let dark = RgbColor {
        r: 30,
        g: 30,
        b: 30,
    };
    let light = RgbColor {
        r: 230,
        g: 230,
        b: 230,
    };
    let normal = accessibility::apca_evaluate(dark, light);
    let reverse = accessibility::apca_evaluate(light, dark);
    assert!(
        normal.lc > 0.0,
        "Normal polarity Lc should be positive: {}",
        normal.lc
    );
    assert!(
        reverse.lc < 0.0,
        "Reverse polarity Lc should be negative: {}",
        reverse.lc
    );
}

#[test]
fn wcag_black_on_white_passes_all_large_text_thresholds() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, black);
    assert!(report.is_aa_large, "ratio={}", report.contrast_ratio);
    assert!(report.is_aaa_large, "ratio={}", report.contrast_ratio);
}

#[test]
fn wcag_mid_contrast_passes_large_but_fails_normal() {
    let dark = RgbColor {
        r: 140,
        g: 140,
        b: 140,
    };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, dark);
    assert!(
        report.contrast_ratio >= 3.0 && report.contrast_ratio < 4.5,
        "Expected ratio in [3, 4.5), got {}",
        report.contrast_ratio
    );
    assert!(report.is_aa_large, "ratio={}", report.contrast_ratio);
    assert!(!report.is_aa_normal, "ratio={}", report.contrast_ratio);
}

#[test]
fn wcag_black_on_white_passes_ui_component_threshold() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::evaluate(white, black);
    assert!(report.is_aa_ui, "ratio={}", report.contrast_ratio);
}

#[test]
fn wcag_low_contrast_fails_ui_component_threshold() {
    let c1 = RgbColor {
        r: 200,
        g: 200,
        b: 200,
    };
    let c2 = RgbColor {
        r: 220,
        g: 220,
        b: 220,
    };
    let report = accessibility::evaluate(c1, c2);
    assert!(!report.is_aa_ui, "ratio={}", report.contrast_ratio);
}

#[test]
fn apca_black_on_white_is_normal_polarity() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(black, white);
    assert!(report.is_normal_polarity);
    assert!(report.lc > 0.0);
}

#[test]
fn apca_white_on_black_is_reverse_polarity() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(white, black);
    assert!(!report.is_normal_polarity);
    assert!(report.lc < 0.0);
}

#[test]
fn apca_identical_colors_polarity_is_false() {
    let grey = RgbColor {
        r: 128,
        g: 128,
        b: 128,
    };
    let report = accessibility::apca_evaluate(grey, grey);
    assert!(!report.is_normal_polarity);
    assert_eq!(report.lc, 0.0);
}

#[test]
fn apca_black_on_white_passes_preferred() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(black, white);
    assert!(report.passes_preferred);
    assert!(report.lc.abs() >= 90.0);
}

#[test]
fn apca_mid_contrast_fails_preferred() {
    let mid = RgbColor {
        r: 85,
        g: 85,
        b: 85,
    };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(mid, white);
    assert!(report.passes_body_text);
    assert!(!report.passes_preferred);
}

#[test]
fn apca_decorative_passes_ui_fails_for_mid_contrast() {
    let mid = RgbColor {
        r: 185,
        g: 185,
        b: 185,
    };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(mid, white);
    assert!(report.passes_decorative);
    assert!(!report.passes_ui_component);
}

#[test]
fn apca_low_contrast_fails_decorative() {
    let c1 = RgbColor {
        r: 200,
        g: 200,
        b: 200,
    };
    let c2 = RgbColor {
        r: 210,
        g: 210,
        b: 210,
    };
    let report = accessibility::apca_evaluate(c1, c2);
    assert!(!report.passes_decorative);
    assert!(!report.passes_visibility);
}

#[test]
fn apca_identical_colors_fails_visibility() {
    let grey = RgbColor {
        r: 128,
        g: 128,
        b: 128,
    };
    let report = accessibility::apca_evaluate(grey, grey);
    assert!(!report.passes_visibility);
}

#[test]
fn apca_high_contrast_passes_visibility() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(black, white);
    assert!(report.passes_visibility);
}

#[test]
fn apca_tier_ordering_is_monotone() {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };
    let report = accessibility::apca_evaluate(black, white);
    if report.passes_preferred {
        assert!(report.passes_body_text);
    }
    if report.passes_body_text {
        assert!(report.passes_large_text);
    }
    if report.passes_large_text {
        assert!(report.passes_ui_component);
    }
    if report.passes_ui_component {
        assert!(report.passes_decorative);
    }
    if report.passes_decorative {
        assert!(report.passes_visibility);
    }
}
