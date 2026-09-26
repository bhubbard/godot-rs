use godot::prelude::*;

#[test]
fn test_transform3d_identity() {
    let t = Transform3D::IDENTITY;
    assert_eq!(t.origin, Vector3::ZERO);
    assert_eq!(t.basis, Basis::IDENTITY);

    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(t.xform(v), v);
}

#[test]
fn test_transform3d_translation_and_inverse() {
    let offset = Vector3::new(10.0, -20.0, 30.0);
    let t = Transform3D::from_origin(offset);
    let pt = Vector3::new(1.0, 1.0, 1.0);
    let transformed = t.xform(pt);
    assert_eq!(transformed, pt + offset);

    let inv = t.affine_inverse();
    let back = inv.xform(transformed);
    assert!(back.is_equal_approx(pt));
}

#[test]
fn test_transform3d_looking_at() {
    let eye = Transform3D::from_origin(Vector3::new(0.0, 0.0, 5.0));
    let looked = eye.looking_at(Vector3::ZERO, Vector3::UP);
    let fwd = looked.basis.xform(Vector3::FORWARD);
    assert!(fwd.is_equal_approx(Vector3::FORWARD));
}
