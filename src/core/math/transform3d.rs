use super::{Basis, Vector3};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform3D {
    pub basis: Basis,
    pub origin: Vector3,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        basis: Basis::IDENTITY,
        origin: Vector3::ZERO,
    };

    pub const FLIP_X: Self = Self {
        basis: Basis::FLIP_X,
        origin: Vector3::ZERO,
    };

    #[inline]
    pub const fn new(basis: Basis, origin: Vector3) -> Self {
        Self { basis, origin }
    }

    #[inline]
    pub const fn from_origin(origin: Vector3) -> Self {
        Self {
            basis: Basis::IDENTITY,
            origin,
        }
    }

    #[inline]
    pub fn is_equal_approx(&self, b: &Self) -> bool {
        self.basis.is_equal_approx(&b.basis) && self.origin.is_equal_approx(b.origin)
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.basis.is_finite() && self.origin.is_finite()
    }

    #[inline]
    pub fn xform(&self, v: Vector3) -> Vector3 {
        self.basis.xform(v) + self.origin
    }

    #[inline]
    pub fn xform_inv(&self, v: Vector3) -> Vector3 {
        let v_inv = v - self.origin;
        self.basis.transposed().xform(v_inv)
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        self.affine_inverse()
    }

    #[inline]
    pub fn affine_inverse(&self) -> Self {
        let inv_basis = self.basis.inverse();
        let inv_origin = inv_basis.xform(-self.origin);
        Self::new(inv_basis, inv_origin)
    }

    #[inline]
    pub fn orthonormalized(&self) -> Self {
        Self::new(self.basis.orthonormalized(), self.origin)
    }

    #[inline]
    pub fn looking_at(&self, target: Vector3, up: Vector3) -> Self {
        Self::new(
            Basis::looking_at(target - self.origin, up),
            self.origin,
        )
    }

    #[inline]
    pub fn translated(&self, offset: Vector3) -> Self {
        Self::new(self.basis, self.origin + self.basis.xform(offset))
    }

    #[inline]
    pub fn scaled(&self, scale: Vector3) -> Self {
        Self::new(self.basis * Basis::from_scale(scale), self.origin)
    }

    #[inline]
    pub fn rotated(&self, axis: Vector3, angle: f32) -> Self {
        Self::new(self.basis * Basis::from_axis_angle(axis, angle), self.origin)
    }

    pub fn interpolate_with(&self, to: &Self, weight: f32) -> Self {
        let basis = self.basis.slerp(to.basis, weight);
        let origin = self.origin.lerp(to.origin, weight);
        Self::new(basis, origin)
    }
}

impl Mul for Transform3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            origin: self.xform(rhs.origin),
            basis: self.basis * rhs.basis,
        }
    }
}

impl Mul<Vector3> for Transform3D {
    type Output = Vector3;
    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.xform(rhs)
    }
}
