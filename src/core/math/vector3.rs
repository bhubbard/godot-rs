use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const INF: Self = Self {
        x: f32::INFINITY,
        y: f32::INFINITY,
        z: f32::INFINITY,
    };
    pub const UP: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Self = Self { x: 0.0, y: -1.0, z: 0.0 };
    pub const LEFT: Self = Self { x: -1.0, y: 0.0, z: 0.0 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const FORWARD: Self = Self { x: 0.0, y: 0.0, z: -1.0 };
    pub const BACK: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(val: f32) -> Self {
        Self { x: val, y: val, z: val }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
            Self::new(self.x / l, self.y / l, self.z / l)
        }
    }

    #[inline]
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 0.0001
    }

    #[inline]
    pub fn is_equal_approx(self, to: Self) -> bool {
        (self.x - to.x).abs() <= 0.00001
            && (self.y - to.y).abs() <= 0.00001
            && (self.z - to.z).abs() <= 0.00001
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
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
    pub fn angle_to(self, to: Self) -> f32 {
        (self.cross(to).length()).atan2(self.dot(to))
    }

    #[inline]
    pub fn signed_angle_to(self, to: Self, axis: Self) -> f32 {
        let cross_to = self.cross(to);
        let unsigned_angle = (cross_to.length()).atan2(self.dot(to));
        let sign = cross_to.dot(axis);
        if sign < 0.0 {
            -unsigned_angle
        } else {
            unsigned_angle
        }
    }

    #[inline]
    pub fn dot(self, with: Self) -> f32 {
        self.x * with.x + self.y * with.y + self.z * with.z
    }

    #[inline]
    pub fn cross(self, with: Self) -> Self {
        Self::new(
            self.y * with.z - self.z * with.y,
            self.z * with.x - self.x * with.z,
            self.x * with.y - self.y * with.x,
        )
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    pub fn sign(self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.z.signum())
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
            self.z + (to.z - self.z) * weight,
        )
    }

    pub fn slerp(self, to: Self, weight: f32) -> Self {
        let start_len_sq = self.length_squared();
        let end_len_sq = to.length_squared();
        if start_len_sq == 0.0 || end_len_sq == 0.0 {
            return self.lerp(to, weight);
        }
        let axis = self.cross(to).normalized();
        if axis.length_squared() == 0.0 {
            return self.lerp(to, weight);
        }
        let angle = self.angle_to(to);
        let q = crate::core::math::Quaternion::from_axis_angle(axis, angle * weight);
        let start_len = start_len_sq.sqrt();
        let result_len = (1.0 - weight) * start_len + weight * end_len_sq.sqrt();
        q.xform(self.normalized()) * result_len
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
            if step.z != 0.0 {
                (self.z / step.z + 0.5).floor() * step.z
            } else {
                self.z
            },
        )
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
    }

    #[inline]
    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }

    #[inline]
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    #[inline]
    pub fn min_axis_index(self) -> usize {
        let mut min_i = 0;
        let mut min_val = self.x;
        if self.y < min_val {
            min_val = self.y;
            min_i = 1;
        }
        if self.z < min_val {
            min_i = 2;
        }
        min_i
    }

    #[inline]
    pub fn max_axis_index(self) -> usize {
        let mut max_i = 0;
        let mut max_val = self.x;
        if self.y > max_val {
            max_val = self.y;
            max_i = 1;
        }
        if self.z > max_val {
            max_i = 2;
        }
        max_i
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

impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vector3) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<f32> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Vector3) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign<f32> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3i {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub const UP: Self = Self { x: 0, y: 1, z: 0 };
    pub const DOWN: Self = Self { x: 0, y: -1, z: 0 };
    pub const LEFT: Self = Self { x: -1, y: 0, z: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0, z: 0 };
    pub const FORWARD: Self = Self { x: 0, y: 0, z: -1 };
    pub const BACK: Self = Self { x: 0, y: 0, z: 1 };

    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn as_vec3(self) -> Vector3 {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    pub fn min_axis_index(self) -> usize {
        let mut min_i = 0;
        let mut min_val = self.x;
        if self.y < min_val {
            min_val = self.y;
            min_i = 1;
        }
        if self.z < min_val {
            min_i = 2;
        }
        min_i
    }

    #[inline]
    pub fn max_axis_index(self) -> usize {
        let mut max_i = 0;
        let mut max_val = self.x;
        if self.y > max_val {
            max_val = self.y;
            max_i = 1;
        }
        if self.z > max_val {
            max_i = 2;
        }
        max_i
    }
}

impl Index<usize> for Vector3i {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3i"),
        }
    }
}

impl IndexMut<usize> for Vector3i {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3i"),
        }
    }
}

impl Add for Vector3i {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3i {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<i32> for Vector3i {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::fmt::Display for Vector3i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3i({}, {}, {})", self.x, self.y, self.z)
    }
}
