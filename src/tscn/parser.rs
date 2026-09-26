use crate::core::math::{Color, Rect2, Vector2, Vector3};
use crate::core::variant::Variant;
use crate::scene::resources::{PackedScene, SceneNodeData};
use indexmap::IndexMap;
use std::collections::HashMap;

pub struct TscnParser;

impl TscnParser {
    pub fn parse_file(path: &str) -> Result<PackedScene, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse_str(&content)
    }

    pub fn parse_str(content: &str) -> Result<PackedScene, String> {
        let mut packed_scene = PackedScene::new();
        let mut current_node: Option<SceneNodeData> = None;
        let mut ext_resources: HashMap<String, String> = HashMap::new();

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                // Section header
                if let Some(node) = current_node.take() {
                    packed_scene.add_node_data(node);
                }

                let section_content = &line[1..line.len() - 1];
                let mut parts = section_content.split_whitespace();
                let section_type = parts.next().unwrap_or("");

                match section_type {
                    "gd_scene" => {
                        // Header attributes
                    }
                    "ext_resource" => {
                        let attrs = parse_attributes(section_content);
                        if let (Some(id), Some(path)) = (attrs.get("id"), attrs.get("path")) {
                            ext_resources.insert(id.clone(), path.clone());
                        }
                    }
                    "node" => {
                        let attrs = parse_attributes(section_content);
                        let name = attrs.get("name").cloned().unwrap_or_else(|| "Node".to_string());
                        let type_name = attrs.get("type").cloned().unwrap_or_else(|| "Node".to_string());
                        let parent_path = attrs.get("parent").cloned();
                        let groups_attr = attrs.get("groups");

                        let groups = if let Some(g_str) = groups_attr {
                            parse_string_list(g_str)
                        } else {
                            Vec::new()
                        };

                        current_node = Some(SceneNodeData {
                            name,
                            type_name,
                            parent_path,
                            properties: IndexMap::new(),
                            groups,
                        });
                    }
                    _ => {}
                }
                continue;
            }

            // Inside a node section: parse property assignment key = value
            if let Some(node) = current_node.as_mut()
                && let Some((key, val_str)) = line.split_once('=')
            {
                let key = key.trim();
                if key == "groups" {
                    let groups = parse_string_list(val_str.trim());
                    for g in groups {
                        if !node.groups.contains(&g) {
                            node.groups.push(g);
                        }
                    }
                } else {
                    let val = parse_variant_value(val_str.trim(), &ext_resources);
                    node.properties.insert(key.to_string(), val);
                }
            }
        }

        if let Some(node) = current_node {
            packed_scene.add_node_data(node);
        }

        Ok(packed_scene)
    }
}

fn parse_attributes(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut chars = content.chars().peekable();

    // Skip tag name
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            break;
        }
        chars.next();
    }

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // Read key
        let mut key = String::new();
        while let Some(&k) = chars.peek() {
            if k == '=' || k.is_whitespace() {
                break;
            }
            key.push(k);
            chars.next();
        }

        while let Some(&eq) = chars.peek() {
            chars.next();
            if eq == '=' {
                break;
            }
        }

        // Read value (quoted string or bareword)
        while let Some(&w) = chars.peek() {
            if !w.is_whitespace() {
                break;
            }
            chars.next();
        }

        let mut val = String::new();
        if let Some(&'"') = chars.peek() {
            chars.next(); // Consume quote
            for v in chars.by_ref() {
                if v == '"' {
                    break;
                }
                val.push(v);
            }
        } else if let Some(&'[') = chars.peek() {
            chars.next();
            val.push('[');
            let mut depth = 1;
            for v in chars.by_ref() {
                val.push(v);
                if v == '[' {
                    depth += 1;
                } else if v == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
        } else {
            while let Some(&v) = chars.peek() {
                if v.is_whitespace() {
                    break;
                }
                val.push(v);
                chars.next();
            }
        }

        if !key.is_empty() {
            map.insert(key, val);
        }
    }

    map
}

fn parse_string_list(s: &str) -> Vec<String> {
    let trimmed = s.trim().trim_start_matches('[').trim_end_matches(']');
    trimmed
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

fn parse_variant_value(val_str: &str, ext_resources: &HashMap<String, String>) -> Variant {
    let s = val_str.trim();

    if s == "true" {
        return Variant::Bool(true);
    }
    if s == "false" {
        return Variant::Bool(false);
    }
    if s == "null" || s == "nil" {
        return Variant::Nil;
    }

    // String literal
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Variant::String(s[1..s.len() - 1].to_string());
    }

    // ExtResource("...")
    if s.starts_with("ExtResource(") && s.ends_with(')') {
        let inside = &s["ExtResource(".len()..s.len() - 1].trim().trim_matches('"');
        if let Some(path) = ext_resources.get(*inside) {
            return Variant::String(path.clone());
        }
        return Variant::String(inside.to_string());
    }

    // Vector2(x, y)
    if s.starts_with("Vector2(") && s.ends_with(')') {
        let inner = &s["Vector2(".len()..s.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 2 {
            let x = parts[0].trim().parse::<f32>().unwrap_or(0.0);
            let y = parts[1].trim().parse::<f32>().unwrap_or(0.0);
            return Variant::Vector2(Vector2::new(x, y));
        }
    }

    // Vector3(x, y, z)
    if s.starts_with("Vector3(") && s.ends_with(')') {
        let inner = &s["Vector3(".len()..s.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 3 {
            let x = parts[0].trim().parse::<f32>().unwrap_or(0.0);
            let y = parts[1].trim().parse::<f32>().unwrap_or(0.0);
            let z = parts[2].trim().parse::<f32>().unwrap_or(0.0);
            return Variant::Vector3(Vector3::new(x, y, z));
        }
    }

    // Color(r, g, b, a)
    if s.starts_with("Color(") && s.ends_with(')') {
        let inner = &s["Color(".len()..s.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() >= 3 {
            let r = parts[0].trim().parse::<f32>().unwrap_or(1.0);
            let g = parts[1].trim().parse::<f32>().unwrap_or(1.0);
            let b = parts[2].trim().parse::<f32>().unwrap_or(1.0);
            let a = if parts.len() > 3 {
                parts[3].trim().parse::<f32>().unwrap_or(1.0)
            } else {
                1.0
            };
            return Variant::Color(Color::new(r, g, b, a));
        }
    }

    // Rect2(x, y, w, h)
    if s.starts_with("Rect2(") && s.ends_with(')') {
        let inner = &s["Rect2(".len()..s.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() == 4 {
            let x = parts[0].trim().parse::<f32>().unwrap_or(0.0);
            let y = parts[1].trim().parse::<f32>().unwrap_or(0.0);
            let w = parts[2].trim().parse::<f32>().unwrap_or(0.0);
            let h = parts[3].trim().parse::<f32>().unwrap_or(0.0);
            return Variant::Rect2(Rect2::from_components(x, y, w, h));
        }
    }

    // Number (integer or float)
    if let Ok(i) = s.parse::<i64>() {
        return Variant::Int(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return Variant::Float(f);
    }

    // Fallback bare string
    Variant::String(s.to_string())
}
