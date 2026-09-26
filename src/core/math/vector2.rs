use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

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
    pub fn is_equal_approx(self, to: Self) -> bool {
        (self.x - to.x).abs() <= 0.00001 && (self.y - to.y).abs() <= 0.00001
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
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
    pub fn orthogonal(self) -> Self {
        Self::new(self.y, -self.x)
    }

    #[inline]
    pub fn aspect(self) -> f32 {
        self.x / self.y
    }

    #[inline]
    pub fn project(self, b: Self) -> Self {
        let len_sq = b.length_squared();
        if len_sq == 0.0 {
            Self::ZERO
        } else {
            b * (self.dot(b) / len_sq)
        }
    }

    #[inline]
    pub fn slide(self, normal: Self) -> Self {
        self - normal * self.dot(normal)
    }

    #[inline]
    pub fn bounce(self, normal: Self) -> Self {
        -self.reflect(normal)
    }

    #[inline]
    pub fn reflect(self, normal: Self) -> Self {
        normal * (2.0 * self.dot(normal)) - self
    }

    #[inline]
    pub fn lerp(self, to: Self, weight: f32) -> Self {
        Self::new(
            self.x + (to.x - self.x) * weight,
            self.y + (to.y - self.y) * weight,
        )
    }

    pub fn slerp(self, to: Self, weight: f32) -> Self {
        let start_len_sq = self.length_squared();
        let end_len_sq = to.length_squared();
        if start_len_sq == 0.0 || end_len_sq == 0.0 {
            return self.lerp(to, weight);
        }
        let start_len = start_len_sq.sqrt();
        let result_len = (1.0 - weight) * start_len + weight * end_len_sq.sqrt();
        let angle = self.angle_to(to);
        self.rotated(angle * weight) * (result_len / start_len)
    }

    pub fn cubic_interpolate(self, b: Self, pre_a: Self, post_b: Self, weight: f32) -> Self {
        let p0 = pre_a;
        let p1 = self;
        let p2 = b;
        let p3 = post_b;

        let t = weight;
        let t2 = t * t;
        let t3 = t2 * t;

        ((p1 * 2.0)
            + (-p0 + p2) * t
            + (p0 * 2.0 - p1 * 5.0 + p2 * 4.0 - p3) * t2
            + (-p0 + p1 * 3.0 - p2 * 3.0 + p3) * t3)
            * 0.5
    }

    #[inline]
    pub fn snapped(self, step: Self) -> Self {
        Self::new(
            if step.x != 0.0 {
                (self.x / step.x + 0.5).floor() * step.x
            } else {
                self.x
            },
            if step.y != 0.0 {
                (self.y / step.y + 0.5).floor() * step.y
            } else {
                self.y
            },
        )
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
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
    pub fn min_axis_index(self) -> usize {
        if self.x < self.y {
            0
        } else {
            1
        }
    }

    #[inline]
    pub fn max_axis_index(self) -> usize {
        if self.x > self.y {
            0
        } else {
            1
        }
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

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vector2"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Vector2"),
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
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn as_vec2(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn min_axis_index(self) -> usize {
        if self.x < self.y {
            0
        } else {
            1
        }
    }

    #[inline]
    pub fn max_axis_index(self) -> usize {
        if self.x > self.y {
            0
        } else {
            1
        }
    }
}

impl Index<usize> for Vector2i {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vector2i"),
        }
    }
}

impl IndexMut<usize> for Vector2i {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Vector2i"),
        }
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

impl Mul<i32> for Vector2i {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl std::fmt::Display for Vector2i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2i({}, {})", self.x, self.y)
    }
}
