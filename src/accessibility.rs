use crate::color::relative_luminance;
use crate::types::RgbColor;
use serde::Serialize;

const AA_NORMAL: f32 = 4.5;
const AAA_NORMAL: f32 = 7.0;
const AA_LARGE: f32 = 3.0;
const AAA_LARGE: f32 = 4.5;
const AA_UI: f32 = 3.0;

const APCA_BG_LIGHT: f32 = 0.56;
const APCA_TXT_DARK: f32 = 0.57;
const APCA_BG_DARK: f32 = 0.65;
const APCA_TXT_LIGHT: f32 = 0.62;
const APCA_SCALE: f32 = 1.14;
const APCA_SOFT_THRESH: f32 = 0.022;
const APCA_SOFT_EXP: f32 = 1.414;
const APCA_NOISE_FLOOR: f32 = 0.1;
const APCA_LUM_R: f32 = 0.2126729;
const APCA_LUM_G: f32 = 0.7151522;
const APCA_LUM_B: f32 = 0.0721750;

pub const APCA_THRESHOLD_PREFERRED: f32 = 90.0;
pub const APCA_THRESHOLD_BODY: f32 = 75.0;
pub const APCA_THRESHOLD_LARGE: f32 = 60.0;
pub const APCA_THRESHOLD_UI: f32 = 45.0;
pub const APCA_THRESHOLD_DECORATIVE: f32 = 30.0;
pub const APCA_THRESHOLD_VISIBILITY: f32 = 15.0;

#[derive(Debug, Clone, Serialize)]
pub struct ApcaReport {
    pub lc: f32,
    pub is_normal_polarity: bool,
    pub passes_preferred: bool,
    pub passes_body_text: bool,
    pub passes_large_text: bool,
    pub passes_ui_component: bool,
    pub passes_decorative: bool,
    pub passes_visibility: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccessibilityReport {
    pub contrast_ratio: f32,
    pub is_aa_normal: bool,
    pub is_aaa_normal: bool,
    pub is_aa_large: bool,
    pub is_aaa_large: bool,
    pub is_aa_ui: bool,
    pub recommended_font_color: String,
    pub apca: ApcaReport,
}

pub fn evaluate(background: RgbColor, foreground: RgbColor) -> AccessibilityReport {
    let la = relative_luminance(background);
    let lb = relative_luminance(foreground);
    let (l1, l2) = if la >= lb { (la, lb) } else { (lb, la) };
    let contrast_ratio = (l1 + 0.05) / (l2 + 0.05);
    let recommended_font_color = best_font_color(background);
    let apca = apca_evaluate(foreground, background);

    AccessibilityReport {
        contrast_ratio: (contrast_ratio * 100.0).round() / 100.0,
        is_aa_normal: contrast_ratio >= AA_NORMAL,
        is_aaa_normal: contrast_ratio >= AAA_NORMAL,
        is_aa_large: contrast_ratio >= AA_LARGE,
        is_aaa_large: contrast_ratio >= AAA_LARGE,
        is_aa_ui: contrast_ratio >= AA_UI,
        recommended_font_color,
        apca,
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

pub fn apca_evaluate(text_rgb: RgbColor, bg_rgb: RgbColor) -> ApcaReport {
    let y_txt = apca_soft_clamp(apca_luminance(text_rgb));
    let y_bg = apca_soft_clamp(apca_luminance(bg_rgb));
    let lc = apca_lc(y_txt, y_bg);
    let lc_abs = lc.abs();

    ApcaReport {
        lc,
        is_normal_polarity: lc > 0.0,
        passes_preferred: lc_abs >= APCA_THRESHOLD_PREFERRED,
        passes_body_text: lc_abs >= APCA_THRESHOLD_BODY,
        passes_large_text: lc_abs >= APCA_THRESHOLD_LARGE,
        passes_ui_component: lc_abs >= APCA_THRESHOLD_UI,
        passes_decorative: lc_abs >= APCA_THRESHOLD_DECORATIVE,
        passes_visibility: lc_abs >= APCA_THRESHOLD_VISIBILITY,
    }
}

fn apca_luminance(rgb: RgbColor) -> f32 {
    let r = (rgb.r as f32 / 255.0).powf(2.4);
    let g = (rgb.g as f32 / 255.0).powf(2.4);
    let b = (rgb.b as f32 / 255.0).powf(2.4);
    APCA_LUM_R * r + APCA_LUM_G * g + APCA_LUM_B * b
}

fn apca_soft_clamp(y: f32) -> f32 {
    if y >= APCA_SOFT_THRESH {
        y
    } else {
        y + (APCA_SOFT_THRESH - y).powf(APCA_SOFT_EXP)
    }
}

fn apca_lc(y_txt: f32, y_bg: f32) -> f32 {
    let sapc = if y_bg >= y_txt {
        (y_bg.powf(APCA_BG_LIGHT) - y_txt.powf(APCA_TXT_DARK)) * APCA_SCALE
    } else {
        (y_bg.powf(APCA_BG_DARK) - y_txt.powf(APCA_TXT_LIGHT)) * APCA_SCALE
    };
    if sapc.abs() < APCA_NOISE_FLOOR {
        return 0.0;
    }
    let lc = sapc * 100.0;
    (lc * 100.0).round() / 100.0
}

#[inline]
fn contrast(la: f32, lb: f32) -> f32 {
    let (l1, l2) = if la >= lb { (la, lb) } else { (lb, la) };
    (l1 + 0.05) / (l2 + 0.05)
}
