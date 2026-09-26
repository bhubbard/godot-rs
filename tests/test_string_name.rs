use godot::prelude::*;

#[test]
fn test_string_name_creation_and_equality() {
    let sn1 = StringName::new("player");
    let sn2 = StringName::from("player");
    assert_eq!(sn1, sn2);
    assert_eq!(sn1.as_str(), "player");

    let sn_empty = StringName::default();
    assert!(sn_empty.is_empty());
}
