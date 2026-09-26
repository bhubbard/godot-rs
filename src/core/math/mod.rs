pub mod aabb;
pub mod basis;
pub mod color;
pub mod plane;
pub mod quaternion;
pub mod rect2;
pub mod transform2d;
pub mod transform3d;
pub mod vector2;
pub mod vector3;
pub mod vector4;

pub use aabb::Aabb;
pub use basis::Basis;
pub use color::Color;
pub use plane::Plane;
pub use quaternion::Quaternion;
pub use rect2::{Rect2, Rect2i};
pub use transform2d::Transform2D;
pub use transform3d::Transform3D;
pub use vector2::{Vector2, Vector2i};
pub use vector3::{Vector3, Vector3i};
pub use vector4::{Vector4, Vector4i};

pub const CMP_EPSILON: f32 = 0.00001;

pub fn is_equal_approx(a: f32, b: f32) -> bool {
    (a - b).abs() <= CMP_EPSILON
}

pub fn lerp(from: f32, to: f32, weight: f32) -> f32 {
    from + (to - from) * weight
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
