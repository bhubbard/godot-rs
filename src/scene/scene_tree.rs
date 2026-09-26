use crate::core::object::ObjectId;
use crate::core::string::NodePath;
use crate::scene::node::{Node, NodeData, ProcessMode};
use std::collections::HashMap;

pub struct SceneTree {
    root_id: ObjectId,
    nodes: HashMap<ObjectId, Node>,
    current_scene_id: Option<ObjectId>,
    pub paused: bool,
    pub process_frame: u64,
    pub physics_frame: u64,
    pub time_elapsed: f64,
}

impl Default for SceneTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneTree {
    pub fn new() -> Self {
        let mut root_node = Node::new("root", "Window");
        let root_id = root_node.id();
        root_node.is_in_tree = true;
        root_node.is_ready = true;

        let mut nodes = HashMap::new();
        nodes.insert(root_id, root_node);

        Self {
            root_id,
            nodes,
            current_scene_id: None,
            paused: false,
            process_frame: 0,
            physics_frame: 0,
            time_elapsed: 0.0,
        }
    }

    pub fn get_root(&self) -> ObjectId {
        self.root_id
    }

    pub fn get_current_scene(&self) -> Option<ObjectId> {
        self.current_scene_id
    }

    pub fn set_current_scene(&mut self, scene_id: ObjectId) {
        self.current_scene_id = Some(scene_id);
    }

    pub fn add_node(&mut self, node: Node) -> ObjectId {
        let id = node.id();
        self.nodes.insert(id, node);
        id
    }

    pub fn get_node(&self, id: ObjectId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: ObjectId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn add_child(&mut self, parent_id: ObjectId, child_id: ObjectId) -> bool {
        if !self.nodes.contains_key(&parent_id) || !self.nodes.contains_key(&child_id) {
            return false;
        }

        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent = Some(parent_id);
            child.is_in_tree = true;
            child.is_ready = true;
        }

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            if !parent.children.contains(&child_id) {
                parent.children.push(child_id);
            }
        }

