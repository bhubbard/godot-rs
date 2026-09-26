use godot::prelude::*;

#[test]
fn test_object_properties_and_signals() {
    ClassDb::init();

    let mut obj = GodotObject::new("Player");
    assert!(obj.get_instance_id().is_valid());
    assert_eq!(obj.get_class(), "Player");

    // Properties
    obj.set("health", Variant::Int(100));
    assert_eq!(obj.get("health"), Some(&Variant::Int(100)));
    assert!(obj.has_property("health"));

    // Signals
    obj.add_user_signal("died");
    let target = Callable::new(999, "game_over");
    obj.connect("died", target.clone());
    assert!(obj.is_connected("died", &target));

    let emissions = obj.emit_signal("died", &[]);
    assert_eq!(emissions, vec![target.clone()]);

    let disconnected = obj.disconnect("died", &target);
    assert!(disconnected);
    assert!(!obj.is_connected("died", &target));
}
