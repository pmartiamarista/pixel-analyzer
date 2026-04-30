# pixel-analyzer
WASM library that extracts color palettes, WCAG contrast ratios, and colorfulness metrics from PNG, JPEG, and WebP images — runs in browser and Node.js.

## Quick Start

```ts
import init, { analyze } from './pkg/pixel_analyzer.js';

async function run() {
    await init();
    const bytes = new Uint8Array(await (await fetch('image.jpg')).arrayBuffer());
    const report = await analyze(bytes);
    
    console.log('Dominant:', report.main.dominant.hex);
    if (report.main.accent) {
        console.log('Accent:', report.main.accent.hex);
    }
}
```

## API Reference

### `init(): Promise<void>`
Initialises the WASM module. Must be called once before any other function.

### `analyze(data: Uint8Array, options?: AnalysisOptions): Promise<AnalysisReport>`
Runs the full analysis pipeline on the provided image buffer.

| Parameter | Type | Required | Description |
|---|---|---|---|
| `data` | `Uint8Array` | Yes | Raw bytes of PNG, JPEG, or WebP image |
| `options` | `AnalysisOptions` | No | Configuration for clusters and sampling |

**Returns:** `Promise<AnalysisReport>`

### `terminate(): void`
Explicitly releases resources. Useful for SPAs where the module might be re-loaded.

### `AnalysisOptions` (Type)

| Option | Type | Default | Description |
|---|---|---|---|
| `max_colors` | `number` | `5` | Clusters to extract (2 to 16) |
| `quality` | `Quality` | `Balanced` | Sampling density: `Draft` / `Balanced` / `Precise` |
| `convergence` | `number` | `1.0` | Delta-E threshold for K-Means early-exit |

### `AnalysisReport` (Type)

| Field | Type | Description |
|---|---|---|
| `main` | `MainPalette` | Semantic hierarchy (dominant, accent) |
| `palettes` | `Palettes` | Filtered lists (vibrant, muted, light, dark) |
| `accessibility` | `AccessibilityReport` | WCAG 2.1 contrast and font recommendations |
| `image_stats` | `ImageStats` | Brightness, colorfulness, entropy, etc. |
| `color_theory` | `ColorTheory` | Harmonies (complementary, triadic, analogous) |
| `analysis_time_ms`| `number` | Execution time in milliseconds |

## Pipeline Overview
Buffer validation → format detection (`decoder.rs`)
  → pixel decode + RGBA expansion
  → spatial downsample (32×32 stratified grid)
  → K-Means++ clustering (ΔE convergence)
  → palette assembly + accent selection
  → WCAG contrast + font color recommendation
  → report serialisation → JsValue

## Development

Prerequisites: Rust (v1.85+), `wasm-pack`, Node.js.

```bash
### Build
make wasm          # optimised WASM build (< 500 KB)
make wasm-dev      # development build (no wasm-opt)

### Test
make test          # Rust unit + integration suite
make test-wasm     # wasm-pack test via Node.js

### Verify
make verify        # clippy + fmt + test in sequence
```

## Documentation
- **[Technical Specification (DETA)](docs/DETA.md)**: Engineering rules and architecture.
- **[API Contract (API.md)](docs/API.md)**: Exhaustive type definitions.
- **[Changelog](CHANGELOG.md)**: Release history.

## License
MIT
