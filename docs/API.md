# API Documentation — pixel-analyzer

This document provides a detailed reference for the `pixel-analyzer` JavaScript/TypeScript API.

## Lifecycle
The mandatory call order for using the library is:

1. `init()` — initialise WASM memory. Must be called once per session.
2. `analyze(buffer, options)` — run pipeline. Returns `AnalysisReport`.
3. `terminate()` — free WASM memory. Optional but recommended for SPAs.

Calling `analyze()` before `init()` will result in an `Error: WASM module not initialized`.

## Core Functions

### `init(): Promise<void>`
Initializes the WebAssembly module. This must be called once before any other function is executed.

### `analyze(data: Uint8Array, options?: AnalysisOptions): Promise<AnalysisReport>`
The main entry point for image analysis.
- **`data`**: A `Uint8Array` containing the raw bytes of the image (PNG, JPEG, or WebP).
- **`options`**: (Optional) An instance of `AnalysisOptions`.

### `terminate(): void`
Explicitly releases resources. Useful for high-performance applications or Single Page Applications (SPAs) where the module might be re-loaded frequently.

---

## Configuration Types

### `AnalysisOptions`
A class used to configure the analysis behavior.

- **`new(max_colors: number, quality: Quality, convergence: number)`**: Constructor.
- **`static defaults()`**: Returns an instance with standard balanced settings.

| Property | Type | Description |
| :--- | :--- | :--- |
| `max_colors` | `number` | Number of clusters to extract (2 to 16). |
| `quality` | `Quality` | Sampling density (Draft, Balanced, Precise). |
| `convergence` | `number` | Delta-E threshold for K-Means convergence. Lower is more precise. |

### `Quality` (Enum)
- **`Draft`**: Samples 25% of the image grid. Fastest performance.
- **`Balanced`**: Samples 50% of the image grid. Good for general production use.
- **`Precise`**: Samples 100% of the image grid. Best for high-fidelity palette extraction.

---

## Output Data Model

### `AnalysisReport`
The structured result returned by the `analyze` function.

```typescript
interface AnalysisReport {
  main: MainPalette;
  palettes: Palettes;
  accessibility: AccessibilityReport;
  image_stats: ImageStats;
  color_theory: ColorTheory;
  analysis_time_ms: number;
  pixels_analyzed: number;
  warning?: string;
}

interface MainPalette {
  dominant: ColorEntry;
  accent?: ColorEntry | null;
  background_suggestion: string;
  foreground_suggestion: string;
}

interface Palettes {
  vibrant: ColorEntry[];
  muted: ColorEntry[];
  light: ColorEntry[];
  dark: ColorEntry[];
  raw: ColorEntry[];
}

interface AccessibilityReport {
  contrast_ratio: number;
  is_aa_normal: boolean;
  is_aaa_normal: boolean;
  recommended_font_color: string;
}

interface ImageStats {
  brightness: number;
  colorfulness: number;
  entropy: number;
  dominant_hue_group: "warm" | "cool" | "neutral";
  orientation: "landscape" | "portrait" | "square";
}

interface ColorTheory {
  complementary: string;
  triadic: [string, string];
  analogous: [string, string];
}

interface ColorEntry {
  hex: string;
  population: number;
  is_dark: boolean;
  lab: { l: number; a: number; b: number };
  lch: { l: number; c: number; h: number };
}
```
