# PixelAnalyzer WASM Core
## Technical Specification and Architecture Document (DETA)

**Version:** 2.1  
**Technologies:** Rust (v1.85+), WebAssembly, CIELAB / CIELCh  
**Status:** Final Specification — Ready for Implementation

---

## 1. Project Identity

### 1.1 Mission

> To provide developers and designers with a professional-grade visual analysis library that, running entirely on the client via WebAssembly, eliminates network latency and server dependency to extract chromatic intelligence, accessibility metrics, and design recommendations from any image, in any modern JS environment.

### 1.2 Vision

> To become the de facto standard for color processing in distributed environments (Web, Edge, Node.js), being the first library in the ecosystem to combine academic rigor (CIE 1976/2000, WCAG 2.1), memory safety guaranteed by Rust, and an API so expressive that its output can be integrated directly into a design system without post-processing.

### 1.3 General Objectives

| # | Objective |
|---|----------|
| OG-1 | Perform chromatic palette analysis with CIELAB-based perceptual accuracy. |
| OG-2 | Generate accessibility reports that comply with WCAG 2.1 standards (AA and AAA). |
| OG-3 | Produce design recommendations based on color theory (LCh, harmonies). |
| OG-4 | Guarantee constant analysis time independent of the original image size. |
| OG-5 | Deliver a WASM binary under 500 KB with zero runtime dependencies on native code. |

### 1.4 Specific Objectives

| # | Objective | Success Metric |
|---|----------|-----------------|
| OE-1 | Support PNG, JPEG, and WebP formats without C dependencies. Decoding via `png`, `zune-jpeg`, and `image-webp` crates directly. | Successful decoding in ≥ 99% of cases. |
| OE-2 | Extract between 2 and 16 dominant colors with K-Means++. | Convergence in < 100 iterations for ε = 1.0 ΔE. |
| OE-3 | Implement 32×32 grid stratified spatial sampling. | Representation of all regions in the sample. |
| OE-4 | Classify colors into vibrant, muted, light, and dark categories via CIELCh. | Thresholds: C* > 28 (vibrant), C* < 15 (muted), L* > 80 (light), L* < 20 (dark). |
| OE-5 | Calculate WCAG 2.1 contrast ratio between dominant and accent colors. | Return `is_aa_normal` (4.5:1) and `is_aaa_normal` (7:1) flags. |
| OE-6 | Measure global colorfulness using the Hasler-Suesstrunk metric. | Normalized result in the [0, 100] range. |
| OE-7 | Calculate visual entropy (complexity) with Shannon's formula. | Result in the [0.0, 8.0] bits range. |
| OE-8 | Suggest complementary, triadic, and analogous color harmonies from LCh space. | Accuracy of ±1° in the h* hue angle. |
| OE-9 | Expose a complete lifecycle: `init()`, `analyze()`, `terminate()`. | Zero memory leaks verified with WASM tooling. |

---

## 2. Design Philosophy

The development is guided by five core pillars:

- **KISS:** A single public entry function abstracts the entire pipeline. The consumer never manages WASM memory directly.
- **SOLID:** Each module has a single responsibility (color transformation, sampling, clustering, metrics, reporting). They are independent and testable in isolation.
- **YAGNI:** Advanced categorization, ICC profiles, and GIF/AVIF support are excluded from the v1.0 core and marked as extensible modules.
- **Clean Code:** Semantic naming in Rust (`delta_e`, `stratified_sample`, `kmeans_plus_plus`), explicit data contracts, and documentation containing bibliographic references.
- **No Comments:** All comments are forbidden — inline, block, and doc comments. Code must be self-documenting through naming.

---

## 3. System Architecture

### "All-in-Rust" Pipeline

