# API Documentation — pixel-analyzer

This document provides the complete reference for the `pixel-analyzer` JavaScript/TypeScript API.

---

## Lifecycle

The mandatory call order is:

1. `init()` — initialise WASM memory. Call once per session.
2. `analyze(buffer, options)` — run the full pipeline. Returns `AnalysisReport`.
3. `terminate()` — free WASM memory. Optional; useful in SPAs.

Calling `analyze()` before `init()` throws `Error: WASM module not initialized`.

```ts
import { init, analyze, AnalysisOptions } from 'pixel-analyzer';

await init();
const bytes = new Uint8Array(await (await fetch('image.jpg')).arrayBuffer());
const report = await analyze(bytes);

console.log('Dominant:',    report.main.dominant.hex);
console.log('WCAG ratio:',  report.accessibility.contrast_ratio);
console.log('APCA Lc:',     report.accessibility.apca.lc);
```

---

## Core Functions

### `init(): Promise<void>`
Initializes the WebAssembly module. Must be called once before any other function.

### `analyze(data, options?): Promise<AnalysisReport>`

| Parameter | Type | Required | Description |
|---|---|---|---|
| `data` | `Uint8Array` | Yes | Raw bytes of PNG, JPEG, or WebP image |
| `options` | `AnalysisOptions` | No | Cluster count, sampling density, convergence |

Returns `Promise<AnalysisReport>`.

### `terminate(): void`
Releases WASM resources. Recommended in SPAs that conditionally reload the module.

---

## Configuration

### `AnalysisOptions`

```ts
new AnalysisOptions(max_colors: number, quality: Quality, convergence: number)
AnalysisOptions.defaults() // Balanced / 5 colors / convergence 1.0
```

| Option | Type | Default | Range | Description |
|---|---|---|---|---|
| `max_colors` | `number` | `5` | 2 – 16 | K-Means cluster count |
| `quality` | `Quality` | `Balanced` | — | Spatial sampling density |
| `convergence` | `number` | `1.0` | > 0 | ΔE convergence threshold for K-Means early-exit |

### `Quality`

| Value | Fraction sampled | Use case |
|---|---|---|
| `Quality.Draft` | 25 % | Thumbnails, real-time preview |
| `Quality.Balanced` | 50 % | General production use |
| `Quality.Precise` | 100 % | High-fidelity extraction, print |

---

## Output Model

### `AnalysisReport`

| Field | Type | Description |
|---|---|---|
| `main` | `MainPalette` | Dominant and accent with background/foreground suggestions |
| `palettes` | `Palettes` | Vibrant, muted, light, dark, and raw cluster lists |
| `accessibility` | `AccessibilityReport` | WCAG 2.1 / 2.2 contrast + APCA |
| `image_stats` | `ImageStats` | Brightness, colorfulness, entropy, hue group, orientation |
| `color_theory` | `ColorTheory` | Complementary, triadic, analogous harmonies |
| `analysis_time_ms` | `number` | Wall-clock time in milliseconds |
| `pixels_analyzed` | `number` | Pixel count after spatial sampling |
| `warning?` | `string` | Present when grayscale input or no perceptually distinct accent |

### `MainPalette`

| Field | Type | Description |
|---|---|---|
| `dominant` | `ColorEntry` | Highest-population cluster |
| `accent?` | `ColorEntry \| null` | Most perceptually distinct cluster (ΔE ≥ 5 from dominant) |
| `background_suggestion` | `string` | Hex — muted variant of dominant for UI background |
| `foreground_suggestion` | `string` | `#000000` or `#FFFFFF` — readable on dominant |

### `Palettes`

| Field | Type | Filter |
|---|---|---|
| `vibrant` | `ColorEntry[]` | CIELCh C* > 28 |
| `muted` | `ColorEntry[]` | CIELCh C* < 15 |
| `light` | `ColorEntry[]` | L* > 80 |
| `dark` | `ColorEntry[]` | L* < 20 |
| `raw` | `ColorEntry[]` | All clusters, sorted by population descending |

