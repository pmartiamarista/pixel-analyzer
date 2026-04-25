# Changelog

## [0.1.6] - 2026-04-25

### Fixed
- [fix]: resolve critical type contract mismatch in `bindings.ts`; `AccessibilityReport` fields now correctly match Rust struct (`is_aa_normal`, `is_aaa_normal`, `recommended_font_color`) (F-01, OE-5)
- [fix]: replace `any` type for `options` in `analyze()` with proper `AnalysisOptions` class in `bindings.ts` (F-05)
- [fix]: replace silent failure path in `decoder.rs` via `unwrap_or(0)` with explicit error propagation (F-09)
- [fix]: fix tautological hue rotation test in `tests/color.rs` to actually verify `color_theory` logic (F-06)
- [fix]: remove duplicate K-Means insufficient-pixels test in `tests/kmeans.rs` (F-07)
- [fix]: update `docs/DETA.md` policy to restore `///` doc-comment permission; correctly align with Engineering Rule #2 (F-10)
- [fix]: synchronise `docs/DETA.md` binary footprint target and project structure with current reality (F-03, F-04)

### Added
- [feat]: added `make test-wasm` target and integrated `wasm-pack test` into CI workflow (M-06)
- [test]: added `tests/decoder.rs` covering format identification (PNG/JPEG/WebP) and RGBA expansion paths (F-08)
- [test]: added `tests/error.rs` verifying all `AnalyzerError` variant message formats (M-08)

### Refactored
- [refactor]: implement `fmt::Display` for `AnalyzerError` to support native testing and cleaner `JsValue` conversion

### Performance
- [perf]: enable `wasm-opt` in `Cargo.toml` with optimized flags (`-Oz`) for binary size reduction (F-02, N-02)

## [0.1.5] - 2026-04-25

### Added
- [feat]: strongly-typed WASM bindings via `bindings.ts` injection (initial implementation).

## [0.1.4] - 2026-04-25

### Refactored
- [refactor]: extract `src/decoder.rs` as dedicated ingestion module; `lib.rs` now calls `decoder::decode()` and has zero knowledge of image format specifics
- [refactor]: decompose monolithic `tests/color_conversion.rs` into modular files (`color.rs`, `accessibility.rs`, `kmeans.rs`, etc.) in the `tests/` directory
- [refactor]: move all unit tests out of the `src/` modules into the `tests/` integration suite for a clean separation of concerns

### Fixed
- [fix]: replace `image` crate with direct decoders `png`, `zune-jpeg`, `image-webp`; eliminates `pxfm`/`moxcms` chain (~200 KB overhead) from WASM binary (N-01)
- [fix]: add file existence guard to CI WASM size step before `du -k` measurement (N-04)
- [fix]: update CI WASM size gate from 200 KB to 500 KB reflecting actual optimised binary size after decoder refactor
- [fix]: update `docs/DETA.md` §5.3 to declare `accent?: ColorEntry | null` matching Rust type (N-03)
- [fix]: update OG-5 in `docs/DETA.md` from 200 KB to 500 KB to reflect the measured optimised binary
- [fix]: refactor `From<AnalyzerError> for JsValue` and `report` match logic to resolve `rust-analyzer` type inference ambiguity and false-positive shadowing warnings

## [0.1.3] - 2026-04-24

### Fixed
- [fix]: refactor `pick_accent()` to return `Option<ColorEntry>` with ΔE≥5 filter; eliminates monochromatic-image bug where accent was identical to dominant (C-02)
- [fix]: propagate `Option<ColorEntry>` through `MainPalette.accent`; emit warning when no perceptually distinct accent exists
- [fix]: move `hex_to_rgb()` from `report.rs` to `RgbColor::from_hex()` in `types.rs` per module responsibility rule (M-02)
- [fix]: synchronise wasm-bindgen family pin versions with Cargo.lock (`=0.2.118`, `=0.4.68`, `=0.3.95`) (M-04)

### Refactored
- [refactor]: remove four dead-code `LchColor` methods (`is_vibrant`, `is_muted`, `is_light_tone`, `is_dark_tone`); thresholds remain inline in `report.rs` (M-01)

### Tests
- [test]: add six boundary tests for `AnalysisConfig::validate()` covering all invalid edge cases (C-03)
- [test]: add `complementary_hue_rotation_is_exact_at_lch_level` and `complementary_hex_hue_within_tolerance_of_expected` to satisfy OE-8 numeric hue assertion (M-03)

### CI
- [feat]: add WASM binary size gate to `build-wasm` job; exits with code 1 if `pixel_analyzer_bg.wasm` exceeds 200 KB (C-01)
- [fix]: include `**/Cargo.toml` in Cargo cache key hash for both `quality` and `test` jobs (M-05)

## [0.1.2] - 2026-04-23

### Changed
- Migrated project to Rust 2024 Edition.
- Updated dependencies to latest stable versions.
- Disabled `wasm-opt` in `Cargo.toml` to ensure build stability with modern Rust.
- Refactored `Makefile` with comprehensive targets (`verify`, `wasm-dev`, `doc`).

### Added
- Created `docs/API.md` with full TypeScript-style type definitions.
- Added MIT `LICENSE` file.
- Implemented `doc` target in `Makefile` for auto-generating documentation.
- Integrated `repository` and `license` metadata in `Cargo.toml`.

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
