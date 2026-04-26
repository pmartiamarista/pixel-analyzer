use pixel_analyzer::report;
use pixel_analyzer::types::{Cluster, LabColor, RgbColor};

fn mock_clusters(l1: f32, l2: f32) -> Vec<Cluster> {
    vec![
        Cluster {
            centroid: LabColor {
                l: l1,
                a: 0.0,
                b: 0.0,
            },
            pixel_count: 10,
            total_pixels: 15,
        },
        Cluster {
            centroid: LabColor {
                l: l2,
                a: 0.0,
                b: 0.0,
            },
            pixel_count: 5,
            total_pixels: 15,
        },
    ]
}

fn mock_pixels() -> (Vec<RgbColor>, Vec<LabColor>) {
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128,
        },
        RgbColor { r: 0, g: 0, b: 0 },
    ];
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        },
        LabColor {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        },
    ];
    (rgb, lab)
}

#[test]
fn report_dominant_is_highest_population_cluster() {
    let clusters = mock_clusters(100.0, 0.0);
    let (rgb, lab) = mock_pixels();
    let report = report::build(
        report::ReportInputs {
            clusters: &clusters,
            rgb_pixels: &rgb,
            lab_pixels: &lab,
            width: 1,
            height: 2,
        },
        0.0,
        None,
    );
    assert!(report.main.dominant.hex.to_uppercase().contains("FFFFFF"));
}

#[test]
fn report_accent_is_none_for_monochromatic_input() {
    let clusters = mock_clusters(50.0, 51.0);
    let (rgb, lab) = mock_pixels();
    let report = report::build(
        report::ReportInputs {
            clusters: &clusters,
            rgb_pixels: &rgb,
            lab_pixels: &lab,
            width: 1,
            height: 2,
        },
        0.0,
        None,
    );
    assert!(report.main.accent.is_none());
}

#[test]
fn report_accent_present_for_contrasting_input() {
    let clusters = mock_clusters(10.0, 90.0);
    let (rgb, lab) = mock_pixels();
    let report = report::build(
        report::ReportInputs {
            clusters: &clusters,
            rgb_pixels: &rgb,
            lab_pixels: &lab,
            width: 1,
            height: 2,
        },
        0.0,
        None,
    );
    assert!(report.main.accent.is_some());
}
