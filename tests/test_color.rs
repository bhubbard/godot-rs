use godot::prelude::*;

#[test]
fn test_color_constants() {
    assert_eq!(Color::WHITE, Color::new(1.0, 1.0, 1.0, 1.0));
    assert_eq!(Color::BLACK, Color::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(Color::RED, Color::new(1.0, 0.0, 0.0, 1.0));
    assert_eq!(Color::GREEN, Color::new(0.0, 1.0, 0.0, 1.0));
    assert_eq!(Color::BLUE, Color::new(0.0, 0.0, 1.0, 1.0));
    assert_eq!(Color::TRANSPARENT, Color::new(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_color_conversions_and_hex() {
    let c = Color::from_rgba8(255, 128, 0, 255);
    assert_eq!(c.to_html_hex(), "#ff8000");

    let parsed = Color::from_html("#ff8000").unwrap();
    assert!(c.is_equal_approx(parsed));
}

#[test]
fn test_color_manipulation() {
    let red = Color::RED;
    assert_eq!(red.inverted(), Color::new(0.0, 1.0, 1.0, 1.0));

    let darkened = red.darkened(0.5);
    assert_eq!(darkened.r, 0.5);

    let lightened = red.lightened(0.5);
    assert_eq!(lightened.g, 0.5);
    assert_eq!(lightened.b, 0.5);

    let half_white = Color::WHITE.lerp(Color::BLACK, 0.5);
    assert_eq!(half_white, Color::new(0.5, 0.5, 0.5, 1.0));
}
