# Changelog

## [0.1.2] - 2026-04-23

### Changed
- Migrated project to Rust 2024 Edition.
- Updated all dependencies to their latest stable versions (`wasm-bindgen`, `serde`, `image`, etc.).
- Fixed clippy warnings arising from the new compiler version (`collapsible_if`).

## [0.1.1] — 2026-04-23

### Compliance & Refactoring
- [refactor]: delete orphaned root-level .rs files; src/ is canonical
- [refactor]: purge all inline and block comments from src/ per comments-docs policy
- [refactor]: updated Makefile to match project structure and removed stale references
- [refactor]: extract HueGroup and Orientation enums; remove stringly-typed ImageStats fields
- [refactor]: extract validate_buffer and serialise from run_pipeline; enforce single abstraction level
- [fix]: remove .map(|v| v) no-op from analyze()
- [fix]: remove stale development comment from report.rs
- [feat]: add README with full API reference, quick-start, and pipeline overview
- [feat]: add CHANGELOG
- [test]: replace placeholder integration test with 14 real colour-pipeline tests
- [refactor]: remove placeholder repository URL from Cargo.toml

## [0.1.0] — 2026-04-10

### Initial Implementation
- [feat]: WASM entry point — init / analyze / terminate lifecycle
- [feat]: sRGB → CIE XYZ D65 → CIELAB → CIELCh transform chain
- [feat]: K-Means++ clustering with ΔE convergence and deterministic LCG
- [feat]: two-stage spatial sampler — nearest-neighbour downsample + 32×32 stratified grid
- [feat]: WCAG 2.1 contrast ratio (AA / AAA) and recommended font colour
- [feat]: Hasler-Suesstrunk colorfulness metric
- [feat]: Shannon entropy over L* histogram
- [feat]: complementary, triadic, and analogous colour harmony generation via LCh rotation
- [feat]: typed AnalyzerError enum with JS-compatible rejection messages
- [feat]: Quality enum — Draft / Balanced / Precise sampling modes
- [fix]: is_dark threshold corrected to L* < 50 (v2.0 spec correction)
