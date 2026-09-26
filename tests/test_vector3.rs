use godot::prelude::*;

#[test]
fn test_vector3_constructor() {
    let empty = Vector3::default();
    let zero = Vector3::new(0.0, 0.0, 0.0);
    assert_eq!(empty, zero);
    assert_eq!(Vector3::splat(3.0), Vector3::new(3.0, 3.0, 3.0));
    assert_eq!(Vector3::ZERO, Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(Vector3::ONE, Vector3::new(1.0, 1.0, 1.0));
    assert_eq!(Vector3::UP, Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(Vector3::DOWN, Vector3::new(0.0, -1.0, 0.0));
    assert_eq!(Vector3::LEFT, Vector3::new(-1.0, 0.0, 0.0));
    assert_eq!(Vector3::RIGHT, Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(Vector3::FORWARD, Vector3::new(0.0, 0.0, -1.0));
    assert_eq!(Vector3::BACK, Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_vector3_angle_and_linear_algebra() {
    let right = Vector3::RIGHT;
    let up = Vector3::UP;

    assert!(is_equal_approx(right.angle_to(up), std::f32::consts::FRAC_PI_2));
    assert_eq!(right.cross(up), Vector3::BACK); // X cross Y = Z in right-handed coords (BACK = +Z in Godot)
    assert_eq!(right.dot(up), 0.0);

    // Signed angle to
    let signed = right.signed_angle_to(up, Vector3::BACK);
    assert!(is_equal_approx(signed, std::f32::consts::FRAC_PI_2));

    // Project
    let proj = Vector3::new(2.0, 5.0, 0.0).project(Vector3::UP);
    assert_eq!(proj, Vector3::new(0.0, 5.0, 0.0));

    // Slide
    let vel = Vector3::new(10.0, -5.0, 2.0);
    let slide = vel.slide(Vector3::UP);
    assert_eq!(slide, Vector3::new(10.0, 0.0, 2.0));

    // Bounce
    let bounced = Vector3::new(0.0, -5.0, 0.0).bounce(Vector3::UP);
    assert_eq!(bounced, Vector3::new(0.0, 5.0, 0.0));
}

#[test]
fn test_vector3_axis_and_indexing() {
    let mut v = Vector3::new(1.0, 3.0, 2.0);
    assert_eq!(v.min_axis_index(), 0);
    assert_eq!(v.max_axis_index(), 1);
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], 3.0);
    assert_eq!(v[2], 2.0);

    v[2] = 10.0;
    assert_eq!(v.z, 10.0);
    assert_eq!(v.max_axis_index(), 2);
}

#[test]
fn test_vector3_interpolation() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(3.0, 4.0, 5.0);
    assert_eq!(a.lerp(b, 0.5), Vector3::new(2.0, 3.0, 4.0));

    let moved = Vector3::new(0.0, 0.0, 0.0).move_toward(Vector3::new(0.0, 10.0, 0.0), 4.0);
    assert_eq!(moved, Vector3::new(0.0, 4.0, 0.0));
}
