use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    #[inline]
    pub fn is_equal_approx(self, other: Self) -> bool {
        (self.r - other.r).abs() <= 0.00001
            && (self.g - other.g).abs() <= 0.00001
            && (self.b - other.b).abs() <= 0.00001
            && (self.a - other.a).abs() <= 0.00001
    }

    #[inline]
    pub fn inverted(self) -> Self {
        Self::new(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, self.a)
    }

    #[inline]
    pub fn darkened(self, amount: f32) -> Self {
        Self::new(
            self.r * (1.0 - amount),
            self.g * (1.0 - amount),
            self.b * (1.0 - amount),
            self.a,
        )
    }

    #[inline]
    pub fn lightened(self, amount: f32) -> Self {
        Self::new(
            self.r + (1.0 - self.r) * amount,
            self.g + (1.0 - self.g) * amount,
            self.b + (1.0 - self.b) * amount,
            self.a,
        )
    }

    #[inline]
    pub fn blend(self, over: Self) -> Self {
        let res_a = self.a + over.a * (1.0 - self.a);
        if res_a <= 0.0 {
            Self::TRANSPARENT
        } else {
            Self::new(
                (self.r * self.a + over.r * over.a * (1.0 - self.a)) / res_a,
                (self.g * self.a + over.g * over.a * (1.0 - self.a)) / res_a,
                (self.b * self.a + over.b * over.a * (1.0 - self.a)) / res_a,
                res_a,
            )
        }
    }

    #[inline]
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    #[inline]
    pub fn to_rgba8(self) -> (u8, u8, u8, u8) {
        (
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        )
    }

    #[inline]
    pub fn to_html_hex(self) -> String {
        let (r, g, b, a) = self.to_rgba8();
        if a == 255 {
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else {
            format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
        }
    }

    pub fn from_html(html: &str) -> Option<Self> {
        let hex = html.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::from_rgba8(r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::from_rgba8(r, g, b, a))
            }
            _ => None,
        }
    }

    #[inline]
    pub fn lerp(self, to: Self, weight: f32) -> Self {
        Self::new(
            self.r + (to.r - self.r) * weight,
            self.g + (to.g - self.g) * weight,
            self.b + (to.b - self.b) * weight,
            self.a + (to.a - self.a) * weight,
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        )
    }
}
