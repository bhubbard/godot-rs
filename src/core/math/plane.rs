use super::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vector3,
    pub d: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            normal: Vector3::UP,
            d: 0.0,
        }
    }
}

impl Plane {
    #[inline]
    pub const fn new(normal: Vector3, d: f32) -> Self {
        Self { normal, d }
    }

    #[inline]
    pub fn from_point_normal(point: Vector3, normal: Vector3) -> Self {
        let n = normal.normalized();
        Self {
            normal: n,
            d: n.dot(point),
        }
    }

    #[inline]
    pub fn distance_to(&self, point: Vector3) -> f32 {
        self.normal.dot(point) - self.d
    }

    #[inline]
    pub fn is_point_over(&self, point: Vector3) -> bool {
        self.distance_to(point) > 0.0
    }

    #[inline]
    pub fn project(&self, point: Vector3) -> Vector3 {
        point - self.normal * self.distance_to(point)
    }
}
