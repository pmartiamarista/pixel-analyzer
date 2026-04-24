use crate::color::lch_to_hex;
use crate::types::LchColor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ColorTheory {
    pub complementary: String,

    pub triadic: [String; 2],

    pub analogous: [String; 2],
}

pub fn generate(base: LchColor) -> ColorTheory {
    ColorTheory {
        complementary: rotate_hue(base, 180.0),
        triadic: [rotate_hue(base, 120.0), rotate_hue(base, 240.0)],
        analogous: [rotate_hue(base, -30.0), rotate_hue(base, 30.0)],
    }
}

#[inline]
fn rotate_hue(base: LchColor, degrees: f32) -> String {
    let new_h = (base.h + degrees).rem_euclid(360.0);
    let rotated = LchColor {
        l: base.l,
        c: base.c,
        h: new_h,
    };
    lch_to_hex(rotated)
}
