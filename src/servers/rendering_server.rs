use crate::core::math::{Color, Rect2, Vector2};
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Rect {
        rect: Rect2,
        color: Color,
        filled: bool,
    },
    Circle {
        pos: Vector2,
        radius: f32,
        color: Color,
    },
    Line {
        from: Vector2,
        to: Vector2,
        color: Color,
        width: f32,
    },
    Texture {
        path: String,
        rect: Rect2,
        modulate: Color,
    },
    String {
        pos: Vector2,
        text: String,
        color: Color,
    },
}

#[derive(Debug, Default)]
pub struct RenderingServer {
    pub default_clear_color: Color,
    commands: Vec<DrawCommand>,
}

static RENDERING_SERVER: RwLock<Option<RenderingServer>> = RwLock::new(None);

impl RenderingServer {
    pub fn init() {
        let mut lock = RENDERING_SERVER.write().unwrap();
        *lock = Some(Self {
            default_clear_color: Color::new(0.15, 0.15, 0.18, 1.0),
            commands: Vec::new(),
        });
    }

    pub fn draw_rect(rect: Rect2, color: Color, filled: bool) {
        if let Ok(mut lock) = RENDERING_SERVER.write() {
            if let Some(ref mut s) = *lock {
                s.commands.push(DrawCommand::Rect { rect, color, filled });
            }
        }
    }

    pub fn draw_line(from: Vector2, to: Vector2, color: Color, width: f32) {
        if let Ok(mut lock) = RENDERING_SERVER.write() {
            if let Some(ref mut s) = *lock {
                s.commands.push(DrawCommand::Line { from, to, color, width });
            }
        }
    }

    pub fn draw_texture(path: &str, rect: Rect2, modulate: Color) {
        if let Ok(mut lock) = RENDERING_SERVER.write() {
            if let Some(ref mut s) = *lock {
                s.commands.push(DrawCommand::Texture {
                    path: path.to_string(),
                    rect,
                    modulate,
                });
            }
        }
    }

    pub fn draw_string(pos: Vector2, text: &str, color: Color) {
        if let Ok(mut lock) = RENDERING_SERVER.write() {
            if let Some(ref mut s) = *lock {
                s.commands.push(DrawCommand::String {
                    pos,
                    text: text.to_string(),
                    color,
                });
            }
        }
    }

    pub fn clear_commands() -> Vec<DrawCommand> {
        if let Ok(mut lock) = RENDERING_SERVER.write() {
            if let Some(ref mut s) = *lock {
                return std::mem::take(&mut s.commands);
            }
        }
        Vec::new()
    }
}
