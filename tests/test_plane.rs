use godot::prelude::*;

#[test]
fn test_plane_construction() {
    let p = Plane::new(Vector3::UP, 10.0);
    assert_eq!(p.normal, Vector3::UP);
    assert_eq!(p.d, 10.0);

    let p2 = Plane::from_point_normal(Vector3::new(0.0, 5.0, 0.0), Vector3::UP);
    assert_eq!(p2.d, 5.0);
}

#[test]
fn test_plane_distance_and_projection() {
    let p = Plane::new(Vector3::UP, 0.0); // Ground plane y = 0
    let pt = Vector3::new(1.0, 5.0, 2.0);

    assert_eq!(p.distance_to(pt), 5.0);
    assert!(p.is_point_over(pt));

    let projected = p.project(pt);
    assert_eq!(projected, Vector3::new(1.0, 0.0, 2.0));
}
