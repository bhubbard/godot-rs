use godot::prelude::*;

#[test]
fn test_class_db_hierarchy() {
    ClassDb::init();

    assert!(ClassDb::class_exists("Node"));
    assert!(ClassDb::class_exists("CharacterBody2D"));
    assert!(ClassDb::class_exists("MeshInstance3D"));

    // Check inheritance chain: CharacterBody2D -> PhysicsBody2D -> CollisionObject2D -> Node2D -> CanvasItem -> Node -> Object
    assert!(ClassDb::is_parent_class("CharacterBody2D", "PhysicsBody2D"));
    assert!(ClassDb::is_parent_class("CharacterBody2D", "Node2D"));
    assert!(ClassDb::is_parent_class("CharacterBody2D", "Node"));
    assert!(ClassDb::is_parent_class("CharacterBody2D", "Object"));
    assert!(!ClassDb::is_parent_class("CharacterBody2D", "Control"));
}
