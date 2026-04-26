use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HueGroup {
    Warm,
    Cool,
    Neutral,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl RgbColor {
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_hex(hex: &str) -> Self {
        let h = hex.trim_start_matches('#');
        if h.len() == 6
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&h[0..2], 16),
                u8::from_str_radix(&h[2..4], 16),
                u8::from_str_radix(&h[4..6], 16),
            )
        {
            return RgbColor { r, g, b };
        }
        RgbColor { r: 0, g: 0, b: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LabColor {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl LabColor {
    #[inline]
    pub fn distance_sq(self, other: LabColor) -> f32 {
        let dl = self.l - other.l;
        let da = self.a - other.a;
        let db = self.b - other.b;
        dl * dl + da * da + db * db
    }

    #[inline]
    pub fn delta_e(self, other: LabColor) -> f32 {
        self.distance_sq(other).sqrt()
    }

    #[inline]
    pub fn is_dark(self) -> bool {
        self.l < 50.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LchColor {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ColorEntry {
    pub hex: String,
    pub population: f32,
    pub is_dark: bool,
    pub lab: LabValues,
    pub lch: LchValues,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabValues {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LchValues {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub centroid: LabColor,
    pub pixel_count: usize,
    pub total_pixels: usize,
}

impl Cluster {
    pub fn population(&self) -> f32 {
        if self.total_pixels == 0 {
            return 0.0;
        }
        self.pixel_count as f32 / self.total_pixels as f32
    }
}
