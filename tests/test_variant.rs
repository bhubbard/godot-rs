use godot::prelude::*;

#[test]
fn test_variant_types_and_arithmetic() {
    let a = Variant::Int(10);
    let b = Variant::Int(25);
    let c = a + b;
    assert_eq!(c, Variant::Int(35));

    let f1 = Variant::Float(2.5);
    let f2 = Variant::Float(4.0);
    assert_eq!(f1 * f2, Variant::Float(10.0));

    let s1 = Variant::String("Hello, ".to_string());
    let s2 = Variant::String("Godot-RS!".to_string());
    assert_eq!(s1 + s2, Variant::String("Hello, Godot-RS!".to_string()));

    let v1 = Variant::Vector2(Vector2::new(10.0, 20.0));
    let v2 = Variant::Vector2(Vector2::new(5.0, 5.0));
    assert_eq!(v1 - v2, Variant::Vector2(Vector2::new(5.0, 15.0)));
}

#[test]
fn test_variant_conversions() {
    let int_var = Variant::Int(42);
    assert_eq!(int_var.to_i64(), Some(42));
    assert_eq!(int_var.to_f64(), Some(42.0));
    assert!(int_var.to_bool());

    let nil_var = Variant::Nil;
    assert!(!nil_var.to_bool());
    assert!(nil_var.is_nil());

    let vec_var = Variant::Vector2(Vector2::new(12.0, 34.0));
    assert_eq!(vec_var.to_vector2(), Some(Vector2::new(12.0, 34.0)));
}
