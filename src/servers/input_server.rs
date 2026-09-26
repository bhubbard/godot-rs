use crate::core::math::Vector2;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct InputServer {
    pressed_actions: HashSet<String>,
    just_pressed_actions: HashSet<String>,
    action_strengths: HashMap<String, f32>,
}

static INPUT_SERVER: RwLock<Option<InputServer>> = RwLock::new(None);

pub struct Input;

impl Input {
    pub fn init() {
        let mut lock = INPUT_SERVER.write().unwrap();
        *lock = Some(InputServer::default());
    }

    pub fn action_press(action: &str, strength: f32) {
        if let Ok(mut lock) = INPUT_SERVER.write() {
            if let Some(ref mut inp) = *lock {
                if !inp.pressed_actions.contains(action) {
                    inp.just_pressed_actions.insert(action.to_string());
                }
                inp.pressed_actions.insert(action.to_string());
                inp.action_strengths.insert(action.to_string(), strength);
            }
        }
    }

    pub fn action_release(action: &str) {
        if let Ok(mut lock) = INPUT_SERVER.write() {
            if let Some(ref mut inp) = *lock {
                inp.pressed_actions.remove(action);
                inp.just_pressed_actions.remove(action);
                inp.action_strengths.remove(action);
            }
        }
    }

    pub fn flush_just_pressed() {
        if let Ok(mut lock) = INPUT_SERVER.write() {
            if let Some(ref mut inp) = *lock {
                inp.just_pressed_actions.clear();
            }
        }
    }

    pub fn is_action_pressed(action: &str) -> bool {
        INPUT_SERVER
            .read()
            .ok()
            .and_then(|l| l.as_ref().map(|s| s.pressed_actions.contains(action)))
            .unwrap_or(false)
    }

    pub fn is_action_just_pressed(action: &str) -> bool {
        INPUT_SERVER
            .read()
            .ok()
            .and_then(|l| l.as_ref().map(|s| s.just_pressed_actions.contains(action)))
            .unwrap_or(false)
    }

    pub fn get_action_strength(action: &str) -> f32 {
        INPUT_SERVER
            .read()
            .ok()
            .and_then(|l| l.as_ref().and_then(|s| s.action_strengths.get(action).copied()))
            .unwrap_or(0.0)
    }

    pub fn get_axis(negative_action: &str, positive_action: &str) -> f32 {
        let mut axis = 0.0;
        if Self::is_action_pressed(positive_action) {
            axis += 1.0;
        }
        if Self::is_action_pressed(negative_action) {
            axis -= 1.0;
        }
        axis
    }

    pub fn get_vector(
        negative_x: &str,
        positive_x: &str,
        negative_y: &str,
        positive_y: &str,
    ) -> Vector2 {
        let v = Vector2::new(
            Self::get_axis(negative_x, positive_x),
            Self::get_axis(negative_y, positive_y),
        );
        let len = v.length();
        if len > 1.0 {
            v.normalized()
        } else {
            v
        }
    }
}
