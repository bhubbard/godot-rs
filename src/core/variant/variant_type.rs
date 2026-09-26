use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VariantType {
    Nil,
    Bool,
    Int,
    Float,
    String,
    Vector2,
    Vector2i,
    Rect2,
    Rect2i,
    Vector3,
    Vector3i,
    Transform2D,
    Vector4,
    Vector4i,
    Plane,
    Quaternion,
    Aabb,
    Basis,
    Transform3D,
    Color,
    StringName,
    NodePath,
    Rid,
    ObjectId,
    Callable,
    Signal,
    Dictionary,
    Array,
    PackedByteArray,
    PackedInt32Array,
    PackedInt64Array,
    PackedFloat32Array,
    PackedFloat64Array,
    PackedStringArray,
    PackedVector2Array,
    PackedVector3Array,
    PackedColorArray,
}

impl std::fmt::Display for VariantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
