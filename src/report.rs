use crate::accessibility::{self, AccessibilityReport};
use crate::color::{lab_to_lch, lab_to_rgb};
use crate::color_theory::{self, ColorTheory};
use crate::metrics::{self, ImageStats};
use crate::types::{Cluster, ColorEntry, LabColor, LabValues, LchValues, RgbColor};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnalysisReport {
    pub main: MainPalette,
    pub palettes: Palettes,
    pub accessibility: AccessibilityReport,
    pub image_stats: ImageStats,
    pub color_theory: ColorTheory,

    pub analysis_time_ms: f64,

    pub pixels_analyzed: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MainPalette {
    pub dominant: ColorEntry,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<ColorEntry>,

    pub background_suggestion: String,

    pub foreground_suggestion: String,
}

#[derive(Debug, Serialize)]
pub struct Palettes {
    pub vibrant: Vec<ColorEntry>,
    pub muted: Vec<ColorEntry>,
    pub light: Vec<ColorEntry>,
    pub dark: Vec<ColorEntry>,
    pub raw: Vec<ColorEntry>,
}

pub fn build(
    clusters: &[Cluster],
    rgb_pixels: &[RgbColor],
    lab_pixels: &[LabColor],
    img_width: u32,
    img_height: u32,
    analysis_time_ms: f64,
    warning: Option<String>,
) -> AnalysisReport {
    let entries: Vec<ColorEntry> = clusters.iter().map(cluster_to_entry).collect();

    let dominant = entries[0].clone();

    let accent = pick_accent(&entries, &dominant);

    let foreground_suggestion = accessibility::best_font_color(RgbColor::from_hex(&dominant.hex));

    let background_suggestion = suggest_background(clusters[0].centroid);

    let vibrant: Vec<_> = entries.iter().filter(|e| e.lch.c > 28.0).cloned().collect();
    let muted: Vec<_> = entries.iter().filter(|e| e.lch.c < 15.0).cloned().collect();
    let light: Vec<_> = entries.iter().filter(|e| e.lch.l > 80.0).cloned().collect();
    let dark: Vec<_> = entries.iter().filter(|e| e.lch.l < 20.0).cloned().collect();

    let accessibility = accent
        .as_ref()
        .map(|ac| {
            accessibility::evaluate(
                RgbColor::from_hex(&dominant.hex),
                RgbColor::from_hex(&ac.hex),
            )
        })
        .unwrap_or_else(|| {
            accessibility::evaluate(
                RgbColor::from_hex(&dominant.hex),
                RgbColor::from_hex(&dominant.hex),
            )
        });

    let no_accent_warning = accent.is_none().then_some(
        "No perceptually distinct accent colour found (ΔE < 5 from dominant).".to_string(),
    );

    let merged_warning = match (warning, no_accent_warning) {
        (Some(w), Some(a)) => Some(format!("{} {}", w, a)),
        (w, a) => w.or(a),
    };

    let image_stats = metrics::compute(
        rgb_pixels,
        lab_pixels,
        dominant.lch.h,
        img_width,
        img_height,
    );

    let base_lch = crate::types::LchColor {
        l: dominant.lch.l,
        c: dominant.lch.c,
        h: dominant.lch.h,
    };
    let color_theory = color_theory::generate(base_lch);

    AnalysisReport {
        main: MainPalette {
            dominant,
            accent,
            background_suggestion,
            foreground_suggestion,
        },
        palettes: Palettes {
            vibrant,
            muted,
            light,
            dark,
            raw: entries,
        },
        accessibility,
        image_stats,
        color_theory,
        analysis_time_ms,
        pixels_analyzed: lab_pixels.len(),
        warning: merged_warning,
    }
}

fn cluster_to_entry(cluster: &Cluster) -> ColorEntry {
    let lab = cluster.centroid;
    let lch = lab_to_lch(lab);
    let rgb = lab_to_rgb(lab);

    ColorEntry {
        hex: rgb.to_hex(),
        population: (cluster.population() * 10000.0).round() / 10000.0,
        is_dark: lab.is_dark(),
        lab: LabValues {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        },
        lch: LchValues {
            l: lch.l,
            c: lch.c,
            h: lch.h,
        },
    }
}

fn pick_accent(entries: &[ColorEntry], dominant: &ColorEntry) -> Option<ColorEntry> {
    let dom_lab = LabColor {
        l: dominant.lab.l,
        a: dominant.lab.a,
        b: dominant.lab.b,
    };

    entries
        .iter()
        .skip(1)
        .filter(|e| {
            let lab = LabColor {
                l: e.lab.l,
                a: e.lab.a,
                b: e.lab.b,
            };
            lab.delta_e(dom_lab) > 5.0
        })
        .max_by(|a, b| {
            let score_a = accent_score(a, dom_lab);
            let score_b = accent_score(b, dom_lab);
            score_a.partial_cmp(&score_b).unwrap()
        })
        .cloned()
}

fn accent_score(entry: &ColorEntry, dominant_lab: LabColor) -> f32 {
    let lab = LabColor {
        l: entry.lab.l,
        a: entry.lab.a,
        b: entry.lab.b,
    };
    lab.delta_e(dominant_lab) * entry.lch.c
}

fn suggest_background(dominant: LabColor) -> String {
    let bg_lab = if dominant.l > 70.0 {
        LabColor {
            l: dominant.l - 10.0,
            a: dominant.a * 0.1,
            b: dominant.b * 0.1,
        }
    } else {
        LabColor {
            l: 95.0,
            a: dominant.a * 0.05,
            b: dominant.b * 0.05,
        }
    };
    lab_to_rgb(bg_lab).to_hex()
}
