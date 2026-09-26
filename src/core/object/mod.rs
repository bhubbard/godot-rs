pub mod class_db;
pub mod object_id;

pub use class_db::ClassDb;
pub use object_id::ObjectId;

use crate::core::string::StringName;
use crate::core::variant::{Callable, Variant};
use indexmap::IndexMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notification {
    PostInitialize = 0,
    PreDelete = 1,
    EnterTree = 10,
    ExitTree = 11,
    Ready = 13,
    Paused = 14,
    Unpaused = 15,
    PhysicsProcess = 16,
    Process = 17,
    Parented = 18,
    Unparented = 19,
    Draw = 30,
    VisibilityChanged = 31,
    TransformChanged = 2000,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignalConnection {
    pub callable: Callable,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct GodotObject {
    id: ObjectId,
    class_name: StringName,
    properties: IndexMap<StringName, Variant>,
    signals: HashMap<StringName, Vec<SignalConnection>>,
}

impl GodotObject {
    pub fn new(class_name: impl Into<StringName>) -> Self {
        Self {
            id: ObjectId::new(),
            class_name: class_name.into(),
            properties: IndexMap::new(),
            signals: HashMap::new(),
        }
    }

    pub fn get_instance_id(&self) -> ObjectId {
        self.id
    }

    pub fn get_class(&self) -> &str {
        self.class_name.as_str()
    }

    pub fn is_class(&self, class: &str) -> bool {
        self.class_name.as_str() == class || ClassDb::is_parent_class(self.class_name.as_str(), class)
    }

    pub fn set(&mut self, property: impl Into<StringName>, value: impl Into<Variant>) {
        self.properties.insert(property.into(), value.into());
    }

    pub fn get(&self, property: &str) -> Option<&Variant> {
        let name = StringName::new(property);
        self.properties.get(&name)
    }

    pub fn has_property(&self, property: &str) -> bool {
        let name = StringName::new(property);
        self.properties.contains_key(&name)
    }

    pub fn get_property_list(&self) -> Vec<StringName> {
        self.properties.keys().cloned().collect()
    }

    pub fn add_user_signal(&mut self, signal_name: impl Into<StringName>) {
        self.signals.entry(signal_name.into()).or_default();
    }

    pub fn connect(&mut self, signal_name: impl Into<StringName>, callable: Callable) {
        let name = signal_name.into();
        let list = self.signals.entry(name).or_default();
        if !list.iter().any(|c| c.callable == callable) {
            list.push(SignalConnection { callable, flags: 0 });
        }
    }

    pub fn disconnect(&mut self, signal_name: &str, callable: &Callable) -> bool {
        let name = StringName::new(signal_name);
        if let Some(list) = self.signals.get_mut(&name) {
            let initial_len = list.len();
            list.retain(|c| &c.callable != callable);
            return list.len() < initial_len;
        }
        false
    }

    pub fn is_connected(&self, signal_name: &str, callable: &Callable) -> bool {
        let name = StringName::new(signal_name);
        if let Some(list) = self.signals.get(&name) {
            list.iter().any(|c| &c.callable == callable)
        } else {
            false
        }
    }

    pub fn emit_signal(&self, signal_name: &str, _args: &[Variant]) -> Vec<Callable> {
        let name = StringName::new(signal_name);
        if let Some(list) = self.signals.get(&name) {
            list.iter().map(|c| c.callable.clone()).collect()
        } else {
            Vec::new()
        }
    }
}
