use crate::types::{LabColor, LchColor, RgbColor};

const D65_XN: f32 = 0.95047;
const D65_YN: f32 = 1.00000;
const D65_ZN: f32 = 1.08883;

const EPSILON: f32 = 0.008856;
const KAPPA: f32 = 903.3;
const ONE_THIRD: f32 = 1.0 / 3.0;

#[inline]
fn linearise_channel(c: u8) -> f32 {
    let v = c as f32 / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055_f32).powf(2.4)
    }
}

pub fn rgb_to_xyz(rgb: RgbColor) -> (f32, f32, f32) {
    let r = linearise_channel(rgb.r);
    let g = linearise_channel(rgb.g);
    let b = linearise_channel(rgb.b);

    let x = 0.4124 * r + 0.3576 * g + 0.1805 * b;
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let z = 0.0193 * r + 0.1192 * g + 0.9505 * b;

    (x, y, z)
}

#[inline]
fn f_lab(t: f32) -> f32 {
    if t > EPSILON {
        t.powf(ONE_THIRD)
    } else {
        (KAPPA * t + 16.0) / 116.0
    }
}

pub fn xyz_to_lab(x: f32, y: f32, z: f32) -> LabColor {
    let fx = f_lab(x / D65_XN);
    let fy = f_lab(y / D65_YN);
    let fz = f_lab(z / D65_ZN);

    LabColor {
        l: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

pub fn rgb_to_lab(rgb: RgbColor) -> LabColor {
    let (x, y, z) = rgb_to_xyz(rgb);
    xyz_to_lab(x, y, z)
}

pub fn lab_to_lch(lab: LabColor) -> LchColor {
    let c = (lab.a * lab.a + lab.b * lab.b).sqrt();
    let h_rad = lab.b.atan2(lab.a);
    let h = (h_rad.to_degrees() + 360.0) % 360.0;
    LchColor { l: lab.l, c, h }
}

pub fn lch_to_lab(lch: LchColor) -> LabColor {
    let h_rad = lch.h.to_radians();
    LabColor {
        l: lch.l,
        a: lch.c * h_rad.cos(),
        b: lch.c * h_rad.sin(),
    }
}

fn lab_to_xyz(lab: LabColor) -> (f32, f32, f32) {
    let fy = (lab.l + 16.0) / 116.0;
    let fx = lab.a / 500.0 + fy;
    let fz = fy - lab.b / 200.0;

    let x = if fx.powi(3) > EPSILON {
        fx.powi(3)
    } else {
        (116.0 * fx - 16.0) / KAPPA
    };
    let y = if lab.l > KAPPA * EPSILON {
        ((lab.l + 16.0) / 116.0).powi(3)
    } else {
        lab.l / KAPPA
    };
    let z = if fz.powi(3) > EPSILON {
        fz.powi(3)
    } else {
        (116.0 * fz - 16.0) / KAPPA
    };

    (x * D65_XN, y * D65_YN, z * D65_ZN)
}

#[inline]
fn compand_channel(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

pub fn lab_to_rgb(lab: LabColor) -> RgbColor {
    let (x, y, z) = lab_to_xyz(lab);

    let r_lin = 3.2406 * x - 1.5372 * y - 0.4986 * z;
    let g_lin = -0.9689 * x + 1.8758 * y + 0.0415 * z;
    let b_lin = 0.0557 * x - 0.2040 * y + 1.0570 * z;

    let to_u8 = |v: f32| -> u8 { (compand_channel(v.clamp(0.0, 1.0)) * 255.0).round() as u8 };

    RgbColor {
        r: to_u8(r_lin),
        g: to_u8(g_lin),
        b: to_u8(b_lin),
    }
}

pub fn lch_to_hex(lch: LchColor) -> String {
    lab_to_rgb(lch_to_lab(lch)).to_hex()
}

pub fn relative_luminance(rgb: RgbColor) -> f32 {
    let r = linearise_channel(rgb.r);
    let g = linearise_channel(rgb.g);
    let b = linearise_channel(rgb.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
