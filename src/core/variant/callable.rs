use crate::core::string::StringName;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Callable {
    pub object_id: u64,
    pub method: StringName,
}

impl Callable {
    pub fn new(object_id: u64, method: impl Into<StringName>) -> Self {
        Self {
            object_id,
            method: method.into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.object_id != 0 && !self.method.is_empty()
    }
}

impl fmt::Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callable({}:{})", self.object_id, self.method)
    }
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callable({}:{})", self.object_id, self.method)
    }
}
