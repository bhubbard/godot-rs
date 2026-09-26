use super::Vector3;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quaternion {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();
        let norm_axis = axis.normalized();
        Self::new(
            norm_axis.x * s,
            norm_axis.y * s,
            norm_axis.z * s,
            half_angle.cos(),
        )
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::IDENTITY
        } else {
            Self::new(
                self.x / len,
                self.y / len,
                self.z / len,
                self.w / len,
            )
        }
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let len_sq = self.length_squared();
        if len_sq == 0.0 {
            Self::IDENTITY
        } else {
            Self::new(
                -self.x / len_sq,
                -self.y / len_sq,
                -self.z / len_sq,
                self.w / len_sq,
            )
        }
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn slerp(&self, to: Self, weight: f32) -> Self {
        let mut to1 = to;
        let mut cos_om = self.dot(to);

        if cos_om < 0.0 {
            cos_om = -cos_om;
            to1 = Self::new(-to.x, -to.y, -to.z, -to.w);
        }

        let (scale0, scale1) = if (1.0 - cos_om) > 0.0001 {
            let omega = cos_om.acos();
            let sin_om = omega.sin();
            (
                ((1.0 - weight) * omega).sin() / sin_om,
                (weight * omega).sin() / sin_om,
            )
        } else {
            (1.0 - weight, weight)
        };

        Self::new(
            scale0 * self.x + scale1 * to1.x,
            scale0 * self.y + scale1 * to1.y,
            scale0 * self.z + scale1 * to1.z,
            scale0 * self.w + scale1 * to1.w,
        )
    }

    pub fn xform(&self, v: Vector3) -> Vector3 {
        let qv = Vector3::new(self.x, self.y, self.z);
        let uv = qv.cross(v);
        let uuv = qv.cross(uv);
        v + ((uv * self.w) + uuv) * 2.0
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}
