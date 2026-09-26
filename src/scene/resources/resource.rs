use crate::core::object::GodotObject;

pub trait ResourceTrait {
    fn get_path(&self) -> &str;
    fn set_path(&mut self, path: &str);
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub obj: GodotObject,
    pub resource_path: String,
    pub resource_name: String,
}

impl Default for Resource {
    fn default() -> Self {
        Self::new("Resource")
    }
}

impl Resource {
    pub fn new(class_name: &str) -> Self {
        Self {
            obj: GodotObject::new(class_name),
            resource_path: String::new(),
            resource_name: String::new(),
        }
    }
}

impl ResourceTrait for Resource {
    fn get_path(&self) -> &str {
        &self.resource_path
    }

    fn set_path(&mut self, path: &str) {
        self.resource_path = path.to_string();
    }
}