```
[JS: Uint8Array] 
      │
      ▼
┌─────────────────────────────────────────────────────────┐
│  LAYER 1 · Ingestion and Validation                     │
│  · Config validation (max_colors ∈ [2,16], etc.)        │
│  · Format detection via magic bytes                     │
│  · PNG/JPEG/WebP decoding (Pure Rust, no C FFI)         │
└──────────────────────────┬──────────────────────────────┘
                           │ Vec<u8> (RGBA)
                           ▼
┌─────────────────────────────────────────────────────────┐
│  LAYER 2 · Spatial Normalization                        │
│  · Adaptive downsampling (area) → max. 512×512 px      │
│  · Stratified sampling in 32×32 grid                    │
│  · Fraction selection based on Quality (25/50/100 %)    │
└──────────────────────────┬──────────────────────────────┘
                           │ Vec<RgbColor>
                           ▼
┌─────────────────────────────────────────────────────────┐
│  LAYER 3 · Chromatic Transformation                     │
│  · sRGB (gamma) → sRGB (linear) → CIE XYZ (D65)        │
│  · CIE XYZ → CIELAB (L*a*b*)                           │
│  · CIELAB → CIELCh (L*, C*, h°) for categorization    │
└──────────────────────────┬──────────────────────────────┘
                           │ Vec<LabColor>
                           ▼
┌─────────────────────────────────────────────────────────┐
│  LAYER 4 · K-Means++ Engine                             │
│  · Intelligent centroid initialization                  │
│  · Iteration with ε = 1.0 ΔE early-exit                 │
│  · Max 100 iterations (safety cap)                      │
└──────────────────────────┬──────────────────────────────┘
                           │ Vec<Cluster>
                           ▼
┌─────────────────────────────────────────────────────────┐
│  LAYER 5 · Semantic Analysis and Metrics                │
│  · LCh Classification → vibrant / muted / light / dark  │
│  · WCAG 2.1 contrast ratio                             │
│  · Hasler-Suesstrunk colorfulness                       │
│  · Shannon entropy                                      │
│  · Color Harmonies (complementary, triadic, analogous)  │
└──────────────────────────┬──────────────────────────────┘
                           │ AnalysisReport (JSON)
                           ▼
                    [JS: Promise resolved]
```

---

## 4. Mathematical Foundations

### 4.1 sRGB → CIE XYZ Conversion (D65 Illuminant)

**Step 1 — Gamma Linearization:**
$$C_{linear} = \begin{cases} \dfrac{C_{sRGB}}{12.92} & \text{if } C_{sRGB} \leq 0.04045 \\ \left(\dfrac{C_{sRGB} + 0.055}{1.055}\right)^{2.4} & \text{if } C_{sRGB} > 0.04045 \end{cases}$$

**Step 2 — D65 Transformation Matrix:**
$$\begin{bmatrix} X \\ Y \\ Z \end{bmatrix} = \begin{bmatrix} 0.4124 & 0.3576 & 0.1805 \\ 0.2126 & 0.7152 & 0.0722 \\ 0.0193 & 0.1192 & 0.9505 \end{bmatrix} \cdot \begin{bmatrix} R_{linear} \\ G_{linear} \\ B_{linear} \end{bmatrix}$$

### 4.2 CIE XYZ → CIELAB Conversion

With white reference D65: $X_n = 0.95047$, $Y_n = 1.00000$, $Z_n = 1.08883$

$$f(t) = \begin{cases} t^{1/3} & \text{if } t > \left(\frac{6}{29}\right)^3 \\ \dfrac{t}{3(6/29)^2} + \dfrac{4}{29} & \text{otherwise} \end{cases}$$

$$L^* = 116 \cdot f(Y/Y_n) - 16, \quad a^* = 500 \cdot [f(X/X_n) - f(Y/Y_n)], \quad b^* = 200 \cdot [f(Y/Y_n) - f(Z/Z_n)]$$

### 4.3 CIELAB → CIELCh Conversion

$$C^* = \sqrt{a^{*2} + b^{*2}}, \quad h^* = \text{atan2}(b^*, a^*) \cdot \frac{180°}{\pi} \pmod{360°}$$

### 4.4 Perceptual Distance (ΔE CIE 1976)

$$\Delta E = \sqrt{(L_1^* - L_2^*)^2 + (a_1^* - a_2^*)^2 + (b_1^* - b_2^*)^2}$$

> A value of $\Delta E < 1.0$ is perceptually imperceptible. This value is used as the convergence threshold ε in K-Means++.

### 4.5 K-Means++ Initialization (Centroid Selection)

$$P(x) = \frac{D(x)^2}{\sum_{x' \in X} D(x')^2}$$

