use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OBJECT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectId(u64);

impl ObjectId {
    pub fn new() -> Self {
        Self(NEXT_OBJECT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn from_u64(val: u64) -> Self {
        Self(val)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
