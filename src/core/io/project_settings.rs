use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct ProjectSettings {
    pub application_name: String,
    pub main_scene: String,
    pub window_width: u32,
    pub window_height: u32,
    pub settings: HashMap<String, IndexMap<String, String>>,
}

impl ProjectSettings {
    pub fn new() -> Self {
        Self {
            application_name: "Godot-RS Application".to_string(),
            main_scene: "".to_string(),
            window_width: 1152,
            window_height: 648,
            settings: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self, String> {
        let mut settings = Self::new();
        let mut current_section = "application".to_string();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].to_string();
                continue;
            }

            if let Some((key, val)) = trimmed.split_once('=') {
                let key = key.trim().to_string();
                let mut val = val.trim().to_string();
                if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
                    val = val[1..val.len() - 1].to_string();
                }

                if current_section == "application" {
                    if key == "config/name" {
                        settings.application_name = val.clone();
                    } else if key == "run/main_scene" {
                        settings.main_scene = val.clone();
                    }
                } else if current_section == "display" {
                    if key == "window/size/viewport_width"
                        && let Ok(w) = val.parse::<u32>()
                    {
                        settings.window_width = w;
                    } else if key == "window/size/viewport_height"
                        && let Ok(h) = val.parse::<u32>()
                    {
                        settings.window_height = h;
                    }
                }

                settings
                    .settings
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, val);
            }
        }

        Ok(settings)
    }

    pub fn get_value(&self, section: &str, key: &str) -> Option<&str> {
        self.settings
            .get(section)
            .and_then(|sec| sec.get(key).map(|s| s.as_str()))
    }
}
