use godot::prelude::*;

#[test]
fn test_vector2_constructor() {
    let empty = Vector2::default();
    let zero = Vector2::new(0.0, 0.0);
    assert_eq!(empty, zero);
    assert_eq!(Vector2::splat(5.0), Vector2::new(5.0, 5.0));
    assert_eq!(Vector2::ZERO, Vector2::new(0.0, 0.0));
    assert_eq!(Vector2::ONE, Vector2::new(1.0, 1.0));
    assert_eq!(Vector2::LEFT, Vector2::new(-1.0, 0.0));
    assert_eq!(Vector2::RIGHT, Vector2::new(1.0, 0.0));
    assert_eq!(Vector2::UP, Vector2::new(0.0, -1.0));
    assert_eq!(Vector2::DOWN, Vector2::new(0.0, 1.0));
}

#[test]
fn test_vector2_angle_methods() {
    let v_x = Vector2::new(1.0, 0.0);
    let v_y = Vector2::new(0.0, 1.0);

    assert!(is_equal_approx(v_x.angle(), 0.0));
    assert!(is_equal_approx(v_y.angle(), std::f32::consts::FRAC_PI_2));

    let angle_to = v_x.angle_to(v_y);
    assert!(is_equal_approx(angle_to, std::f32::consts::FRAC_PI_2));

    let rot = v_x.rotated(std::f32::consts::FRAC_PI_2);
    assert!(v_y.is_equal_approx(rot));

    let orthogonal = v_x.orthogonal();
    assert_eq!(orthogonal, Vector2::new(0.0, -1.0));
}

#[test]
fn test_vector2_axis_methods() {
    let mut v = Vector2::new(1.2, 3.4);
    assert_eq!(v.min_axis_index(), 0);
    assert_eq!(v.max_axis_index(), 1);
    assert_eq!(v[0], 1.2);
    assert_eq!(v[1], 3.4);

    v[1] = 5.0;
    assert_eq!(v.y, 5.0);
}

#[test]
fn test_vector2_interpolation() {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(4.0, 5.0);

    assert_eq!(v1.lerp(v2, 0.5), Vector2::new(2.5, 3.5));

    let slerped = Vector2::RIGHT.slerp(Vector2::DOWN, 0.5);
    let expected = Vector2::new(1.0, 1.0).normalized();
    assert!(slerped.is_equal_approx(expected));

    let moved = Vector2::new(1.0, 0.0).move_toward(Vector2::new(10.0, 0.0), 3.0);
    assert_eq!(moved, Vector2::new(4.0, 0.0));
}

#[test]
fn test_vector2_linear_algebra() {
    let a = Vector2::new(2.0, 3.0);
    let b = Vector2::new(4.0, -1.0);

    assert_eq!(a.dot(b), 5.0); // 2*4 + 3*(-1) = 5
    assert_eq!(a.cross(b), -14.0); // 2*(-1) - 3*4 = -14

    // Project
    let p = Vector2::new(1.0, 2.0).project(Vector2::RIGHT);
    assert_eq!(p, Vector2::new(1.0, 0.0));

    // Slide
    let normal = Vector2::UP;
    let vel = Vector2::new(5.0, 10.0);
    let slide = vel.slide(normal);
    assert_eq!(slide, Vector2::new(5.0, 0.0));

    // Bounce & Reflect
    let in_vec = Vector2::new(1.0, 1.0);
    let bounced = in_vec.bounce(Vector2::UP);
    assert_eq!(bounced, Vector2::new(1.0, -1.0));
}

#[test]
fn test_vector2_rounding_and_limiting() {
    let v = Vector2::new(1.6, -2.3);
    assert_eq!(v.round(), Vector2::new(2.0, -2.0));
    assert_eq!(v.floor(), Vector2::new(1.0, -3.0));
    assert_eq!(v.ceil(), Vector2::new(2.0, -2.0));
    assert_eq!(v.abs(), Vector2::new(1.6, 2.3));
    assert_eq!(v.sign(), Vector2::new(1.0, -1.0));

    let snapped = Vector2::new(1.23, 4.56).snapped(Vector2::new(0.5, 1.0));
    assert_eq!(snapped, Vector2::new(1.0, 5.0));

    let clamped = Vector2::new(15.0, -5.0).clamp(Vector2::new(0.0, 0.0), Vector2::new(10.0, 10.0));
    assert_eq!(clamped, Vector2::new(10.0, 0.0));
}
