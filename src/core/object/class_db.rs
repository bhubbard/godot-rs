use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub parent_class: String,
}

#[derive(Debug, Clone)]
pub struct ClassDb {
    classes: HashMap<String, ClassInfo>,
}

static CLASS_DB: RwLock<Option<ClassDb>> = RwLock::new(None);

impl ClassDb {
    pub fn init() {
        let mut db = ClassDb {
            classes: HashMap::new(),
        };

        // Register core Godot classes and inheritance hierarchy
        db.register_class("Object", "");
        db.register_class("RefCounted", "Object");
        db.register_class("Resource", "RefCounted");
        db.register_class("PackedScene", "Resource");
        db.register_class("Texture", "Resource");
        db.register_class("Texture2D", "Texture");
        db.register_class("Mesh", "Resource");
        db.register_class("Material", "Resource");

        // Scene classes
        db.register_class("Node", "Object");
        db.register_class("CanvasItem", "Node");
        db.register_class("Node2D", "CanvasItem");
        db.register_class("Sprite2D", "Node2D");
        db.register_class("Camera2D", "Node2D");
        db.register_class("CollisionShape2D", "Node2D");
        db.register_class("CollisionObject2D", "Node2D");
        db.register_class("Area2D", "CollisionObject2D");
        db.register_class("PhysicsBody2D", "CollisionObject2D");
        db.register_class("CharacterBody2D", "PhysicsBody2D");
        db.register_class("RigidBody2D", "PhysicsBody2D");
        db.register_class("StaticBody2D", "PhysicsBody2D");

        // GUI Control
        db.register_class("Control", "CanvasItem");
        db.register_class("Label", "Control");
        db.register_class("Button", "Control");
        db.register_class("ColorRect", "Control");

        // 3D Nodes
        db.register_class("Node3D", "Node");
        db.register_class("VisualInstance3D", "Node3D");
        db.register_class("MeshInstance3D", "VisualInstance3D");
        db.register_class("Camera3D", "Node3D");
        db.register_class("Light3D", "VisualInstance3D");
        db.register_class("DirectionalLight3D", "Light3D");
        db.register_class("CollisionObject3D", "Node3D");
        db.register_class("Area3D", "CollisionObject3D");
        db.register_class("PhysicsBody3D", "CollisionObject3D");
        db.register_class("CharacterBody3D", "PhysicsBody3D");
        db.register_class("RigidBody3D", "PhysicsBody3D");

        // Viewport / Window
        db.register_class("Viewport", "Node");
        db.register_class("Window", "Viewport");

        let mut lock = CLASS_DB.write().unwrap();
        *lock = Some(db);
    }

    fn register_class(&mut self, name: &str, parent: &str) {
        self.classes.insert(
            name.to_string(),
            ClassInfo {
                name: name.to_string(),
                parent_class: parent.to_string(),
            },
        );
    }

    pub fn class_exists(class_name: &str) -> bool {
        let lock = CLASS_DB.read().unwrap();
        if let Some(ref db) = *lock {
            db.classes.contains_key(class_name)
        } else {
            false
        }
    }

    pub fn get_parent_class(class_name: &str) -> Option<String> {
        let lock = CLASS_DB.read().unwrap();
        if let Some(ref db) = *lock {
            db.classes.get(class_name).map(|c| c.parent_class.clone())
        } else {
            None
        }
    }

    pub fn is_parent_class(class_name: &str, parent_name: &str) -> bool {
        let lock = CLASS_DB.read().unwrap();
        if let Some(ref db) = *lock {
            let mut curr = class_name;
            while let Some(info) = db.classes.get(curr) {
                if info.parent_class == parent_name {
                    return true;
                }
                if info.parent_class.is_empty() {
                    break;
                }
                curr = &info.parent_class;
            }
        }
        false
    }
}
