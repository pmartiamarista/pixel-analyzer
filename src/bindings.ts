/**
 * The final result of the image analysis pipeline.
 */
export interface AnalysisReport {
    /** Main palette containing dominant and accent colors. */
    main: MainPalette;
    /** Categorized palettes (vibrant, muted, light, dark, raw). */
    palettes: Palettes;
    /** WCAG 2.1 accessibility evaluation. */
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
    /** A perceptually distinct accent color (DeltaE >= 5 from dominant). */
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
    /** High chroma / intense colors. */
    vibrant: ColorEntry[];
    /** Low chroma / desaturated colors. */
    muted: ColorEntry[];
    /** High lightness colors. */
    light: ColorEntry[];
    /** Low lightness colors. */
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
 * WCAG 2.1 contrast ratio and accessibility evaluation.
 */
export interface AccessibilityReport {
    /** Contrast ratio between dominant and accent (or dominant and white if no accent). */
    contrast_ratio: number;
    /** Passes WCAG AA for normal text (>= 4.5:1). */
    is_aa_normal: boolean;
    /** Passes WCAG AAA for normal text (>= 7.0:1). */
    is_aaa_normal: boolean;
    /** Recommended hex color for font readability on this color (#000000 or #FFFFFF). */
    recommended_font_color: string;
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
    /** Mean lightness (0.0 to 100.0). */
    brightness: number;
    /** Hasler-Suesstrunk colorfulness rating. */
    colorfulness: number;
    /** Shannon entropy over the L* histogram (higher denotes visual complexity). */
    entropy: number;
    /** Classification of the dominant hue group. */
    dominant_hue_group: "warm" | "cool" | "neutral";
    /** Image aspect ratio classification. */
    orientation: "landscape" | "portrait" | "square";
}

/**
 * Perceptual color harmonies generated via hue-rotation.
 */
export interface ColorTheory {
    /** 180° rotation from base hue. */
    complementary: string;
    /** 120° and 240° rotations from base hue. */
    triadic: [string, string];
    /** -30° and +30° rotations from base hue. */
    analogous: [string, string];
}

/**
 * Asynchronously analyzes an image buffer and returns a detailed perceptual report.
 * 
 * @param {Uint8Array} data - Raw PNG, JPEG, or WebP bytes.
 * @param {AnalysisOptions} options - Configuration for max_colors, quality, and convergence.
 * @returns {Promise<AnalysisReport>}
 */
export function analyze(data: Uint8Array, options?: AnalysisOptions): Promise<AnalysisReport>;
