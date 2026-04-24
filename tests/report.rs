use pixel_analyzer::report;
use pixel_analyzer::types::{LabColor, RgbColor};

#[test]
fn report_dominant_is_highest_population_cluster() {
    let c1 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 100.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 10,
        total_pixels: 15,
    };
    let c2 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 5,
        total_pixels: 15,
    };
    let clusters = vec![c1, c2];
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
    let report = report::build(&clusters, &rgb, &lab, 1, 2, 0.0, None);
    assert!(report.main.dominant.hex.to_uppercase().contains("FFFFFF"));
}

#[test]
fn report_accent_is_none_for_monochromatic_input() {
    let c1 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 1,
        total_pixels: 2,
    };
    let c2 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 51.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 1,
        total_pixels: 2,
    };
    let clusters = vec![c1, c2];
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128,
        },
        RgbColor {
            r: 130,
            g: 130,
            b: 130,
        },
    ];
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        },
        LabColor {
            l: 51.0,
            a: 0.0,
            b: 0.0,
        },
    ];
    let report = report::build(&clusters, &rgb, &lab, 1, 2, 0.0, None);
    assert!(report.main.accent.is_none());
    assert!(
        report
            .warning
            .unwrap()
            .contains("No perceptually distinct accent colour")
    );
}

#[test]
fn report_accent_present_for_contrasting_input() {
    let c1 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 1,
        total_pixels: 2,
    };
    let c2 = pixel_analyzer::types::Cluster {
        centroid: LabColor {
            l: 90.0,
            a: 0.0,
            b: 0.0,
        },
        pixel_count: 1,
        total_pixels: 2,
    };
    let clusters = vec![c1, c2];
    let rgb = vec![
        RgbColor {
            r: 25,
            g: 25,
            b: 25,
        },
        RgbColor {
            r: 230,
            g: 230,
            b: 230,
        },
    ];
    let lab = vec![
        LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        },
        LabColor {
            l: 90.0,
            a: 0.0,
            b: 0.0,
        },
    ];
    let report = report::build(&clusters, &rgb, &lab, 1, 2, 0.0, None);
    assert!(report.main.accent.is_some());
}
