use godot::prelude::*;

#[test]
fn test_project_settings_parsing() {
    let ini = r#"
[application]
config/name="My Test Game"
run/main_scene="res://scenes/main.tscn"

[display]
window/size/viewport_width=1920
window/size/viewport_height=1080
"#;

    let settings = ProjectSettings::parse(ini).expect("Failed to parse project.godot");
    assert_eq!(settings.application_name, "My Test Game");
    assert_eq!(settings.main_scene, "res://scenes/main.tscn");
    assert_eq!(settings.window_width, 1920);
    assert_eq!(settings.window_height, 1080);
}
