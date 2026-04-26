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

pub struct ReportInputs<'a> {
    pub clusters: &'a [Cluster],
    pub rgb_pixels: &'a [RgbColor],
    pub lab_pixels: &'a [LabColor],
    pub width: u32,
    pub height: u32,
}

pub fn build(
    inputs: ReportInputs,
    analysis_time_ms: f64,
    warning: Option<String>,
) -> AnalysisReport {
    let entries: Vec<ColorEntry> = inputs.clusters.iter().map(cluster_to_entry).collect();
    let dominant = entries[0].clone();
    let accent = pick_accent(&entries, &dominant);

    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: inputs.rgb_pixels,
        lab_pixels: inputs.lab_pixels,
        dominant_hue: dominant.lch.h,
        width: inputs.width,
        height: inputs.height,
    });
    AnalysisReport {
        main: MainPalette {
            dominant: dominant.clone(),
            accent: accent.clone(),
            background_suggestion: suggest_background(inputs.clusters[0].centroid),
            foreground_suggestion: accessibility::best_font_color(RgbColor::from_hex(
                &entries[0].hex,
            )),
        },
        palettes: build_palettes(&entries),
        accessibility: build_accessibility(&dominant, &accent),
        image_stats: stats,
        color_theory: build_theory(&dominant),
        analysis_time_ms,
        pixels_analyzed: inputs.lab_pixels.len(),
        warning: merge_warnings(warning, accent.is_none()),
    }
}

fn build_palettes(entries: &[ColorEntry]) -> Palettes {
    Palettes {
        vibrant: entries.iter().filter(|e| e.lch.c > 28.0).cloned().collect(),
        muted: entries.iter().filter(|e| e.lch.c < 15.0).cloned().collect(),
        light: entries.iter().filter(|e| e.lch.l > 80.0).cloned().collect(),
        dark: entries.iter().filter(|e| e.lch.l < 20.0).cloned().collect(),
        raw: entries.to_vec(),
    }
}

fn build_accessibility(dom: &ColorEntry, acc: &Option<ColorEntry>) -> AccessibilityReport {
    acc.as_ref()
        .map(|ac| {
            accessibility::evaluate(RgbColor::from_hex(&dom.hex), RgbColor::from_hex(&ac.hex))
        })
        .unwrap_or_else(|| {
            accessibility::evaluate(RgbColor::from_hex(&dom.hex), RgbColor::from_hex(&dom.hex))
        })
}

fn build_theory(dom: &ColorEntry) -> ColorTheory {
    color_theory::generate(crate::types::LchColor {
        l: dom.lch.l,
        c: dom.lch.c,
        h: dom.lch.h,
    })
}

fn merge_warnings(base: Option<String>, no_accent: bool) -> Option<String> {
    let accent_w = no_accent.then_some(
        "No perceptually distinct accent colour found (ΔE < 5 from dominant).".to_string(),
    );
    match (base, accent_w) {
        (Some(w), Some(a)) => Some(format!("{} {}", w, a)),
        (w, a) => w.or(a),
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
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
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