        true
    }

    pub fn remove_child(&mut self, parent_id: ObjectId, child_id: ObjectId) -> bool {
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.retain(|&id| id != child_id);
        }

        if let Some(child) = self.nodes.get_mut(&child_id) {
            if child.parent == Some(parent_id) {
                child.parent = None;
                child.is_in_tree = false;
                return true;
            }
        }

        false
    }

    pub fn find_node_by_path(&self, start_id: ObjectId, path: &NodePath) -> Option<ObjectId> {
        if path.is_empty() {
            return Some(start_id);
        }

        let mut curr_id = if path.is_absolute() {
            self.root_id
        } else {
            start_id
        };

        for name in path.get_names() {
            if name == "." {
                continue;
            }
            if name == ".." {
                let curr_node = self.nodes.get(&curr_id)?;
                curr_id = curr_node.parent?;
                continue;
            }

            // Find child by name
            let curr_node = self.nodes.get(&curr_id)?;
            let mut found = None;
            for &child_id in &curr_node.children {
                if let Some(child) = self.nodes.get(&child_id) {
                    if child.name == *name {
                        found = Some(child_id);
                        break;
                    }
                }
            }

            curr_id = found?;
        }

        Some(curr_id)
    }

    pub fn get_nodes_in_group(&self, group: &str) -> Vec<ObjectId> {
        self.nodes
            .values()
            .filter(|n| n.is_in_group(group))
            .map(|n| n.id())
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn can_process(&self, id: ObjectId) -> bool {
        if let Some(node) = self.nodes.get(&id) {
            match node.process_mode {
                ProcessMode::Disabled => false,
                ProcessMode::Always => true,
                ProcessMode::WhenPaused => self.paused,
                ProcessMode::Pausable => !self.paused,
                ProcessMode::Inherit => {
                    if let Some(parent_id) = node.parent {
                        self.can_process(parent_id)
                    } else {
                        !self.paused
                    }
                }
            }
        } else {
            false
        }
    }

    pub fn step(&mut self, delta: f32) {
        self.process_frame += 1;
        self.time_elapsed += delta as f64;

        // Traverse nodes and simulate process
        let node_ids: Vec<ObjectId> = self.collect_tree_order(self.root_id);
        for id in node_ids {
            if !self.can_process(id) {
                continue;
            }

            // Process callback or simulation
            // For CharacterBody2D with velocity:
            if let Some(node) = self.nodes.get_mut(&id) {
                if let NodeData::CharacterBody2D(ref mut cb) = node.data {
                    cb.node_2d.position += cb.velocity * delta;
                }
            }
        }
    }

    pub fn physics_step(&mut self, delta: f32) {
        self.physics_frame += 1;

        let node_ids: Vec<ObjectId> = self.collect_tree_order(self.root_id);
        for id in node_ids {
            if !self.can_process(id) {
                continue;
            }

            if let Some(node) = self.nodes.get_mut(&id) {
                match &mut node.data {
                    NodeData::CharacterBody2D(cb) => {
                        // Move and slide simulation
                        cb.node_2d.position += cb.velocity * delta;
                        if cb.node_2d.position.y >= 500.0 {
                            cb.node_2d.position.y = 500.0;
                            cb.velocity.y = 0.0;
                            cb.on_floor = true;
                        }
                    }
                    NodeData::CharacterBody3D(cb) => {
                        cb.node_3d.position += cb.velocity * delta;
                        if cb.node_3d.position.y <= 0.0 {
                            cb.node_3d.position.y = 0.0;
                            cb.velocity.y = 0.0;
                            cb.on_floor = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn collect_tree_order(&self, root: ObjectId) -> Vec<ObjectId> {
        let mut list = Vec::new();
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            list.push(id);
            if let Some(node) = self.nodes.get(&id) {
                for &child in node.children.iter().rev() {
                    stack.push(child);
                }
            }
        }
        list
    }

    pub fn print_tree_pretty(&self) -> String {
        let mut out = String::new();
        self.print_node_recursive(self.root_id, "", true, &mut out);
        out
    }

    fn print_node_recursive(&self, id: ObjectId, prefix: &str, is_last: bool, out: &mut String) {
        if let Some(node) = self.nodes.get(&id) {
            let marker = if id == self.root_id {
                "● "
            } else if is_last {
                "└── "
            } else {
                "├── "
            };

            let details = match &node.data {
                NodeData::Node2D(d) => format!("pos=({}, {})", d.position.x, d.position.y),
                NodeData::Sprite2D(s) => format!("tex=\"{}\" pos=({}, {})", s.texture_path, s.node_2d.position.x, s.node_2d.position.y),
                NodeData::CharacterBody2D(cb) => format!("vel=({}, {}) pos=({}, {}) floor={}", cb.velocity.x, cb.velocity.y, cb.node_2d.position.x, cb.node_2d.position.y, cb.on_floor),
                NodeData::Label(c) => format!("text=\"{}\" pos=({}, {})", c.text, c.position.x, c.position.y),
                NodeData::Button(c) => format!("btn=\"{}\" pos=({}, {})", c.text, c.position.x, c.position.y),
                NodeData::Node3D(d) => format!("pos=({}, {}, {})", d.position.x, d.position.y, d.position.z),
                NodeData::MeshInstance3D(m) => format!("mesh=\"{}\" pos=({}, {}, {})", m.mesh_name, m.node_3d.position.x, m.node_3d.position.y, m.node_3d.position.z),
                _ => String::new(),
            };

            let suffix = if details.is_empty() {
                format!("{} ({})\n", node.name, node.get_class())
            } else {
                format!("{} ({}) [{}]\n", node.name, node.get_class(), details)
            };

            out.push_str(&format!("{}{}{}", prefix, marker, suffix));

            let next_prefix = if id == self.root_id {
                ""
            } else if is_last {
                &format!("{}    ", prefix)
            } else {
                &format!("{}│   ", prefix)
            };

            for (i, &child_id) in node.children.iter().enumerate() {
                let last = i == node.children.len() - 1;
                self.print_node_recursive(child_id, next_prefix, last, out);
            }
        }
    }
}
