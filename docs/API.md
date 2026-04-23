# API Documentation

This document provides a detailed reference for the `pixel-analyzer` JavaScript/TypeScript API.

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
  main: {
    dominant: ColorEntry;              // Most frequent color group
    accent: ColorEntry;                // Most visually distinct vibrant color
    background_suggestion: string;     // Hex code for a matching background
    foreground_suggestion: string;     // "#000000" or "#FFFFFF" for accessibility
  };
  palettes: {
    vibrant: ColorEntry[];             // Highly saturated colors (C* > 28)
    muted: ColorEntry[];               // Low saturation colors (C* < 15)
    light: ColorEntry[];               // High luminance colors (L* > 80)
    dark: ColorEntry[];                // Low luminance colors (L* < 20)
    raw: ColorEntry[];                 // Unfiltered results from K-Means
  };
  accessibility: {
    contrast_ratio: number;            // Contrast between dominant and accent
    is_aa_normal: boolean;             // Passes WCAG AA Large Text
    is_aaa_normal: boolean;            // Passes WCAG AAA
    recommended_font_color: string;    // Best legible font color for the dominant color
  };
  image_stats: {
    brightness: number;                // Mean lightness (0-100)
    colorfulness: number;              // Global saturation metric (0-100)
    entropy: number;                   // Visual complexity (0.0-8.0)
    dominant_hue_group: "warm" | "cool" | "neutral";
    orientation: "landscape" | "portrait" | "square";
  };
  color_theory: {
    complementary: string;             // Hex at +180 degrees
    triadic: [string, string];         // Hexes at ±120 degrees
    analogous: [string, string];       // Hexes at ±30 degrees
  };
  analysis_time_ms: number;            // Total processing time
  pixels_analyzed: number;             // Count of pixels processed
  warning?: string;                    // Non-fatal processing warnings
}

interface ColorEntry {
  hex: string;                         // Web-ready hex string (e.g., "#3498DB")
  population: number;                  // Ratio of pixels in this cluster (0.0 - 1.0)
  is_dark: boolean;                    // True if L* < 50
  lab: { l: number; a: number; b: number };
  lch: { l: number; c: number; h: number };
}
```
