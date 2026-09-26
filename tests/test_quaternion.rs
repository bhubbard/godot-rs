use godot::prelude::*;

#[test]
fn test_quaternion_identity() {
    let q = Quaternion::IDENTITY;
    assert_eq!(q.x, 0.0);
    assert_eq!(q.y, 0.0);
    assert_eq!(q.z, 0.0);
    assert_eq!(q.w, 1.0);
    assert_eq!(q.length(), 1.0);
}

#[test]
fn test_quaternion_rotation() {
    let q = Quaternion::from_axis_angle(Vector3::UP, std::f32::consts::FRAC_PI_2);
    let v = Vector3::RIGHT;
    let res = q.xform(v);
    assert!(res.is_equal_approx(Vector3::FORWARD));
}

#[test]
fn test_quaternion_multiplication() {
    let q1 = Quaternion::from_axis_angle(Vector3::UP, std::f32::consts::FRAC_PI_4);
    let q2 = Quaternion::from_axis_angle(Vector3::UP, std::f32::consts::FRAC_PI_4);
    let q_total = q1 * q2;

    let q_half_pi = Quaternion::from_axis_angle(Vector3::UP, std::f32::consts::FRAC_PI_2);
    assert!(q_total.is_equal_approx(&q_half_pi));
}

#[test]
fn test_quaternion_euler() {
    let euler = Vector3::new(0.0, std::f32::consts::FRAC_PI_2, 0.0);
    let q = Quaternion::from_euler(euler);
    let v = q.xform(Vector3::RIGHT);
    assert!(v.is_equal_approx(Vector3::FORWARD));
}
