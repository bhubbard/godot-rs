use godot::prelude::*;

#[test]
fn test_transform2d_identity_and_components() {
    let t = Transform2D::IDENTITY;
    assert_eq!(t.x, Vector2::new(1.0, 0.0));
    assert_eq!(t.y, Vector2::new(0.0, 1.0));
    assert_eq!(t.origin, Vector2::new(0.0, 0.0));

    let p = Vector2::new(3.0, 4.0);
    assert_eq!(t.xform(p), p);
}

#[test]
fn test_transform2d_translation_and_scale() {
    let t = Transform2D::from_angle_scale_origin(0.0, Vector2::new(2.0, 3.0), Vector2::new(10.0, 20.0));
    let p = Vector2::new(1.0, 1.0);
    let transformed = t.xform(p);
    assert_eq!(transformed, Vector2::new(12.0, 23.0));

    let inv = t.affine_inverse();
    let back = inv.xform(transformed);
    assert!(back.is_equal_approx(p));
}

#[test]
fn test_transform2d_composition() {
    let t1 = Transform2D::from_angle_origin(0.0, Vector2::new(5.0, 0.0));
    let t2 = Transform2D::from_angle_origin(0.0, Vector2::new(0.0, 10.0));
    let t_combined = t1 * t2;

    assert_eq!(t_combined.xform(Vector2::ZERO), Vector2::new(5.0, 10.0));
}
