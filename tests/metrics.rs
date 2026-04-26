use pixel_analyzer::metrics;
use pixel_analyzer::types::{LabColor, RgbColor};

#[test]
fn entropy_zero_for_uniform_image() {
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0
        };
        100
    ];
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128
        };
        100
    ];
    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &rgb,
        lab_pixels: &lab,
        dominant_hue: 0.0,
        width: 10,
        height: 10,
    });
    assert!(stats.entropy < 0.01);
}

#[test]
fn colorfulness_zero_for_grey_image() {
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128
        };
        100
    ];
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0
        };
        100
    ];
    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &rgb,
        lab_pixels: &lab,
        dominant_hue: 0.0,
        width: 10,
        height: 10,
    });
    assert!(stats.colorfulness < 0.01);
}

#[test]
fn classify_hue_group_warm() {
    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &[],
        lab_pixels: &[],
        dominant_hue: 0.0,
        width: 10,
        height: 10,
    });
    assert_eq!(
        stats.dominant_hue_group,
        pixel_analyzer::types::HueGroup::Warm
    );

    let stats2 = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &[],
        lab_pixels: &[],
        dominant_hue: 350.0,
        width: 10,
        height: 10,
    });
    assert_eq!(
        stats2.dominant_hue_group,
        pixel_analyzer::types::HueGroup::Warm
    );
}

#[test]
fn classify_hue_group_cool() {
    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &[],
        lab_pixels: &[],
        dominant_hue: 200.0,
        width: 10,
        height: 10,
    });
    assert_eq!(
        stats.dominant_hue_group,
        pixel_analyzer::types::HueGroup::Cool
    );
}

#[test]
fn classify_hue_group_neutral() {
    let stats = metrics::compute(metrics::MetricsInputs {
        rgb_pixels: &[],
        lab_pixels: &[],
        dominant_hue: 100.0,
        width: 10,
        height: 10,
    });
    assert_eq!(
        stats.dominant_hue_group,
        pixel_analyzer::types::HueGroup::Neutral
    );
}
