use godot::prelude::*;

#[test]
fn test_basis_construction_and_identity() {
    let b = Basis::IDENTITY;
    assert_eq!(b.determinant(), 1.0);
    assert_eq!(b[0], Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(b[1], Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(b[2], Vector3::new(0.0, 0.0, 1.0));

    let v = Vector3::new(2.0, 3.0, 4.0);
    assert_eq!(b * v, v);
}

#[test]
fn test_basis_scale_and_rotation() {
    let scale = Vector3::new(2.0, 3.0, 4.0);
    let b_scale = Basis::from_scale(scale);
    assert_eq!(b_scale.get_scale(), scale);

    let v = Vector3::new(1.0, 1.0, 1.0);
    assert_eq!(b_scale * v, Vector3::new(2.0, 3.0, 4.0));

    // Rotation 90 deg around Y
    let b_rot = Basis::from_axis_angle(Vector3::UP, std::f32::consts::FRAC_PI_2);
    let rotated = b_rot * Vector3::RIGHT;
    assert!(rotated.is_equal_approx(Vector3::FORWARD));
}

#[test]
fn test_basis_inverse() {
    let b = Basis::from_axis_angle(Vector3::UP, 0.7);
    let inv = b.inverse();
    let ident = b * inv;
    assert!(ident.is_equal_approx(&Basis::IDENTITY));
}
