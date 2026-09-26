pub mod callable;
pub mod signal;
pub mod variant_type;

pub use callable::Callable;
pub use signal::Signal;
pub use variant_type::VariantType;

use crate::core::math::{
    Aabb, Basis, Color, Plane, Quaternion, Rect2, Rect2i, Transform2D, Transform3D, Vector2,
    Vector2i, Vector3, Vector3i, Vector4, Vector4i,
};
use crate::core::string::{NodePath, StringName};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Variant {
    #[default]
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vector2(Vector2),
    Vector2i(Vector2i),
    Rect2(Rect2),
    Rect2i(Rect2i),
    Vector3(Vector3),
    Vector3i(Vector3i),
    Transform2D(Transform2D),
    Vector4(Vector4),
    Vector4i(Vector4i),
    Plane(Plane),
    Quaternion(Quaternion),
    Aabb(Aabb),
    Basis(Basis),
    Transform3D(Transform3D),
    Color(Color),
    StringName(StringName),
    NodePath(NodePath),
    Rid(u64),
    ObjectId(u64),
    Callable(Callable),
    Signal(Signal),
    Dictionary(IndexMap<String, Variant>),
    Array(Vec<Variant>),
    PackedByteArray(Vec<u8>),
    PackedInt32Array(Vec<i32>),
    PackedInt64Array(Vec<i64>),
    PackedFloat32Array(Vec<f32>),
    PackedFloat64Array(Vec<f64>),
    PackedStringArray(Vec<String>),
    PackedVector2Array(Vec<Vector2>),
    PackedVector3Array(Vec<Vector3>),
    PackedColorArray(Vec<Color>),
}

