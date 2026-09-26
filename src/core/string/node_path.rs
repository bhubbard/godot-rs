use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct NodePath {
    raw: String,
    names: Vec<String>,
    subnames: Vec<String>,
    is_absolute: bool,
}

impl NodePath {
    pub fn new(path: &str) -> Self {
        let is_absolute = path.starts_with('/');
        let trimmed = path.trim();

        if trimmed.is_empty() {
            return Self {
                raw: String::new(),
                names: Vec::new(),
                subnames: Vec::new(),
                is_absolute: false,
            };
        }

        let mut parts = trimmed.split(':');
        let node_part = parts.next().unwrap_or("");
        let subnames: Vec<String> = parts.map(|s| s.to_string()).collect();

        let names: Vec<String> = node_part
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Self {
            raw: trimmed.to_string(),
            names,
            subnames,
            is_absolute,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn get_name_count(&self) -> usize {
        self.names.len()
    }

    pub fn get_name(&self, idx: usize) -> Option<&str> {
        self.names.get(idx).map(|s| s.as_str())
    }

    pub fn get_names(&self) -> &[String] {
        &self.names
    }

    pub fn get_subname_count(&self) -> usize {
        self.subnames.len()
    }

    pub fn get_subname(&self, idx: usize) -> Option<&str> {
        self.subnames.get(idx).map(|s| s.as_str())
    }

    pub fn get_subnames(&self) -> &[String] {
        &self.subnames
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl From<&str> for NodePath {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for NodePath {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "^{}", self.raw)
    }
}

impl fmt::Debug for NodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodePath(\"{}\")", self.raw)
    }
}
