use crate::core::string::StringName;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signal {
    pub object_id: u64,
    pub name: StringName,
}

impl Signal {
    pub fn new(object_id: u64, name: impl Into<StringName>) -> Self {
        Self {
            object_id,
            name: name.into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.object_id != 0 && !self.name.is_empty()
    }
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signal({}:{})", self.object_id, self.name)
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signal({}:{})", self.object_id, self.name)
    }
}