Where $D(x)$ is the distance to the nearest already-selected centroid.

### 4.6 Clustering Cost Function (Within-Cluster Sum of Squares)

$$J = \sum_{j=1}^{k} \sum_{i \in S_j} \| x_i - \mu_j \|^2$$

### 4.7 WCAG 2.1 Contrast Ratio

$$CR = \frac{L_1 + 0.05}{L_2 + 0.05}, \quad L_1 \geq L_2$$

Where relative luminance $Y$ is calculated from linearized sRGB:
$$Y = 0.2126 \cdot R_{linear} + 0.7152 \cdot G_{linear} + 0.0722 \cdot B_{linear}$$

### 4.8 Global Colorfulness (Hasler-Suesstrunk)

$$rg = R - G, \quad yb = \frac{R + G}{2} - B$$

$$M = \sqrt{\sigma_{rg}^2 + \sigma_{yb}^2} + 0.3 \cdot \sqrt{\mu_{rg}^2 + \mu_{yb}^2}$$

### 4.9 Visual Entropy (Shannon)

$$H = -\sum_{i=0}^{255} p_i \log_2(p_i)$$

Calculated over the luminance histogram of the $L^*$ channel quantized to 256 levels.

### 4.10 Darkness Classification *(v2.0 correction)*

$$is\_dark \iff L^* < 50.0$$

> **Note:** The v1.0 specification had this condition inverted (`L* > 50`). Corrected: a dark color has low $L^*$, with $L^* = 0$ being absolute black.

---

## 5. API Contract

### 5.1 Lifecycle

```typescript
// Initializes the WASM module (once per session)
init(): Promise<void>

// Main analysis — buffer is copied, processed, and released
analyze(data: Uint8Array, config?: Partial<AnalysisOptions>): Promise<AnalysisReport>

// Releases the module (useful in SPA / ephemeral Edge environments)
terminate(): void
```

### 5.2 AnalysisConfig (Input)

| Property | Type | Default | Validation |
|-----------|------|---------|-----------|
| `max_colors` | `number` (u8) | `5` | Range `[2, 16]` — error if out of range |
| `quality` | `Quality` | `Balanced` | `Draft` / `Balanced` / `Precise` |
| `convergence` | `number` (f32) | `1.0` | > 0.0 (ΔE CIE 1976 units) |

#### Quality Enum

| Variant | Sampling Resolution | Recommended Use |
|----------|----------------------|-----------------|
| `Draft` | 25% of 32×32 grid | Real-time, UI |
| `Balanced` | 50% of 32×32 grid | General use |
| `Precise` | 100% of 32×32 grid | Design / Printing |

### 5.3 AnalysisReport (Output)

```typescript
interface AnalysisReport {
  // 1. SEMANTIC HIERARCHY
  main: {
    dominant:              ColorEntry;           // Highest population density
    accent?:               ColorEntry | null;    // Maximizes ΔE and C* vs dominant; null when ΔE < 5
    background_suggestion: string;               // Optimized hex for backgrounds
    foreground_suggestion: string;               // "#000000" | "#FFFFFF"
  };

  // 2. CLASSIFIED PALETTES (LCh filter)
  palettes: {
    vibrant: ColorEntry[];  // C* > 28
    muted:   ColorEntry[];  // C* < 15
    light:   ColorEntry[];  // L* > 80
    dark:    ColorEntry[];  // L* < 20
    raw:     ColorEntry[];  // All clusters unfiltered
  };

  // 3. WCAG 2.1 ACCESSIBILITY
  accessibility: {
    contrast_ratio:         number;               // CR between dominant and accent
    is_aa_normal:           boolean;              // CR ≥ 4.5
    is_aaa_normal:          boolean;              // CR ≥ 7.0
    recommended_font_color: "#000000" | "#FFFFFF";
  };

  // 4. COMPOSITION METRICS
  image_stats: {
    brightness:         number;      // Mean L* [0-100]
    colorfulness:       number;      // Hasler-Suesstrunk [0-100]
    entropy:            number;      // Shannon [0.0-8.0]
    dominant_hue_group: HueGroup;    // "warm" | "cool" | "neutral"
    orientation:        Orientation; // "landscape" | "portrait" | "square"
  };

  // 5. COLOR HARMONIES (from LCh)
  color_theory: {
    complementary: string;            // h* + 180°
    triadic:       [string, string];  // h* ± 120°
    analogous:     [string, string];  // h* ± 30°
  };

  // 6. META
  analysis_time_ms:    number;
  pixels_analyzed:     number;
  warning?:            string;  // e.g. "Grayscale fallback"
}

interface ColorEntry {
  hex:        string;   // e.g. "#1A2B3C"
  population: number;   // fraction [0.0 – 1.0]
  is_dark:    boolean;  // L* < 50
  lab:        { l: number; a: number; b: number };
  lch:        { l: number; c: number; h: number };
}

type HueGroup = "warm" | "cool" | "neutral";
type Orientation = "landscape" | "portrait" | "square";
```