### `ColorEntry`

| Field | Type | Description |
|---|---|---|
| `hex` | `string` | `#RRGGBB` format |
| `population` | `number` | Fraction of sampled pixels (0.0 – 1.0) |
| `is_dark` | `boolean` | `true` when L* < 50 |
| `lab` | `LabValues` | `{ l, a, b }` — CIE 1976 L\*a\*b\* |
| `lch` | `LchValues` | `{ l, c, h }` — CIE 1976 L\*C\*h\* |

---

## Accessibility

### `AccessibilityReport`

The report measures contrast between the **dominant color** (treated as background) and the **accent color** (treated as foreground). When no perceptually distinct accent exists, both operands are the dominant color and a `warning` is emitted.

#### WCAG 2.1 / 2.2

| Field | Type | Pass condition | Standard |
|---|---|---|---|
| `contrast_ratio` | `number` | — | WCAG 1.4.3 / 1.4.6 |
| `is_aa_normal` | `boolean` | ratio ≥ 4.5 : 1 | WCAG AA, SC 1.4.3 |
| `is_aaa_normal` | `boolean` | ratio ≥ 7.0 : 1 | WCAG AAA, SC 1.4.6 |
| `recommended_font_color` | `string` | `#000000` or `#FFFFFF` | — |

WCAG 2.2 (published October 2023) did **not** change contrast thresholds. The same 4.5 : 1 (AA) and 7.0 : 1 (AAA) ratios apply. The new WCAG 2.2 success criteria (2.4.11 – 2.4.13, 2.5.7, 2.5.8, 3.2.6, 3.3.7 – 3.3.9) concern UI implementation, not color analysis, and are outside the scope of this library.

#### Large text (WCAG 2.1 / 2.2)

Large text requires only a 3.0 : 1 ratio at AA (SC 1.4.3). The library does not provide a separate `is_aa_large` field because the classification of a rendered font as "large text" (≥ 18 pt, or ≥ 14 pt bold) is a layout concern. Use `contrast_ratio ≥ 3.0` directly where applicable.

---

### `ApcaReport` — APCA-W3 0.0.98G-4g

> **Status notice**: APCA is the contrast algorithm proposed for WCAG 3.0 (Silver). It is in a W3C Working Draft and is **not** a ratified standard. Implementations may change when WCAG 3.0 is published.

APCA replaces the WCAG 2.x ratio with **Lc (Lightness Contrast)**, a perceptually uniform, directional measure that accounts for spatial frequency (font size and weight).

| Field | Type | Description |
|---|---|---|
| `lc` | `number` | Signed Lc value, 2 d.p. Range ≈ −108 to +108 |
| `is_normal_polarity` | `boolean` | `true` when dark text on light background (lc > 0) |
| `passes_preferred` | `boolean` | \|Lc\| ≥ 90 — preferred for continuous/fluent body text |
| `passes_body_text` | `boolean` | \|Lc\| ≥ 75 — normal body text at standard sizes |
| `passes_large_text` | `boolean` | \|Lc\| ≥ 60 — large text (18 pt+) or bold (14 pt+) |
| `passes_ui_component` | `boolean` | \|Lc\| ≥ 45 — active icons, UI components, placeholder text |
| `passes_decorative` | `boolean` | \|Lc\| ≥ 30 — decorative text, inactive icons, disabled elements |
| `passes_visibility` | `boolean` | \|Lc\| ≥ 15 — above invisibility floor (perceptually present) |

**Polarity** (sign of `lc`):

| Sign | Meaning |
|---|---|
| Positive | Dark text on light background (normal polarity) |
| Negative | Light text on dark background (reverse polarity) |

Both polarities are tested against the same absolute thresholds.

**Reference minimum levels (from APCA-W3 conformance):**

