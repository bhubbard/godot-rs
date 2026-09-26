use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const INF: Self = Self {
        x: f32::INFINITY,
        y: f32::INFINITY,
    };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0 };
    pub const UP: Self = Self { x: 0.0, y: -1.0 };
    pub const DOWN: Self = Self { x: 0.0, y: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(val: f32) -> Self {
        Self { x: val, y: val }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let l = self.length();
        if l == 0.0 {
            Self::ZERO
        } else {
            Self::new(self.x / l, self.y / l)
        }
    }

    #[inline]
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 0.0001
    }

    #[inline]
    pub fn distance_squared_to(self, to: Self) -> f32 {
        (to - self).length_squared()
    }

    #[inline]
    pub fn distance_to(self, to: Self) -> f32 {
        (to - self).length()
    }

    #[inline]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn angle_to(self, to: Self) -> f32 {
        self.cross(to).atan2(self.dot(to))
    }

    #[inline]
    pub fn angle_to_point(self, to: Self) -> f32 {
        (to.y - self.y).atan2(to.x - self.x)
    }

    #[inline]
    pub fn dot(self, with: Self) -> f32 {
        self.x * with.x + self.y * with.y
    }

    #[inline]
    pub fn cross(self, with: Self) -> f32 {
        self.x * with.y - self.y * with.x
    }

    #[inline]
    pub fn sign(self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn rotated(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    #[inline]
    pub fn lerp(self, to: Self, weight: f32) -> Self {
        Self::new(
            self.x + (to.x - self.x) * weight,
            self.y + (to.y - self.y) * weight,
        )
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
        )
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    #[inline]
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    #[inline]
    pub fn move_toward(self, to: Self, delta: f32) -> Self {
        let diff = to - self;
        let len = diff.length();
        if len <= delta || len < 0.00001 {
            to
        } else {
            self + diff / len * delta
        }
    }
}

impl Add for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vector2) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vector2) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn as_vec2(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

impl Add for Vector2i {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2i {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::fmt::Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2i({}, {})", self.x, self.y)
    }
}
