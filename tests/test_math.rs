use godot::prelude::*;

#[test]
fn test_vector2_math() {
    let v1 = Vector2::new(3.0, 4.0);
    assert_eq!(v1.length(), 5.0);

    let v2 = Vector2::new(1.0, 0.0);
    let rot = v2.rotated(std::f32::consts::FRAC_PI_2);
    assert!(is_equal_approx(rot.x, 0.0));
    assert!(is_equal_approx(rot.y, 1.0));

    let sum = v1 + v2;
    assert_eq!(sum, Vector2::new(4.0, 4.0));
}

#[test]
fn test_rect2_intersection() {
    let r1 = Rect2::from_components(0.0, 0.0, 100.0, 100.0);
    let r2 = Rect2::from_components(50.0, 50.0, 100.0, 100.0);
    assert!(r1.intersects(r2));

    let inter = r1.intersection(r2).unwrap();
    assert_eq!(inter.position, Vector2::new(50.0, 50.0));
    assert_eq!(inter.size, Vector2::new(50.0, 50.0));
}

#[test]
fn test_transform2d() {
    let t = Transform2D::from_angle_origin(0.0, Vector2::new(10.0, 20.0));
    let pt = Vector2::new(5.0, 5.0);
    let transformed = t.xform(pt);
    assert_eq!(transformed, Vector2::new(15.0, 25.0));

    let inv = t.inverse();
    let original = inv.xform(transformed);
    assert!(is_equal_approx(original.x, pt.x));
    assert!(is_equal_approx(original.y, pt.y));
}

#[test]
fn test_quaternion_slerp() {
    let q0 = Quaternion::IDENTITY;
    let q1 = Quaternion::from_axis_angle(Vector3::UP, std::f32::consts::PI);
    let q_mid = q0.slerp(q1, 0.5);

    let rot_vec = q_mid.xform(Vector3::RIGHT);
    // Rotating Vector3::RIGHT around UP by 90 degrees points to BACK (Z = 1.0)
    assert!(is_equal_approx(rot_vec.x, 0.0));
    assert!(is_equal_approx(rot_vec.z, 1.0));
}

#[test]
fn test_color() {
    let c = Color::from_rgba8(255, 0, 128, 255);
    assert_eq!(c.to_html_hex(), "#ff0080");

    let parsed = Color::from_html("#ff0080").unwrap();
    assert_eq!(parsed.to_rgba8(), (255, 0, 128, 255));
}
