use crate::core::variant::Variant;
use crate::scene::resources::PackedScene;

pub struct TscnSerializer;

impl TscnSerializer {
    pub fn serialize(scene: &PackedScene) -> String {
        let mut out = String::new();
        out.push_str("[gd_scene format=3]\n\n");

        for node in &scene.nodes {
            out.push_str("[node");
            out.push_str(&format!(" name=\"{}\"", node.name));
            out.push_str(&format!(" type=\"{}\"", node.type_name));

            if let Some(ref p) = node.parent_path {
                out.push_str(&format!(" parent=\"{}\"", p));
            }

            if !node.groups.is_empty() {
                let groups_formatted = node
                    .groups
                    .iter()
                    .map(|g| format!("\"{}\"", g))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!(" groups=[{}]", groups_formatted));
            }

            out.push_str("]\n");

            for (k, v) in &node.properties {
                out.push_str(&format!("{} = {}\n", k, format_variant(v)));
            }
            out.push('\n');
        }

        out
    }
}

fn format_variant(v: &Variant) -> String {
    match v {
        Variant::Nil => "null".to_string(),
        Variant::Bool(b) => format!("{}", b),
        Variant::Int(i) => format!("{}", i),
        Variant::Float(f) => {
            if f.fract() == 0.0 {
                format!("{:.1}", f)
            } else {
                format!("{}", f)
            }
        }
        Variant::String(s) => format!("\"{}\"", s),
        Variant::Vector2(v) => format!("Vector2({}, {})", v.x, v.y),
        Variant::Vector3(v) => format!("Vector3({}, {}, {})", v.x, v.y, v.z),
        Variant::Color(c) => format!("Color({}, {}, {}, {})", c.r, c.g, c.b, c.a),
        Variant::Rect2(r) => format!(
            "Rect2({}, {}, {}, {})",
            r.position.x, r.position.y, r.size.x, r.size.y
        ),
        _ => format!("{}", v),
    }
}
