use pixel_analyzer::kmeans;
use pixel_analyzer::types::LabColor;

#[test]
fn kmeans_clusters_two_separated_groups() {
    let mut pixels = Vec::new();
    for _ in 0..100 {
        pixels.push(LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        });
    }
    for _ in 0..100 {
        pixels.push(LabColor {
            l: 90.0,
            a: 0.0,
            b: 0.0,
        });
    }
    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 0.1, 100).unwrap();
    assert_eq!(clusters.len(), 2);
}

#[test]
fn kmeans_returns_error_on_insufficient_pixels() {
    let pixels = vec![LabColor {
        l: 50.0,
        a: 0.0,
        b: 0.0,
    }];
    let result = kmeans::kmeans_plus_plus(&pixels, 5, 1.0, 100);
    assert!(result.is_err());
}

#[test]
fn clusters_separate_colours() {
    let mut pixels = Vec::new();
    for i in 0..50 {
        pixels.push(LabColor {
            l: 1.0 + i as f32 * 0.01,
            a: 0.0,
            b: 0.0,
        });
    }
    for i in 0..50 {
        pixels.push(LabColor {
            l: 99.0 - i as f32 * 0.01,
            a: 0.0,
            b: 0.0,
        });
    }

    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 1.0, 100).unwrap();
    assert_eq!(clusters.len(), 2);

    assert!(clusters[0].pixel_count >= 40);
    assert!(clusters[1].pixel_count >= 40);
}

#[test]
fn clusters_are_sorted_by_population_descending() {
    let mut pixels = Vec::new();
    for _ in 0..100 {
        pixels.push(LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        });
    }
    for _ in 0..50 {
        pixels.push(LabColor {
            l: 90.0,
            a: 0.0,
            b: 0.0,
        });
    }
    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 1.0, 100).unwrap();
    assert_eq!(clusters.len(), 2);
    assert!(clusters[0].pixel_count >= clusters[1].pixel_count);
}

#[test]
fn kmeans_succeeds_when_k_equals_pixel_count() {
    let pixels = vec![
        LabColor {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        },
        LabColor {
            l: 20.0,
            a: 0.0,
            b: 0.0,
        },
    ];
    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 1.0, 100);
    assert!(clusters.is_ok());
}

#[test]
fn kmeans_converges_with_tight_epsilon() {
    let mut pixels = Vec::new();
    for _ in 0..10 {
        pixels.push(LabColor {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        });
    }
    for _ in 0..10 {
        pixels.push(LabColor {
            l: 60.0,
            a: 0.0,
            b: 0.0,
        });
    }
    let clusters = kmeans::kmeans_plus_plus(&pixels, 2, 0.001, 100).unwrap();
    assert_eq!(clusters.len(), 2);
}
