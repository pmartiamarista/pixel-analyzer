/**
 * The final result of the image analysis pipeline.
 */
export interface AnalysisReport {
    /** Main palette containing dominant and accent colors. */
    main: MainPalette;
    /** Categorized palettes (vibrant, muted, light, dark, raw). */
    palettes: Palettes;
    /**
     * WCAG 2.1 / WCAG 2.2 accessibility evaluation plus APCA.
     *
     * Contrast ratios (contrast_ratio) implement WCAG 2.1 SC 1.4.3 (AA) and
     * SC 1.4.6 (AAA). These thresholds are unchanged in WCAG 2.2.
     * The `apca` field implements APCA-W3 0.0.98G-4g, the algorithm proposed
     * for WCAG 3.0 / Silver. It is NOT yet a published W3C standard.
     */
    accessibility: AccessibilityReport;
    /** Statistical image metrics (entropy, colorfulness, etc). */
    image_stats: ImageStats;
    /** Generated color harmonies based on the dominant color. */
    color_theory: ColorTheory;
    /** Total analysis time in milliseconds. */
    analysis_time_ms: number;
    /** Number of pixels sampled and analyzed. */
    pixels_analyzed: number;
    /** Any warnings encountered during processing (e.g. monochromatic image). */
    warning?: string;
}

/**
 * Summary of the primary colors and suggestions.
 */
export interface MainPalette {
    /** The color with the highest population in the image. */
    dominant: ColorEntry;
    /** A perceptually distinct accent color (DeltaE ≥ 5 from dominant). */
    accent?: ColorEntry;
    /** Recommended background color for UI around this image. */
    background_suggestion: string;
    /** Recommended font color (Black/White) for readability on the dominant color. */
    foreground_suggestion: string;
}

/**
 * Palettes categorized by their perceptual properties.
 */
export interface Palettes {
    /** High chroma / intense colors (CIELCh C* > 28). */
    vibrant: ColorEntry[];
    /** Low chroma / desaturated colors (CIELCh C* < 15). */
    muted: ColorEntry[];
    /** High lightness colors (L* > 80). */
    light: ColorEntry[];
    /** Low lightness colors (L* < 20). */
    dark: ColorEntry[];
    /** All extracted clusters before categorization, sorted by population. */
    raw: ColorEntry[];
}

/**
 * A single color result from the K-Means clustering.
 */
export interface ColorEntry {
    /** Standard hex representation (e.g. "#FF0000"). */
    hex: string;
    /** Normalized population fraction (0.0 to 1.0). */
    population: number;
    /** Indicates if the color is perceptually dark (L* < 50). */
    is_dark: boolean;
    /** Native CIELAB coordinates. */
    lab: LabValues;
    /** Native CIELCh coordinates. */
    lch: LchValues;
}

/**
 * CIELAB (CIE 1976 L*a*b*) color space values.
 */
export interface LabValues {
    l: number;
    a: number;
    b: number;
}

/**
 * CIELCh (CIE 1976 L*C*h*) color space values.
 */
export interface LchValues {
    l: number;
    c: number;
    h: number;
}

/**
 * APCA (Advanced Perceptual Contrast Algorithm) evaluation.
 *
 * Implements APCA-W3 0.0.98G-4g — the contrast algorithm proposed for
 * WCAG 3.0. Status: W3C Working Draft (not yet a published standard).
 *
 * Lc (Lightness Contrast) is a signed value:
 *   - Positive → dark text on light background (normal polarity)
 *   - Negative → light text on dark background (reverse polarity)
 *   - Range: approximately −108 to +108
 *
 * Minimum |Lc| thresholds (from APCA-W3 conformance levels):
 *   - Fluent text (preferred): |Lc| ≥ 90
 *   - Body text (≤ 14pt): |Lc| ≥ 75
 *   - Large / bold text (≥ 18pt or ≥ 14pt bold): |Lc| ≥ 60
 *   - UI components, icons, placeholder text: |Lc| ≥ 45
 *   - Decorative text, inactive icons, disabled elements: |Lc| ≥ 30
 *   - Visibility floor (perceptually present): |Lc| ≥ 15
 *
 * In this report, `text` = foreground (accent color or dominant when no accent),
 * `background` = dominant color.
 */
export interface ApcaReport {
    /**
     * Signed Lightness Contrast value (Lc), rounded to 2 decimal places.
     * Positive = normal polarity (dark on light). Negative = reverse (light on dark).
     */
    lc: number;
    /** True if lc > 0. Explicit dark-on-light flag. */
    is_normal_polarity: boolean;
    /** |Lc| ≥ 90. Preferred for continuous/fluent body text. */
    passes_preferred: boolean;
    /** |Lc| ≥ 75. Required for body/normal text at standard sizes. */
    passes_body_text: boolean;
    /** |Lc| ≥ 60. Required for large text (18pt+) or bold text (14pt+). */
    passes_large_text: boolean;
    /** |Lc| ≥ 45. Required for active UI components, icons, and placeholder text. */
    passes_ui_component: boolean;
    /** |Lc| ≥ 30. Required for decorative text, inactive icons, and disabled elements. */
    passes_decorative: boolean;
    /** |Lc| ≥ 15. Above the invisibility floor (element is perceptually present). */
    passes_visibility: boolean;
}

