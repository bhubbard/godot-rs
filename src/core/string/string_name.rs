use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringName(Arc<str>);

impl StringName {
    pub fn new(s: &str) -> Self {
        Self(Arc::from(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for StringName {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for StringName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for StringName {
    fn from(s: String) -> Self {
        Self(Arc::from(s))
    }
}

impl fmt::Display for StringName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for StringName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&\"{}\"", self.0)
    }
}

impl Serialize for StringName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StringName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StringName::new(&s))
    }
}
