# pixel-analyzer v0.1.5
> High-performance perceptual colour intelligence for the browser, powered by Rust & WebAssembly.

`pixel-analyzer` extracts meaningful colour palettes and image-level metrics entirely in the [CIELAB](https://en.wikipedia.org/wiki/CIELAB_color_space) perceptual colour space. It uses a refined **K-Means++** clustering engine with ΔE (CIE 1976) convergence to ensure that extracted colours are perceptually distinct and accurate.

## Core Features
- **All-in-Rust Pipeline**: Decoding, sampling, transformation, and clustering happen entirely in WASM.
- **Perceptual Accuracy**: All math operates in CIELAB/CIELCh space, not RGB.
- **Stratified Sampling**: A 2-stage spatial sampler ensures small but vibrant details (like logos) aren't missed.
- **Accessibility**: Built-in WCAG 2.1 contrast ratio calculations and font colour recommendations.
- **Advanced Metrics**: Visual entropy (Shannon), colorfulness (Hasler-Suesstrunk), and brightness.
- **Harmonies**: Automatic generation of complementary, triadic, and analogous colour schemes.

## Installation & Build

```bash
# Clone and build for production
wasm-pack build --target web --release
```

The resulting `pkg/` directory contains the compiled `.wasm` (< 500 KB) and JS bindings ready for integration.

## Quick Start

```javascript
import init, { analyze, AnalysisOptions, Quality } from './pkg/pixel_analyzer.js';

async function run() {
    await init();

    const response = await fetch('image.jpg');
    const bytes = new Uint8Array(await response.arrayBuffer());

    const options = AnalysisOptions.defaults();

    const report = await analyze(bytes, options);
    console.log('Dominant:', report.main.dominant.hex);

    if (report.main.accent) {
        console.log('Accent:', report.main.accent.hex);
    }

    console.log('Vibrant Palette:', report.palettes.vibrant);
}
```

## API Reference

### `init(): Promise<void>`
Initialises the WASM module. Must be called once before any other function.

### `analyze(data: Uint8Array, options?: AnalysisOptions): Promise<AnalysisReport>`
Runs the full analysis pipeline on the provided image buffer.
- `data`: Raw bytes of a PNG, JPEG, or WebP image.
- `options`: Optional configuration object.

### `terminate(): void`
Optional teardown hook for explicitly releasing resources in SPA environments.

## AnalysisOptions

| Option | Type | Range | Default | Description |
|---|---|---|---|---|
| `max_colors` | `number` | 2–16 | 5 | Max number of clusters to extract. |
| `quality` | `Quality` | Draft/Balanced/Precise | Balanced | Controls sampling fraction (25%/50%/100%). |
| `convergence` | `number` | > 0 | 1.0 | K-Means ΔE early-exit threshold. |

## Supported Formats
Pure Rust decoding support for:
- PNG
- JPEG
- WebP

## Pipeline Overview
1. **Validation**: Config and buffer integrity checks.
2. **Decoding**: Fast Magic-byte detection and memory loading.
3. **Sampling**: Adaptive downscaling + 32×32 stratified grid selection.
4. **Transform**: sRGB → CIE XYZ D65 → CIELAB conversion.
5. **Clustering**: K-Means++ with ΔE (CIE 1976) convergence.
6. **Reporting**: Semantic palette classification & metric computation.
7. **Serialisation**: Efficient conversion to JS-compatible JSON.

## Development

Use the provided `Makefile` for standard development tasks:

```bash
# Run all quality checks (fmt + clippy)
make verify

# Run unit and integration tests
make test

# Generate and open documentation
make doc

# Clean build artifacts
make clean
```

## Documentation

For deeper dives into the technical implementation and the full API specification, refer to the following documents:

- **[API Reference](docs/API.md)**: Detailed TypeScript definitions and function signatures.
- **[Technical Specification (DETA)](docs/DETA.md)**: Mathematical foundations, system architecture, and clustering logic.
- **[Changelog](CHANGELOG.md)**: Per-version history of all changes.

## License
MIT