impl Variant {
    pub fn get_type(&self) -> VariantType {
        match self {
            Self::Nil => VariantType::Nil,
            Self::Bool(_) => VariantType::Bool,
            Self::Int(_) => VariantType::Int,
            Self::Float(_) => VariantType::Float,
            Self::String(_) => VariantType::String,
            Self::Vector2(_) => VariantType::Vector2,
            Self::Vector2i(_) => VariantType::Vector2i,
            Self::Rect2(_) => VariantType::Rect2,
            Self::Rect2i(_) => VariantType::Rect2i,
            Self::Vector3(_) => VariantType::Vector3,
            Self::Vector3i(_) => VariantType::Vector3i,
            Self::Transform2D(_) => VariantType::Transform2D,
            Self::Vector4(_) => VariantType::Vector4,
            Self::Vector4i(_) => VariantType::Vector4i,
            Self::Plane(_) => VariantType::Plane,
            Self::Quaternion(_) => VariantType::Quaternion,
            Self::Aabb(_) => VariantType::Aabb,
            Self::Basis(_) => VariantType::Basis,
            Self::Transform3D(_) => VariantType::Transform3D,
            Self::Color(_) => VariantType::Color,
            Self::StringName(_) => VariantType::StringName,
            Self::NodePath(_) => VariantType::NodePath,
            Self::Rid(_) => VariantType::Rid,
            Self::ObjectId(_) => VariantType::ObjectId,
            Self::Callable(_) => VariantType::Callable,
            Self::Signal(_) => VariantType::Signal,
            Self::Dictionary(_) => VariantType::Dictionary,
            Self::Array(_) => VariantType::Array,
            Self::PackedByteArray(_) => VariantType::PackedByteArray,
            Self::PackedInt32Array(_) => VariantType::PackedInt32Array,
            Self::PackedInt64Array(_) => VariantType::PackedInt64Array,
            Self::PackedFloat32Array(_) => VariantType::PackedFloat32Array,
            Self::PackedFloat64Array(_) => VariantType::PackedFloat64Array,
            Self::PackedStringArray(_) => VariantType::PackedStringArray,
            Self::PackedVector2Array(_) => VariantType::PackedVector2Array,
            Self::PackedVector3Array(_) => VariantType::PackedVector3Array,
            Self::PackedColorArray(_) => VariantType::PackedColorArray,
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(b) => *b,
            Self::Int(i) => *i != 0,
            Self::Float(f) => *f != 0.0 && !f.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::Array(a) => !a.is_empty(),
            Self::Dictionary(d) => !d.is_empty(),
            _ => true,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            Self::Float(f) => Some(*f as i64),
            Self::Bool(b) => Some(if *b { 1 } else { 0 }),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            Self::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Self::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn to_vector2(&self) -> Option<Vector2> {
        match self {
            Self::Vector2(v) => Some(*v),
            Self::Vector2i(v) => Some(v.as_vec2()),
            _ => None,
        }
    }

    pub fn to_vector3(&self) -> Option<Vector3> {
        match self {
            Self::Vector3(v) => Some(*v),
            Self::Vector3i(v) => Some(v.as_vec3()),
            _ => None,
        }
    }

    pub fn to_color(&self) -> Option<Color> {
        match self {
            Self::Color(c) => Some(*c),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            Self::StringName(sn) => Some(sn.as_str()),
            Self::NodePath(np) => Some(np.as_str()),
            _ => None,
        }
    }
}

// Convenient From implementations
impl From<bool> for Variant {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i32> for Variant {
    fn from(v: i32) -> Self {
        Self::Int(v as i64)
    }
}

impl From<i64> for Variant {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl From<f32> for Variant {
    fn from(v: f32) -> Self {
        Self::Float(v as f64)
    }
}

impl From<f64> for Variant {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<&str> for Variant {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<String> for Variant {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<Vector2> for Variant {
    fn from(v: Vector2) -> Self {
        Self::Vector2(v)
    }
}

impl From<Vector3> for Variant {
    fn from(v: Vector3) -> Self {
        Self::Vector3(v)
    }
}

impl From<Color> for Variant {
    fn from(v: Color) -> Self {
        Self::Color(v)
    }
}

impl From<Transform2D> for Variant {
    fn from(v: Transform2D) -> Self {
        Self::Transform2D(v)
    }
}

impl From<Transform3D> for Variant {
    fn from(v: Transform3D) -> Self {
        Self::Transform3D(v)
    }
}

impl From<Rect2> for Variant {
    fn from(v: Rect2) -> Self {
        Self::Rect2(v)
    }
}

impl From<StringName> for Variant {
    fn from(v: StringName) -> Self {
        Self::StringName(v)
    }
}

impl From<NodePath> for Variant {
    fn from(v: NodePath) -> Self {
        Self::NodePath(v)
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "<null>"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Vector2(v) => write!(f, "Vector2({}, {})", v.x, v.y),
            Self::Vector2i(v) => write!(f, "Vector2i({}, {})", v.x, v.y),
            Self::Rect2(r) => write!(f, "Rect2({}, {}, {}, {})", r.position.x, r.position.y, r.size.x, r.size.y),
            Self::Rect2i(r) => write!(f, "Rect2i({}, {}, {}, {})", r.position.x, r.position.y, r.size.x, r.size.y),
            Self::Vector3(v) => write!(f, "Vector3({}, {}, {})", v.x, v.y, v.z),
            Self::Vector3i(v) => write!(f, "Vector3i({}, {}, {})", v.x, v.y, v.z),
            Self::Transform2D(_) => write!(f, "[Transform2D]"),
            Self::Vector4(v) => write!(f, "Vector4({}, {}, {}, {})", v.x, v.y, v.z, v.w),
            Self::Vector4i(v) => write!(f, "Vector4i({}, {}, {}, {})", v.x, v.y, v.z, v.w),
            Self::Plane(p) => write!(f, "Plane({}, {}, {}, {})", p.normal.x, p.normal.y, p.normal.z, p.d),
            Self::Quaternion(q) => write!(f, "Quaternion({}, {}, {}, {})", q.x, q.y, q.z, q.w),
            Self::Aabb(a) => write!(f, "AABB({}, {})", a.position, a.size),
            Self::Basis(_) => write!(f, "[Basis]"),
            Self::Transform3D(_) => write!(f, "[Transform3D]"),
            Self::Color(c) => write!(f, "Color({}, {}, {}, {})", c.r, c.g, c.b, c.a),
            Self::StringName(sn) => write!(f, "&\"{}\"", sn.as_str()),
            Self::NodePath(np) => write!(f, "^{}", np.as_str()),
            Self::Rid(rid) => write!(f, "RID({})", rid),
            Self::ObjectId(oid) => write!(f, "ObjectID({})", oid),
            Self::Callable(c) => write!(f, "Callable({})", c),
            Self::Signal(s) => write!(f, "Signal({})", s),
            Self::Dictionary(d) => write!(f, "Dictionary(len={})", d.len()),
            Self::Array(a) => write!(f, "Array(len={})", a.len()),
            Self::PackedByteArray(a) => write!(f, "PackedByteArray(len={})", a.len()),
            Self::PackedInt32Array(a) => write!(f, "PackedInt32Array(len={})", a.len()),
            Self::PackedInt64Array(a) => write!(f, "PackedInt64Array(len={})", a.len()),
            Self::PackedFloat32Array(a) => write!(f, "PackedFloat32Array(len={})", a.len()),
            Self::PackedFloat64Array(a) => write!(f, "PackedFloat64Array(len={})", a.len()),
            Self::PackedStringArray(a) => write!(f, "PackedStringArray(len={})", a.len()),
            Self::PackedVector2Array(a) => write!(f, "PackedVector2Array(len={})", a.len()),
            Self::PackedVector3Array(a) => write!(f, "PackedVector3Array(len={})", a.len()),
            Self::PackedColorArray(a) => write!(f, "PackedColorArray(len={})", a.len()),
        }
    }
}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Add for Variant {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a + b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a + b),
            (Self::Int(a), Self::Float(b)) => Self::Float(a as f64 + b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a + b as f64),
            (Self::String(a), Self::String(b)) => Self::String(format!("{}{}", a, b)),
            (Self::Vector2(a), Self::Vector2(b)) => Self::Vector2(a + b),
            (Self::Vector3(a), Self::Vector3(b)) => Self::Vector3(a + b),
            _ => Self::Nil,
        }
    }
}

impl Sub for Variant {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a - b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a - b),
            (Self::Int(a), Self::Float(b)) => Self::Float(a as f64 - b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a - b as f64),
            (Self::Vector2(a), Self::Vector2(b)) => Self::Vector2(a - b),
            (Self::Vector3(a), Self::Vector3(b)) => Self::Vector3(a - b),
            _ => Self::Nil,
        }
    }
}

impl Mul for Variant {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a * b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a * b),
            (Self::Int(a), Self::Float(b)) => Self::Float(a as f64 * b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a * b as f64),
            (Self::Vector2(a), Self::Float(b)) => Self::Vector2(a * b as f32),
            (Self::Vector3(a), Self::Float(b)) => Self::Vector3(a * b as f32),
            _ => Self::Nil,
        }
    }
}

impl Div for Variant {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) if b != 0 => Self::Int(a / b),
            (Self::Float(a), Self::Float(b)) => Self::Float(a / b),
            (Self::Int(a), Self::Float(b)) => Self::Float(a as f64 / b),
            (Self::Float(a), Self::Int(b)) if b != 0 => Self::Float(a / b as f64),
            (Self::Vector2(a), Self::Float(b)) if b != 0.0 => Self::Vector2(a / b as f32),
            (Self::Vector3(a), Self::Float(b)) if b != 0.0 => Self::Vector3(a / b as f32),
            _ => Self::Nil,
        }
    }
}
