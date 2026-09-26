use godot::prelude::*;

#[test]
fn test_aabb_properties() {
    let aabb = Aabb::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0));
    assert_eq!(aabb.position, Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(aabb.size, Vector3::new(4.0, 5.0, 6.0));
    assert_eq!(aabb.end(), Vector3::new(5.0, 7.0, 9.0));
    assert_eq!(aabb.volume(), 120.0);
    assert!(aabb.has_volume());
}

#[test]
fn test_aabb_contains_and_intersection() {
    let aabb1 = Aabb::new(Vector3::ZERO, Vector3::new(10.0, 10.0, 10.0));
    assert!(aabb1.has_point(Vector3::new(5.0, 5.0, 5.0)));
    assert!(!aabb1.has_point(Vector3::new(15.0, 5.0, 5.0)));

    let aabb2 = Aabb::new(Vector3::new(5.0, 5.0, 5.0), Vector3::new(10.0, 10.0, 10.0));
    assert!(aabb1.intersects(aabb2));

    let inter = aabb1.intersection(aabb2).unwrap();
    assert_eq!(inter.position, Vector3::new(5.0, 5.0, 5.0));
    assert_eq!(inter.size, Vector3::new(5.0, 5.0, 5.0));
}

#[test]
fn test_aabb_merge_and_grow() {
    let a = Aabb::new(Vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
    let b = Aabb::new(Vector3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));
    let merged = a.merge(b);
    assert_eq!(merged.position, Vector3::ZERO);
    assert_eq!(merged.size, Vector3::new(3.0, 3.0, 3.0));

    let grown = a.grow(1.0);
    assert_eq!(grown.position, Vector3::new(-1.0, -1.0, -1.0));
    assert_eq!(grown.size, Vector3::new(3.0, 3.0, 3.0));
}
