use super::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Basis {
    pub rows: [Vector3; 3],
}

impl Default for Basis {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Basis {
    pub const IDENTITY: Self = Self {
        rows: [
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ],
    };

    pub const FLIP_X: Self = Self {
        rows: [
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ],
    };

    #[inline]
    pub const fn new(row0: Vector3, row1: Vector3, row2: Vector3) -> Self {
        Self {
            rows: [row0, row1, row2],
        }
    }

    #[inline]
    pub fn from_scale(scale: Vector3) -> Self {
        Self {
            rows: [
                Vector3::new(scale.x, 0.0, 0.0),
                Vector3::new(0.0, scale.y, 0.0),
                Vector3::new(0.0, 0.0, scale.z),
            ],
        }
    }

    pub fn from_quaternion(q: Quaternion) -> Self {
        let d = q.length_squared();
        let s = 2.0 / d;
        let xs = q.x * s;
        let ys = q.y * s;
        let zs = q.z * s;
        let wx = q.w * xs;
        let wy = q.w * ys;
        let wz = q.w * zs;
        let xx = q.x * xs;
        let xy = q.x * ys;
        let xz = q.x * zs;
        let yy = q.y * ys;
        let yz = q.y * zs;
        let zz = q.z * zs;

        Self::new(
            Vector3::new(1.0 - (yy + zz), xy - wz, xz + wy),
            Vector3::new(xy + wz, 1.0 - (xx + zz), yz - wx),
            Vector3::new(xz - wy, yz + wx, 1.0 - (xx + yy)),
        )
    }

    pub fn from_euler(euler: Vector3) -> Self {
        let (cx, sx) = (euler.x.cos(), euler.x.sin());
        let (cy, sy) = (euler.y.cos(), euler.y.sin());
        let (cz, sz) = (euler.z.cos(), euler.z.sin());

        Self::new(
            Vector3::new(cy * cz, -cy * sz, sy),
            Vector3::new(sx * sy * cz + cx * sz, -sx * sy * sz + cx * cz, -sx * cy),
            Vector3::new(-cx * sy * cz + sx * sz, cx * sy * sz + sx * cz, cx * cy),
        )
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let q = Quaternion::from_axis_angle(axis, angle);
        Self::from_quaternion(q)
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.rows[0].dot(self.rows[1].cross(self.rows[2]))
    }

    #[inline]
    pub fn transposed(&self) -> Self {
        Self::new(
            Vector3::new(self.rows[0].x, self.rows[1].x, self.rows[2].x),
            Vector3::new(self.rows[0].y, self.rows[1].y, self.rows[2].y),
            Vector3::new(self.rows[0].z, self.rows[1].z, self.rows[2].z),
        )
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        if det.abs() < 0.00001 {
            return Self::IDENTITY;
        }
        let inv_det = 1.0 / det;
        let r0 = self.rows[1].cross(self.rows[2]) * inv_det;
        let r1 = self.rows[2].cross(self.rows[0]) * inv_det;
        let r2 = self.rows[0].cross(self.rows[1]) * inv_det;
        Self::new(
            Vector3::new(r0.x, r1.x, r2.x),
            Vector3::new(r0.y, r1.y, r2.y),
            Vector3::new(r0.z, r1.z, r2.z),
        )
    }

    #[inline]
    pub fn xform(&self, v: Vector3) -> Vector3 {
        Vector3::new(
            self.rows[0].dot(v),
            self.rows[1].dot(v),
            self.rows[2].dot(v),
        )
    }

    #[inline]
    pub fn scaled(&self, scale: Vector3) -> Self {
        *self * Self::from_scale(scale)
    }

    #[inline]
    pub fn get_scale(&self) -> Vector3 {
        let det_sign = if self.determinant() < 0.0 { -1.0 } else { 1.0 };
        Vector3::new(
            Vector3::new(self.rows[0].x, self.rows[1].x, self.rows[2].x).length(),
            Vector3::new(self.rows[0].y, self.rows[1].y, self.rows[2].y).length(),
            Vector3::new(self.rows[0].z, self.rows[1].z, self.rows[2].z).length() * det_sign,
        )
    }

    pub fn looking_at(target: Vector3, up: Vector3) -> Self {
        let v_z = -target.normalized();
        let v_x = up.cross(v_z).normalized();
        let v_y = v_z.cross(v_x);
        Self::new(
            Vector3::new(v_x.x, v_y.x, v_z.x),
            Vector3::new(v_x.y, v_y.y, v_z.y),
            Vector3::new(v_x.z, v_y.z, v_z.z),
        )
    }
}

impl Mul for Basis {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let tr = rhs.transposed();
        Self::new(
            Vector3::new(self.rows[0].dot(tr.rows[0]), self.rows[0].dot(tr.rows[1]), self.rows[0].dot(tr.rows[2])),
            Vector3::new(self.rows[1].dot(tr.rows[0]), self.rows[1].dot(tr.rows[1]), self.rows[1].dot(tr.rows[2])),
            Vector3::new(self.rows[2].dot(tr.rows[0]), self.rows[2].dot(tr.rows[1]), self.rows[2].dot(tr.rows[2])),
        )
    }
}

impl Mul<Vector3> for Basis {
    type Output = Vector3;
    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.xform(rhs)
    }
}