| Use case | Minimum \|Lc\| |
|---|---|
| Fluent text (preferred) | 90 |
| Body text, columns, prose | 75 |
| Large / bold text headings | 60 |
| UI components, active icons | 45 |
| Decorative text, inactive icons | 30 |
| Visibility floor | 15 |

**Key algorithm differences from WCAG 2.x:**

| Property | WCAG 2.x | APCA |
|---|---|---|
| Linearisation | Piecewise sRGB (IEC 61966-2-1) | Simple `^2.4` power curve |
| Luminance coefficients | R 0.2126 / G 0.7152 / B 0.0722 | R 0.2126729 / G 0.7151522 / B 0.0721750 |
| Output | Symmetric ratio 1 : 1 – 21 : 1 | Signed Lc ≈ −108 to +108 |
| Directionality | None (symmetric) | Text vs. background matters |
| Font size weighting | None | Built into threshold table |
| Near-black handling | None | Soft clamp at Y = 0.022 |

---

## Image Statistics

### `ImageStats`

| Field | Type | Range | Description |
|---|---|---|---|
| `brightness` | `number` | 0 – 100 | Mean CIELCh L* over sampled pixels |
| `colorfulness` | `number` | 0 – 100 | Hasler-Suesstrunk M metric (IS&T/SPIE 2003) |
| `entropy` | `number` | 0 – 8 | Shannon entropy over L* histogram |
| `dominant_hue_group` | `"warm" \| "cool" \| "neutral"` | — | Hue band of dominant cluster |
| `orientation` | `"landscape" \| "portrait" \| "square"` | — | Aspect ratio class |

**Hue group boundaries (CIELCh h° of dominant):**

| Group | Range |
|---|---|
| `warm` | h° ∈ [0°, 70°) ∪ [330°, 360°) — reds, oranges, yellows |
| `cool` | h° ∈ [150°, 330°) — greens, blues, purples |
| `neutral` | h° ∈ [70°, 150°) — yellow-greens |

---

## Color Theory

### `ColorTheory`

All harmonies are generated via LCh hue rotation from the dominant cluster, preserving its L* and C*. The resulting hex values are back-converted through Lab → XYZ → sRGB.

| Field | Type | Rotation |
|---|---|---|
| `complementary` | `string` | +180° |
| `triadic` | `[string, string]` | +120°, +240° |
| `analogous` | `[string, string]` | −30°, +30° |

---

## Pipeline Overview

```
Buffer validation → format detection (decoder.rs)
  → pixel decode + RGBA expansion
  → spatial downsample (nearest-neighbour, max dimension by Quality)
  → stratified 32×32 grid sampling
  → sRGB → XYZ D65 → CIELAB → CIELCh transform
  → K-Means++ clustering (ΔE convergence)
  → palette assembly + accent selection (ΔE ≥ 5)
  → WCAG 2.1/2.2 contrast ratio
  → APCA-W3 Lc calculation
  → Hasler-Suesstrunk colorfulness + Shannon entropy
  → color theory harmonic generation
  → report serialisation → JsValue
```

---

## Bibliography

1. CIE (1976). *Colorimetry*, Publication 15.2. Bureau Central de la CIE.
2. Arthur & Vassilvitskii (2007). "k-means++: The advantages of careful seeding." *SODA*.
3. W3C (2018). *WCAG 2.1*. W3C Recommendation.
4. W3C (2023). *WCAG 2.2*. W3C Recommendation. [https://www.w3.org/TR/WCAG22/](https://www.w3.org/TR/WCAG22/)
5. Hasler & Suesstrunk (2003). "Measuring colorfulness in natural images." *IS&T/SPIE Electronic Imaging*.
6. Myndex Research (2022–2024). *APCA-W3 0.0.98G-4g*. [https://github.com/Myndex/SAPC-APCA](https://github.com/Myndex/SAPC-APCA)
7. W3C Accessibility Guidelines (WCAG) 3.0 Working Draft. [https://www.w3.org/TR/wcag-3.0/](https://www.w3.org/TR/wcag-3.0/)