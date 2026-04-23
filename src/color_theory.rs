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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complementary_is_180_degrees_away() {
        let base = LchColor {
            l: 50.0,
            c: 50.0,
            h: 0.0,
        };
        let theory = generate(base);
        assert!(theory.complementary.starts_with('#'));
        assert_eq!(theory.complementary.len(), 7);
    }

    #[test]
    fn hue_wraps_correctly_past_360() {
        let base = LchColor {
            l: 50.0,
            c: 50.0,
            h: 350.0,
        };
        let theory = generate(base);
        assert!(theory.analogous[1].starts_with('#'));
    }

    #[test]
    fn triadic_produces_two_colours() {
        let base = LchColor {
            l: 60.0,
            c: 40.0,
            h: 120.0,
        };
        let theory = generate(base);
        assert_ne!(theory.triadic[0], theory.triadic[1]);
    }
}
