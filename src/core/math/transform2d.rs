use super::Vector2;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    pub x: Vector2,
    pub y: Vector2,
    pub origin: Vector2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform2D {
    pub const IDENTITY: Self = Self {
        x: Vector2::new(1.0, 0.0),
        y: Vector2::new(0.0, 1.0),
        origin: Vector2::new(0.0, 0.0),
    };

    pub const FLIP_X: Self = Self {
        x: Vector2::new(-1.0, 0.0),
        y: Vector2::new(0.0, 1.0),
        origin: Vector2::new(0.0, 0.0),
    };

    pub const FLIP_Y: Self = Self {
        x: Vector2::new(1.0, 0.0),
        y: Vector2::new(0.0, -1.0),
        origin: Vector2::new(0.0, 0.0),
    };

    #[inline]
    pub const fn new(x: Vector2, y: Vector2, origin: Vector2) -> Self {
        Self { x, y, origin }
    }

    #[inline]
    pub fn from_angle_origin(rot: f32, origin: Vector2) -> Self {
        let (cr, sr) = (rot.cos(), rot.sin());
        Self {
            x: Vector2::new(cr, sr),
            y: Vector2::new(-sr, cr),
            origin,
        }
    }

    #[inline]
    pub fn from_angle_scale_origin(rot: f32, scale: Vector2, origin: Vector2) -> Self {
        let (cr, sr) = (rot.cos(), rot.sin());
        Self {
            x: Vector2::new(cr * scale.x, sr * scale.x),
            y: Vector2::new(-sr * scale.y, cr * scale.y),
            origin,
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.x.x * self.y.y - self.x.y * self.y.x
    }

    #[inline]
    pub fn basis_xform(&self, v: Vector2) -> Vector2 {
        Vector2::new(
            self.x.x * v.x + self.y.x * v.y,
            self.x.y * v.x + self.y.y * v.y,
        )
    }

    #[inline]
    pub fn xform(&self, v: Vector2) -> Vector2 {
        self.basis_xform(v) + self.origin
    }

    #[inline]
    pub fn xform_inv(&self, v: Vector2) -> Vector2 {
        let v_inv = v - self.origin;
        let det = self.determinant();
        if det.abs() < 0.00001 {
            return Vector2::ZERO;
        }
        let inv_det = 1.0 / det;
        Vector2::new(
            (self.y.y * v_inv.x - self.y.x * v_inv.y) * inv_det,
            (-self.x.y * v_inv.x + self.x.x * v_inv.y) * inv_det,
        )
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        if det.abs() < 0.00001 {
            return Self::IDENTITY;
        }
        let inv_det = 1.0 / det;
        let x = Vector2::new(self.y.y * inv_det, -self.x.y * inv_det);
        let y = Vector2::new(-self.y.x * inv_det, self.x.x * inv_det);
        let origin = Vector2::new(
            -(x.x * self.origin.x + y.x * self.origin.y),
            -(x.y * self.origin.x + y.y * self.origin.y),
        );
        Self { x, y, origin }
    }

    #[inline]
    pub fn get_rotation(&self) -> f32 {
        self.x.y.atan2(self.x.x)
    }

    #[inline]
    pub fn get_scale(&self) -> Vector2 {
        let det_sign = if self.determinant() < 0.0 { -1.0 } else { 1.0 };
        Vector2::new(self.x.length(), self.y.length() * det_sign)
    }

    #[inline]
    pub fn rotated(&self, phi: f32) -> Self {
        *self * Self::from_angle_origin(phi, Vector2::ZERO)
    }

    #[inline]
    pub fn scaled(&self, scale: Vector2) -> Self {
        Self {
            x: self.x * scale.x,
            y: self.y * scale.y,
            origin: self.origin,
        }
    }

    #[inline]
    pub fn translated(&self, offset: Vector2) -> Self {
        Self {
            x: self.x,
            y: self.y,
            origin: self.origin + self.basis_xform(offset),
        }
    }
}

impl Mul for Transform2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.basis_xform(rhs.x),
            y: self.basis_xform(rhs.y),
            origin: self.xform(rhs.origin),
        }
    }
}

impl Mul<Vector2> for Transform2D {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.xform(rhs)
    }
}
