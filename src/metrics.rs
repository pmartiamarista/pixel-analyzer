use crate::types::{HueGroup, LabColor, Orientation, RgbColor};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ImageStats {
    pub brightness: f32,
    pub colorfulness: f32,
    pub entropy: f32,
    pub dominant_hue_group: HueGroup,
    pub orientation: Orientation,
}

pub fn compute(
    rgb_pixels: &[RgbColor],
    lab_pixels: &[LabColor],
    dominant_hue: f32,
    img_width: u32,
    img_height: u32,
) -> ImageStats {
    let brightness = mean_brightness(lab_pixels);
    let colorfulness = hasler_suesstrunk(rgb_pixels);
    let entropy = shannon_entropy(lab_pixels);
    let dominant_hue_group = classify_hue_group(dominant_hue);
    let orientation = classify_orientation(img_width, img_height);

    ImageStats {
        brightness: (brightness * 100.0).round() / 100.0,
        colorfulness: (colorfulness * 100.0).round() / 100.0,
        entropy: (entropy * 1000.0).round() / 1000.0,
        dominant_hue_group,
        orientation,
    }
}

fn mean_brightness(lab: &[LabColor]) -> f32 {
    if lab.is_empty() {
        return 0.0;
    }
    let sum: f32 = lab.iter().map(|c| c.l).sum();
    sum / lab.len() as f32
}

fn hasler_suesstrunk(pixels: &[RgbColor]) -> f32 {
    if pixels.is_empty() {
        return 0.0;
    }

    let n = pixels.len() as f32;

    let rg: Vec<f32> = pixels.iter().map(|px| px.r as f32 - px.g as f32).collect();
    let yb: Vec<f32> = pixels
        .iter()
        .map(|px| 0.5 * (px.r as f32 + px.g as f32) - px.b as f32)
        .collect();

    let mean_rg = rg.iter().sum::<f32>() / n;
    let mean_yb = yb.iter().sum::<f32>() / n;

    let var_rg = rg.iter().map(|v| (v - mean_rg).powi(2)).sum::<f32>() / n;
    let var_yb = yb.iter().map(|v| (v - mean_yb).powi(2)).sum::<f32>() / n;

    let m = (var_rg + var_yb).sqrt() + 0.3 * (mean_rg.powi(2) + mean_yb.powi(2)).sqrt();

    (m / 1.09).clamp(0.0, 100.0)
}

fn shannon_entropy(lab: &[LabColor]) -> f32 {
    if lab.is_empty() {
        return 0.0;
    }

    let mut histogram = [0u32; 256];
    for px in lab {
        let bin = (px.l.clamp(0.0, 99.99) as usize).min(255);
        histogram[bin] += 1;
    }

    let total = lab.len() as f32;
    histogram
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f32 / total;
            -p * p.log2()
        })
        .sum()
}

fn classify_hue_group(h: f32) -> HueGroup {
    let h = h.rem_euclid(360.0);
    if !(70.0..330.0).contains(&h) {
        HueGroup::Warm
    } else if (150.0..330.0).contains(&h) {
        HueGroup::Cool
    } else {
        HueGroup::Neutral
    }
}

fn classify_orientation(w: u32, h: u32) -> Orientation {
    match w.cmp(&h) {
        std::cmp::Ordering::Greater => Orientation::Landscape,
        std::cmp::Ordering::Less => Orientation::Portrait,
        std::cmp::Ordering::Equal => Orientation::Square,
    }
}
