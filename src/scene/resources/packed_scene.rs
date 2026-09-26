use super::resource::Resource;
use crate::core::object::ObjectId;
use crate::core::variant::Variant;
use crate::scene::node::Node;
use crate::scene::scene_tree::SceneTree;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneNodeData {
    pub name: String,
    pub type_name: String,
    pub parent_path: Option<String>,
    pub properties: IndexMap<String, Variant>,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PackedScene {
    pub resource: Resource,
    pub nodes: Vec<SceneNodeData>,
}

impl Default for PackedScene {
    fn default() -> Self {
        Self::new()
    }
}

impl PackedScene {
    pub fn new() -> Self {
        Self {
            resource: Resource::new("PackedScene"),
            nodes: Vec::new(),
        }
    }

    pub fn add_node_data(&mut self, node: SceneNodeData) {
        self.nodes.push(node);
    }

    /// Instantiates the packed scene into the given SceneTree, returning the root node's ObjectId.
    pub fn instantiate(&self, tree: &mut SceneTree) -> Option<ObjectId> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut path_to_id: HashMap<String, ObjectId> = HashMap::new();
        let mut root_id: Option<ObjectId> = None;

        for (idx, node_data) in self.nodes.iter().enumerate() {
            let mut node = Node::new(&node_data.name, &node_data.type_name);

            // Apply groups
            for g in &node_data.groups {
                node.add_to_group(g);
            }

            // Apply properties
            for (key, val) in &node_data.properties {
                node.set_property(key, val.clone());
            }

            let node_id = tree.add_node(node);

            if idx == 0 {
                root_id = Some(node_id);
                path_to_id.insert(".".to_string(), node_id);
                path_to_id.insert(node_data.name.clone(), node_id);
            } else {
                let parent_key = node_data
                    .parent_path
                    .as_deref()
                    .unwrap_or(".");

                let parent_id = path_to_id.get(parent_key).copied().or(root_id);

                if let Some(pid) = parent_id {
                    tree.add_child(pid, node_id);
                }

                let full_path = if parent_key == "." {
                    node_data.name.clone()
                } else {
                    format!("{}/{}", parent_key, node_data.name)
                };
                path_to_id.insert(full_path, node_id);
            }
        }

        root_id
    }
}
