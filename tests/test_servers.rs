use godot::prelude::*;

#[test]
fn test_servers_initialization_and_rendering() {
    init_servers();

    RenderingServer::draw_rect(
        Rect2::from_components(0.0, 0.0, 100.0, 100.0),
        Color::RED,
        true,
    );
    let cmds = RenderingServer::clear_commands();
    assert_eq!(cmds.len(), 1);
}

#[test]
fn test_physics_server_2d_raycast() {
    init_servers();

    PhysicsServer2D::register_body(PhysicsBody2DDesc {
        id: 10,
        position: Vector2::new(100.0, 100.0),
        size: Vector2::new(50.0, 50.0),
        is_static: true,
    });

    let hit = PhysicsServer2D::raycast(Vector2::new(0.0, 100.0), Vector2::new(200.0, 100.0));
    assert!(hit.is_some());
    let res = hit.unwrap();
    assert_eq!(res.collider_id, 10);
    assert!(is_equal_approx(res.position.x, 75.0)); // Left edge of body at 100 - 25 = 75
}

#[test]
fn test_audio_server() {
    init_servers();
    assert!(AudioServer::get_bus_count() >= 1);
    AudioServer::set_bus_volume_db(0, -6.0);
    assert_eq!(AudioServer::get_bus_volume_db(0), -6.0);
}

#[test]
fn test_input_server() {
    init_servers();

    Input::action_press("ui_right", 1.0);
    assert!(Input::is_action_pressed("ui_right"));
    assert!(Input::is_action_just_pressed("ui_right"));

    let axis = Input::get_axis("ui_left", "ui_right");
    assert_eq!(axis, 1.0);

    Input::flush_just_pressed();
    assert!(!Input::is_action_just_pressed("ui_right"));
    assert!(Input::is_action_pressed("ui_right"));

    Input::action_release("ui_right");
    assert!(!Input::is_action_pressed("ui_right"));
}
