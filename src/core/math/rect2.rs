use super::{Vector2, Vector2i};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Rect2 {
    pub position: Vector2,
    pub size: Vector2,
}

impl Rect2 {
    pub const ZERO: Self = Self {
        position: Vector2::ZERO,
        size: Vector2::ZERO,
    };

    #[inline]
    pub const fn new(position: Vector2, size: Vector2) -> Self {
        Self { position, size }
    }

    #[inline]
    pub const fn from_components(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
        }
    }

    #[inline]
    pub fn end(self) -> Vector2 {
        self.position + self.size
    }

    #[inline]
    pub fn area(self) -> f32 {
        self.size.x * self.size.y
    }

    #[inline]
    pub fn has_point(self, point: Vector2) -> bool {
        if point.x < self.position.x {
            return false;
        }
        if point.y < self.position.y {
            return false;
        }
        if point.x >= self.position.x + self.size.x {
            return false;
        }
        if point.y >= self.position.y + self.size.y {
            return false;
        }
        true
    }

    #[inline]
    pub fn intersects(self, b: Self) -> bool {
        if self.position.x >= b.position.x + b.size.x {
            return false;
        }
        if self.position.x + self.size.x <= b.position.x {
            return false;
        }
        if self.position.y >= b.position.y + b.size.y {
            return false;
        }
        if self.position.y + self.size.y <= b.position.y {
            return false;
        }
        true
    }

    #[inline]
    pub fn intersection(self, b: Self) -> Option<Self> {
        let new_pos = Vector2::new(
            self.position.x.max(b.position.x),
            self.position.y.max(b.position.y),
        );
        let new_end = Vector2::new(
            (self.position.x + self.size.x).min(b.position.x + b.size.x),
            (self.position.y + self.size.y).min(b.position.y + b.size.y),
        );

        if new_pos.x < new_end.x && new_pos.y < new_end.y {
            Some(Self::new(new_pos, new_end - new_pos))
        } else {
            None
        }
    }

    #[inline]
    pub fn merge(self, b: Self) -> Self {
        let new_pos = Vector2::new(
            self.position.x.min(b.position.x),
            self.position.y.min(b.position.y),
        );
        let new_end = Vector2::new(
            (self.position.x + self.size.x).max(b.position.x + b.size.x),
            (self.position.y + self.size.y).max(b.position.y + b.size.y),
        );
        Self::new(new_pos, new_end - new_pos)
    }

    #[inline]
    pub fn grow(self, amount: f32) -> Self {
        Self::new(
            self.position - Vector2::splat(amount),
            self.size + Vector2::splat(amount * 2.0),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Rect2i {
    pub position: Vector2i,
    pub size: Vector2i,
}

impl Rect2i {
    #[inline]
    pub const fn new(position: Vector2i, size: Vector2i) -> Self {
        Self { position, size }
    }

    #[inline]
    pub fn as_rect2(self) -> Rect2 {
        Rect2::new(self.position.as_vec2(), self.size.as_vec2())
    }
}
