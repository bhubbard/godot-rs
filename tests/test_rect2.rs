use godot::prelude::*;

#[test]
fn test_rect2_constructors_and_properties() {
    let r = Rect2::from_components(10.0, 20.0, 100.0, 50.0);
    assert_eq!(r.position, Vector2::new(10.0, 20.0));
    assert_eq!(r.size, Vector2::new(100.0, 50.0));
    assert_eq!(r.end(), Vector2::new(110.0, 70.0));
    assert_eq!(r.area(), 5000.0);
    assert!(r.has_area());
}

#[test]
fn test_rect2_points_and_intersection() {
    let r = Rect2::from_components(0.0, 0.0, 50.0, 50.0);
    assert!(r.has_point(Vector2::new(25.0, 25.0)));
    assert!(!r.has_point(Vector2::new(60.0, 25.0)));

    let r2 = Rect2::from_components(40.0, 40.0, 50.0, 50.0);
    assert!(r.intersects(r2));

    let inter = r.intersection(r2).unwrap();
    assert_eq!(inter.position, Vector2::new(40.0, 40.0));
    assert_eq!(inter.size, Vector2::new(10.0, 10.0));
}

#[test]
fn test_rect2_merge_and_grow() {
    let r1 = Rect2::from_components(0.0, 0.0, 10.0, 10.0);
    let r2 = Rect2::from_components(20.0, 20.0, 10.0, 10.0);
    let merged = r1.merge(r2);
    assert_eq!(merged.position, Vector2::new(0.0, 0.0));
    assert_eq!(merged.size, Vector2::new(30.0, 30.0));

    let grown = r1.grow(5.0);
    assert_eq!(grown.position, Vector2::new(-5.0, -5.0));
    assert_eq!(grown.size, Vector2::new(20.0, 20.0));
}

#[test]
fn test_rect2i() {
    let ri = Rect2i::from_components(5, 5, 20, 20);
    assert!(ri.has_point(Vector2i::new(10, 10)));
    assert_eq!(ri.area(), 400);
}
