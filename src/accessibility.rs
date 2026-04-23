use crate::color::relative_luminance;
use crate::types::RgbColor;
use serde::Serialize;

const AA_NORMAL: f32 = 4.5;

const AAA_NORMAL: f32 = 7.0;

#[derive(Debug, Clone, Serialize)]
pub struct AccessibilityReport {
    pub contrast_ratio: f32,

    pub is_aa_normal: bool,

    pub is_aaa_normal: bool,

    pub recommended_font_color: String,
}

pub fn evaluate(color_a: RgbColor, color_b: RgbColor) -> AccessibilityReport {
    let la = relative_luminance(color_a);
    let lb = relative_luminance(color_b);

    let (l1, l2) = if la >= lb { (la, lb) } else { (lb, la) };
    let contrast_ratio = (l1 + 0.05) / (l2 + 0.05);

    let recommended_font_color = best_font_color(color_a);

    AccessibilityReport {
        contrast_ratio: (contrast_ratio * 100.0).round() / 100.0,
        is_aa_normal: contrast_ratio >= AA_NORMAL,
        is_aaa_normal: contrast_ratio >= AAA_NORMAL,
        recommended_font_color,
    }
}

pub fn best_font_color(background: RgbColor) -> String {
    let black = RgbColor { r: 0, g: 0, b: 0 };
    let white = RgbColor {
        r: 255,
        g: 255,
        b: 255,
    };

    let bg_lum = relative_luminance(background);

    let cr_black = contrast(bg_lum, relative_luminance(black));
    let cr_white = contrast(bg_lum, relative_luminance(white));

    if cr_black >= cr_white {
        "#000000".to_string()
    } else {
        "#FFFFFF".to_string()
    }
}

#[inline]
fn contrast(la: f32, lb: f32) -> f32 {
    let (l1, l2) = if la >= lb { (la, lb) } else { (lb, la) };
    (l1 + 0.05) / (l2 + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white_is_21_to_1() {
        let report = evaluate(
            RgbColor {
                r: 255,
                g: 255,
                b: 255,
            },
            RgbColor { r: 0, g: 0, b: 0 },
        );
        assert!(
            (report.contrast_ratio - 21.0).abs() < 0.1,
            "Black on white should be ~21:1, got {}",
            report.contrast_ratio
        );
        assert!(report.is_aa_normal);
        assert!(report.is_aaa_normal);
    }

    #[test]
    fn white_background_recommends_black_font() {
        let font = best_font_color(RgbColor {
            r: 255,
            g: 255,
            b: 255,
        });
        assert_eq!(font, "#000000");
    }

    #[test]
    fn black_background_recommends_white_font() {
        let font = best_font_color(RgbColor { r: 0, g: 0, b: 0 });
        assert_eq!(font, "#FFFFFF");
    }
}
