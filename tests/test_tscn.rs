use godot::prelude::*;

#[test]
fn test_tscn_parsing_and_instantiation() {
    ClassDb::init();
    init_servers();

    let tscn_src = r#"[gd_scene format=3]

[node name="Level" type="Node2D"]
position = Vector2(10, 20)

[node name="Hero" type="CharacterBody2D" parent="."]
position = Vector2(100, 200)
velocity = Vector2(50, 0)
groups = ["players", "characters"]

[node name="Icon" type="Sprite2D" parent="Hero"]
position = Vector2(0, 0)

[node name="UI" type="Control" parent="."]
position = Vector2(0, 0)

[node name="Title" type="Label" parent="UI"]
text = "Level 1"
"#;

    let scene = TscnParser::parse_str(tscn_src).expect("Failed to parse tscn");
    assert_eq!(scene.nodes.len(), 5);

    let mut tree = SceneTree::new();
    let root = tree.get_root();

    let scene_root = scene.instantiate(&mut tree).expect("Failed to instantiate");
    tree.add_child(root, scene_root);

    // Verify root is Level
    let level_node = tree.get_node(scene_root).unwrap();
    assert_eq!(level_node.name, "Level");

    // Verify finding Hero
    let hero_id = tree.find_node_by_path(scene_root, &NodePath::new("Hero")).unwrap();
    let hero_node = tree.get_node(hero_id).unwrap();
    assert_eq!(hero_node.name, "Hero");
    assert!(hero_node.is_in_group("players"));
    assert!(hero_node.is_in_group("characters"));

    // Verify round-trip serialization
    let serialized = TscnSerializer::serialize(&scene);
    assert!(serialized.contains("[node name=\"Hero\" type=\"CharacterBody2D\" parent=\".\" groups=[\"players\", \"characters\"]]"));
}
