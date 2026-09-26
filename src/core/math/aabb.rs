use super::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Aabb {
    pub position: Vector3,
    pub size: Vector3,
}

impl Aabb {
    pub const ZERO: Self = Self {
        position: Vector3::ZERO,
        size: Vector3::ZERO,
    };

    #[inline]
    pub const fn new(position: Vector3, size: Vector3) -> Self {
        Self { position, size }
    }

    #[inline]
    pub fn end(&self) -> Vector3 {
        self.position + self.size
    }

    #[inline]
    pub fn volume(&self) -> f32 {
        self.size.x * self.size.y * self.size.z
    }

    #[inline]
    pub fn has_point(&self, point: Vector3) -> bool {
        if point.x < self.position.x || point.y < self.position.y || point.z < self.position.z {
            return false;
        }
        let end = self.end();
        if point.x > end.x || point.y > end.y || point.z > end.z {
            return false;
        }
        true
    }

    #[inline]
    pub fn intersects(&self, b: Self) -> bool {
        let end_a = self.end();
        let end_b = b.end();
        if self.position.x >= end_b.x || end_a.x <= b.position.x {
            return false;
        }
        if self.position.y >= end_b.y || end_a.y <= b.position.y {
            return false;
        }
        if self.position.z >= end_b.z || end_a.z <= b.position.z {
            return false;
        }
        true
    }

    pub fn merge(&self, b: Self) -> Self {
        let new_pos = Vector3::new(
            self.position.x.min(b.position.x),
            self.position.y.min(b.position.y),
            self.position.z.min(b.position.z),
        );
        let new_end = Vector3::new(
            self.end().x.max(b.end().x),
            self.end().y.max(b.end().y),
            self.end().z.max(b.end().z),
        );
        Self::new(new_pos, new_end - new_pos)
    }

    pub fn grow(&self, amount: f32) -> Self {
        Self::new(
            self.position - Vector3::splat(amount),
            self.size + Vector3::splat(amount * 2.0),
        )
    }
}
