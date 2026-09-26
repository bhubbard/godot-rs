use godot::prelude::*;

#[test]
fn test_node_path_parsing() {
    let p = NodePath::new("Player/Arm/Hand:rotation");
    assert!(!p.is_absolute());
    assert_eq!(p.get_name_count(), 3);
    assert_eq!(p.get_name(0), Some("Player"));
    assert_eq!(p.get_name(1), Some("Arm"));
    assert_eq!(p.get_name(2), Some("Hand"));

    assert_eq!(p.get_subname_count(), 1);
    assert_eq!(p.get_subname(0), Some("rotation"));
}

#[test]
fn test_node_path_absolute() {
    let p = NodePath::new("/root/Main/Camera2D");
    assert!(p.is_absolute());
    assert_eq!(p.get_names(), &["root", "Main", "Camera2D"]);
}
