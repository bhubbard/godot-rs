use godot::prelude::*;

#[test]
fn test_scene_tree_hierarchy_and_paths() {
    ClassDb::init();
    init_servers();

    let mut tree = SceneTree::new();
    let root = tree.get_root();

    let main_node = Node::new("Main", "Node2D");
    let main_id = tree.add_node(main_node);
    tree.add_child(root, main_id);

    let mut player_node = Node::new("Player", "CharacterBody2D");
    player_node.add_to_group("actors");
    player_node.set_property("velocity", Variant::Vector2(Vector2::new(10.0, 0.0)));
    let player_id = tree.add_node(player_node);
    tree.add_child(main_id, player_id);

    let sprite_node = Node::new("Sprite", "Sprite2D");
    let sprite_id = tree.add_node(sprite_node);
    tree.add_child(player_id, sprite_id);

    // Verify finding node by relative path
    let found_player = tree.find_node_by_path(main_id, &NodePath::new("Player"));
    assert_eq!(found_player, Some(player_id));

    let found_sprite = tree.find_node_by_path(main_id, &NodePath::new("Player/Sprite"));
    assert_eq!(found_sprite, Some(sprite_id));

    // Verify groups
    let actors = tree.get_nodes_in_group("actors");
    assert_eq!(actors, vec![player_id]);

    // Verify stepping
    tree.physics_step(1.0);
    let p_ref = tree.get_node(player_id).unwrap();
    if let NodeData::CharacterBody2D(ref cb) = p_ref.data {
        assert_eq!(cb.node_2d.position.x, 10.0);
    }
}
