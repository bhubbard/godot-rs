use crate::core::math::{Color, Transform2D, Transform3D, Vector2, Vector3};
use crate::core::object::{GodotObject, ObjectId};
use crate::core::variant::Variant;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProcessMode {
    #[default]
    Inherit,
    Pausable,
    WhenPaused,
    Always,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node2DData {
    pub position: Vector2,
    pub rotation: f32,
    pub scale: Vector2,
    pub z_index: i32,
    pub visible: bool,
    pub modulate: Color,
}

impl Default for Node2DData {
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            rotation: 0.0,
            scale: Vector2::ONE,
            z_index: 0,
            visible: true,
            modulate: Color::WHITE,
        }
    }
}

impl Node2DData {
    pub fn get_transform(&self) -> Transform2D {
        Transform2D::from_angle_scale_origin(self.rotation, self.scale, self.position)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite2DData {
    pub node_2d: Node2DData,
    pub texture_path: String,
    pub offset: Vector2,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Default for Sprite2DData {
    fn default() -> Self {
        Self {
            node_2d: Node2DData::default(),
            texture_path: String::new(),
            offset: Vector2::ZERO,
            flip_h: false,
            flip_v: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera2DData {
    pub node_2d: Node2DData,
    pub zoom: Vector2,
    pub offset: Vector2,
    pub enabled: bool,
}

impl Default for Camera2DData {
    fn default() -> Self {
        Self {
            node_2d: Node2DData::default(),
            zoom: Vector2::ONE,
            offset: Vector2::ZERO,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape2DType {
    Circle { radius: f32 },
    Rectangle { size: Vector2 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionShape2DData {
    pub node_2d: Node2DData,
    pub shape: Shape2DType,
    pub disabled: bool,
}

impl Default for CollisionShape2DData {
    fn default() -> Self {
        Self {
            node_2d: Node2DData::default(),
            shape: Shape2DType::Rectangle { size: Vector2::new(32.0, 32.0) },
            disabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBody2DData {
    pub node_2d: Node2DData,
    pub velocity: Vector2,
    pub floor_normal: Vector2,
    pub on_floor: bool,
    pub on_wall: bool,
    pub on_ceiling: bool,
    pub motion_mode: MotionMode2D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MotionMode2D {
    #[default]
    Grounded,
    Floating,
}

impl Default for CharacterBody2DData {
    fn default() -> Self {
        Self {
            node_2d: Node2DData::default(),
            velocity: Vector2::ZERO,
            floor_normal: Vector2::UP,
            on_floor: false,
            on_wall: false,
            on_ceiling: false,
            motion_mode: MotionMode2D::Grounded,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area2DData {
    pub node_2d: Node2DData,
    pub monitoring: bool,
    pub monitorable: bool,
}

impl Default for Area2DData {
    fn default() -> Self {
        Self {
            node_2d: Node2DData::default(),
            monitoring: true,
            monitorable: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlData {
    pub position: Vector2,
    pub size: Vector2,
    pub custom_minimum_size: Vector2,
    pub text: String,
    pub color: Color,
    pub visible: bool,
}

impl Default for ControlData {
    fn default() -> Self {
        Self {
            position: Vector2::ZERO,
            size: Vector2::new(100.0, 40.0),
            custom_minimum_size: Vector2::ZERO,
            text: String::new(),
            color: Color::WHITE,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node3DData {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub visible: bool,
}

impl Default for Node3DData {
    fn default() -> Self {
        Self {
            position: Vector3::ZERO,
            rotation: Vector3::ZERO,
            scale: Vector3::ONE,
            visible: true,
        }
    }
}

impl Node3DData {
    pub fn get_transform(&self) -> Transform3D {
        let b = crate::core::math::Basis::from_euler(self.rotation).scaled(self.scale);
        Transform3D::new(b, self.position)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshInstance3DData {
    pub node_3d: Node3DData,
    pub mesh_name: String,
}

impl Default for MeshInstance3DData {
    fn default() -> Self {
        Self {
            node_3d: Node3DData::default(),
            mesh_name: "BoxMesh".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera3DData {
    pub node_3d: Node3DData,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub current: bool,
}

impl Default for Camera3DData {
    fn default() -> Self {
        Self {
            node_3d: Node3DData::default(),
            fov: 75.0,
            near: 0.05,
            far: 4000.0,
            current: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalLight3DData {
    pub node_3d: Node3DData,
    pub color: Color,
    pub energy: f32,
}

impl Default for DirectionalLight3DData {
    fn default() -> Self {
        Self {
            node_3d: Node3DData::default(),
            color: Color::WHITE,
            energy: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBody3DData {
    pub node_3d: Node3DData,
    pub velocity: Vector3,
    pub on_floor: bool,
}

impl Default for CharacterBody3DData {
    fn default() -> Self {
        Self {
            node_3d: Node3DData::default(),
            velocity: Vector3::ZERO,
            on_floor: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    Node,
    Node2D(Node2DData),
    Sprite2D(Sprite2DData),
    Camera2D(Camera2DData),
    CollisionShape2D(CollisionShape2DData),
    CharacterBody2D(CharacterBody2DData),
    Area2D(Area2DData),
    Control(ControlData),
    Label(ControlData),
    Button(ControlData),
    ColorRect(ControlData),
    Node3D(Node3DData),
    MeshInstance3D(MeshInstance3DData),
    Camera3D(Camera3DData),
    DirectionalLight3D(DirectionalLight3DData),
    CharacterBody3D(CharacterBody3DData),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub obj: GodotObject,
    pub name: String,
    pub parent: Option<ObjectId>,
    pub children: Vec<ObjectId>,
    pub owner: Option<ObjectId>,
    pub scene_file_path: String,
    pub process_mode: ProcessMode,
    pub is_ready: bool,
    pub is_in_tree: bool,
    pub groups: HashSet<String>,
    pub data: NodeData,
}

impl Node {
    pub fn new(name: impl Into<String>, class_name: &str) -> Self {
        let name_str = name.into();
        let mut obj = GodotObject::new(class_name);
        obj.add_user_signal("tree_entered");
        obj.add_user_signal("tree_exited");
        obj.add_user_signal("ready");

        let data = match class_name {
            "Node2D" => NodeData::Node2D(Node2DData::default()),
            "Sprite2D" => NodeData::Sprite2D(Sprite2DData::default()),
            "Camera2D" => NodeData::Camera2D(Camera2DData::default()),
            "CollisionShape2D" => NodeData::CollisionShape2D(CollisionShape2DData::default()),
            "CharacterBody2D" => NodeData::CharacterBody2D(CharacterBody2DData::default()),
            "Area2D" => NodeData::Area2D(Area2DData::default()),
            "Control" => NodeData::Control(ControlData::default()),
            "Label" => NodeData::Label(ControlData::default()),
            "Button" => NodeData::Button(ControlData::default()),
            "ColorRect" => NodeData::ColorRect(ControlData::default()),
            "Node3D" => NodeData::Node3D(Node3DData::default()),
            "MeshInstance3D" => NodeData::MeshInstance3D(MeshInstance3DData::default()),
            "Camera3D" => NodeData::Camera3D(Camera3DData::default()),
            "DirectionalLight3D" => NodeData::DirectionalLight3D(DirectionalLight3DData::default()),
            "CharacterBody3D" => NodeData::CharacterBody3D(CharacterBody3DData::default()),
            _ => NodeData::Node,
        };

        Self {
            obj,
            name: name_str,
            parent: None,
            children: Vec::new(),
            owner: None,
            scene_file_path: String::new(),
            process_mode: ProcessMode::Inherit,
            is_ready: false,
            is_in_tree: false,
            groups: HashSet::new(),
            data,
        }
    }

    pub fn id(&self) -> ObjectId {
        self.obj.get_instance_id()
    }

    pub fn get_class(&self) -> &str {
        self.obj.get_class()
    }

    pub fn add_to_group(&mut self, group: &str) {
        self.groups.insert(group.to_string());
    }

    pub fn remove_from_group(&mut self, group: &str) {
        self.groups.remove(group);
    }

    pub fn is_in_group(&self, group: &str) -> bool {
        self.groups.contains(group)
    }

    pub fn set_property(&mut self, name: &str, value: Variant) {
        // Sync with specific node typed data
        match &mut self.data {
            NodeData::Node2D(d) => match name {
                "position" => {
                    if let Some(v) = value.to_vector2() {
                        d.position = v;
                    }
                }
                "rotation" => {
                    if let Some(r) = value.to_f64() {
                        d.rotation = r as f32;
                    }
                }
                "scale" => {
                    if let Some(v) = value.to_vector2() {
                        d.scale = v;
                    }
                }
                "visible" => d.visible = value.to_bool(),
                _ => {}
            },
            NodeData::Sprite2D(s) => match name {
                "position" => {
                    if let Some(v) = value.to_vector2() {
                        s.node_2d.position = v;
                    }
                }
                "texture" => {
                    if let Some(st) = value.as_str() {
                        s.texture_path = st.to_string();
                    }
                }
                "flip_h" => s.flip_h = value.to_bool(),
                "flip_v" => s.flip_v = value.to_bool(),
                _ => {}
            },
            NodeData::CharacterBody2D(cb) => match name {
                "position" => {
                    if let Some(v) = value.to_vector2() {
                        cb.node_2d.position = v;
                    }
                }
                "velocity" => {
                    if let Some(v) = value.to_vector2() {
                        cb.velocity = v;
                    }
                }
                _ => {}
            },
            NodeData::Label(c) | NodeData::Button(c) => match name {
                "text" => {
                    if let Some(st) = value.as_str() {
                        c.text = st.to_string();
                    }
                }
                "position" => {
                    if let Some(v) = value.to_vector2() {
                        c.position = v;
                    }
                }
                "size" => {
                    if let Some(v) = value.to_vector2() {
                        c.size = v;
                    }
                }
                _ => {}
            },
            NodeData::Node3D(d) => match name {
                "position" => {
                    if let Some(v) = value.to_vector3() {
                        d.position = v;
                    }
                }
                "rotation" => {
                    if let Some(v) = value.to_vector3() {
                        d.rotation = v;
                    }
                }
                "scale" => {
                    if let Some(v) = value.to_vector3() {
                        d.scale = v;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        self.obj.set(name, value);
    }

    pub fn get_property(&self, name: &str) -> Option<&Variant> {
        self.obj.get(name)
    }
}
