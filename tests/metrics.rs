use pixel_analyzer::metrics;
use pixel_analyzer::types::{LabColor, RgbColor};

#[test]
fn entropy_zero_for_uniform_image() {
    let pixels = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        };
        100
    ];
    let stats = metrics::compute(
        &vec![
            RgbColor {
                r: 128,
                g: 128,
                b: 128,
            };
            100
        ],
        &pixels,
        0.0,
        10,
        10,
    );
    assert!(stats.entropy < 0.01);
}

#[test]
fn colorfulness_zero_for_grey_image() {
    let rgb = vec![
        RgbColor {
            r: 128,
            g: 128,
            b: 128,
        };
        100
    ];
    let lab = vec![
        LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        };
        100
    ];
    let stats = metrics::compute(&rgb, &lab, 0.0, 10, 10);
    assert!(stats.colorfulness < 0.01);
}
