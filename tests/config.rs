use pixel_analyzer::config::AnalysisConfig;

#[test]
fn config_accepts_valid_defaults() {
    let cfg = AnalysisConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn config_rejects_max_colors_below_minimum() {
    let cfg = AnalysisConfig {
        max_colors: 0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn config_rejects_max_colors_of_one() {
    let cfg = AnalysisConfig {
        max_colors: 1,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn config_rejects_max_colors_above_maximum() {
    let cfg = AnalysisConfig {
        max_colors: 17,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn config_rejects_zero_convergence_threshold() {
    let cfg = AnalysisConfig {
        convergence_threshold: 0.0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn config_rejects_negative_convergence_threshold() {
    let cfg = AnalysisConfig {
        convergence_threshold: -1.0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn config_rejects_zero_max_iterations() {
    let cfg = AnalysisConfig {
        max_iterations: 0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}