---

## 6. Project Structure

```
pixel-analyzer/
├── Cargo.toml                  # Dependencies and release profile
├── CHANGELOG.md                # Evolution of the project
├── README.md                   # Quick start and public documentation
├── DETA.md                     # This document (Technical Specification)
├── Makefile                    # Build and maintenance automation
├── src/
│   ├── lib.rs                  # WASM Entry Point · init/analyze/terminate
│   ├── error.rs                # AnalyzerError → JsValue conversions
│   ├── config.rs               # AnalysisConfig + Quality enum + validation
│   ├── types.rs                # Core types: RgbColor, LabColor, LchColor, etc.
│   ├── color.rs                # Space transforms: sRGB→XYZ→Lab→LCh
│   ├── sampler.rs              # Adaptive downsampling + stratified sampling
│   ├── kmeans.rs               # K-Means++ engine with early-exit
│   ├── report.rs               # AnalysisReport assembly
│   ├── accessibility.rs        # WCAG 2.1 contrast evaluation
│   ├── metrics.rs              # Hasler-Suesstrunk, Shannon entropy, stats
│   └── color_theory.rs         # Harmonies: complementary, triadic, analogous
└── tests/
    └── color_conversion.rs     # Unified integration and unit test suite
```

---

## 7. WASM Implementation Considerations

- **Asynchronous Execution:** `wasm-bindgen-futures` + `future_to_promise` ensures the browser's main thread remains responsive.
- **Binary Footprint:** `release` profile using `opt-level = "z"`, `lto = true`, and `panic = "abort"` to hit the < 200 KB target.
- **Memory Safety:** JS `Uint8Array` is copied into a Rust `Vec<u8>` at the start of `analyze()`. The Rust borrow checker manages automated deallocation. No raw pointers are exposed.
- **Error Handling:** Pipeline failures reject JS Promises with descriptive strings from the `AnalyzerError` enum.
- **Zero-Unsafe:** No `unsafe` blocks are permitted in the library. If performance profiles require it later, they must be documented with `// SAFETY:` justifications.

---

## 8. Bibliography

1. CIE (1976). *Colorimetry*, Publication 15.2. Bureau Central de la CIE, Vienna.
2. Arthur, D., & Vassilvitskii, S. (2007). "k-means++: The advantages of careful seeding". *Proc. ACM-SIAM SODA*.
3. W3C (2018). *Web Content Accessibility Guidelines (WCAG) 2.1*. W3C Recommendation.
4. Hasler, D., & Suesstrunk, S. E. (2003). "Measuring colorfulness in natural images". *IS&T/SPIE Electronic Imaging*.
5. Gonzalez, R. C., & Woods, R. E. (2018). *Digital Image Processing*, 4th ed. Pearson.
6. Cochran, W. G. (1977). *Sampling Techniques*. Wiley.
7. Shannon, C. E. (1948). "A Mathematical Theory of Communication". *Bell System Technical Journal*.
8. Poynton, C. (2012). *Digital Video and HD: Algorithms and Interfaces*. Morgan Kaufmann.
9. Luo, M. R., Cui, G., & Rigby, B. (2001). "The development of CIEDE2000". *Color Research & Application*.
10. Fitzgerald, A. et al. (2019–2025). *wasm-bindgen*. Rust and WebAssembly Working Group. https://github.com/rustwasm/wasm-bindgen
