use godot::prelude::*;

#[test]
fn test_callable_creation_and_validity() {
    let c1 = Callable::new(100, "on_pressed");
    assert!(c1.is_valid());
    assert_eq!(c1.object_id, 100);
    assert_eq!(c1.method.as_str(), "on_pressed");

    let c_invalid = Callable::new(0, "");
    assert!(!c_invalid.is_valid());
}

#[test]
fn test_signal_creation_and_validity() {
    let s = Signal::new(42, "body_entered");
    assert!(s.is_valid());
    assert_eq!(s.object_id, 42);
    assert_eq!(s.name.as_str(), "body_entered");
}