/**
 * WCAG 2.1 / WCAG 2.2 contrast evaluation plus APCA.
 *
 * contrast_ratio uses the WCAG relative luminance formula (IEC 61966-2-1 sRGB,
 * D65 whitepoint). Thresholds cover all three WCAG contrast success criteria:
 *   SC 1.4.3  — normal text AA / large text AA  (ratio ≥ 4.5 / ≥ 3.0)
 *   SC 1.4.6  — normal text AAA / large text AAA (ratio ≥ 7.0 / ≥ 4.5)
 *   SC 1.4.11 — non-text contrast AA (icons, UI components) (ratio ≥ 3.0)
 * These thresholds are identical in WCAG 2.2 — the standard did not change
 * contrast requirements between 2.1 and 2.2.
 *
 * "Large text" under WCAG 2.x is ≥ 18 pt (24 CSS px) regular weight,
 * or ≥ 14 pt (18.67 CSS px) bold. Font-size classification is a layout
 * concern; consumers should select the appropriate boolean directly.
 *
 * WCAG 2.2 new criteria not covered by this report (UI implementation, not color):
 *   SC 2.4.11 Focus Not Obscured (Minimum) — AA
 *   SC 2.4.12 Focus Not Obscured (Enhanced) — AAA
 *   SC 2.4.13 Focus Appearance — AAA
 *   SC 2.5.7 Dragging Movements — AA
 *   SC 2.5.8 Target Size Minimum (24×24 CSS px) — AA
 *   SC 3.2.6 Consistent Help — A
 *   SC 3.3.7 Redundant Entry — A
 *   SC 3.3.8 Accessible Authentication Minimum — AA
 */
export interface AccessibilityReport {
    /**
     * WCAG 2.1 / 2.2 contrast ratio between dominant (background) and
     * accent (foreground), or dominant vs. itself when no accent exists.
     */
    contrast_ratio: number;
    /** Passes WCAG AA for normal text (contrast_ratio ≥ 4.5:1). SC 1.4.3. */
    is_aa_normal: boolean;
    /** Passes WCAG AAA for normal text (contrast_ratio ≥ 7.0:1). SC 1.4.6. */
    is_aaa_normal: boolean;
    /** Passes WCAG AA for large text (contrast_ratio ≥ 3.0:1). SC 1.4.3. */
    is_aa_large: boolean;
    /** Passes WCAG AAA for large text (contrast_ratio ≥ 4.5:1). SC 1.4.6. */
    is_aaa_large: boolean;
    /**
     * Passes WCAG AA for non-text contrast (contrast_ratio ≥ 3.0:1). SC 1.4.11.
     * Applies to UI components (buttons, inputs, focus rings) and graphical
     * objects (icons, charts, infographics) against their adjacent background.
     */
    is_aa_ui: boolean;
    /** Recommended hex color (#000000 or #FFFFFF) for readability on the dominant background. */
    recommended_font_color: string;
    /** APCA-W3 0.0.98G-4g evaluation (proposed WCAG 3.0 algorithm). */
    apca: ApcaReport;
}

/**
 * Configuration for the analysis pipeline.
 */
export declare class AnalysisOptions {
    /** Number of clusters to extract (2 to 16). */
    max_colors: number;
    /** Sampling density. */
    quality: Quality;
    /** Delta-E threshold for K-Means convergence. */
    convergence: number;

    constructor(max_colors: number, quality: Quality, convergence: number);
    /** Returns an instance with standard balanced settings. */
    static defaults(): AnalysisOptions;
}

/**
 * Pixel sampling density variants.
 */
export enum Quality {
    Draft = 0,
    Balanced = 1,
    Precise = 2,
}

/**
 * Image-level statistical metrics.
 */
export interface ImageStats {
    /** Mean L* lightness (0.0 to 100.0). */
    brightness: number;
    /** Hasler-Suesstrunk colorfulness rating (0.0 to 100.0). */
    colorfulness: number;
    /** Shannon entropy over the L* histogram (higher = more visual complexity). */
    entropy: number;
    /** Classification of the dominant hue group. */
    dominant_hue_group: "warm" | "cool" | "neutral";
    /** Image aspect ratio classification. */
    orientation: "landscape" | "portrait" | "square";
}

/**
 * Perceptual color harmonies generated via LCh hue-rotation from the dominant color.
 */
export interface ColorTheory {
    /** 180° rotation from base hue. */
    complementary: string;
    /** 120° and 240° rotations from base hue. */
    triadic: [string, string];
    /** −30° and +30° rotations from base hue. */
    analogous: [string, string];
}

/**
 * Asynchronously analyzes an image buffer and returns a detailed perceptual report.
 *
 * @param {Uint8Array} data - Raw PNG, JPEG, or WebP bytes.
 * @param {AnalysisOptions} [options] - Configuration for max_colors, quality, and convergence.
 * @returns {Promise<AnalysisReport>}
 *
 * @example
 * const bytes = new Uint8Array(await (await fetch('image.jpg')).arrayBuffer());
 * const report = await analyze(bytes);
 * console.log('WCAG ratio:', report.accessibility.contrast_ratio);
 * console.log('APCA Lc:', report.accessibility.apca.lc);
 */
export function analyze(data: Uint8Array, options?: AnalysisOptions): Promise<AnalysisReport>;