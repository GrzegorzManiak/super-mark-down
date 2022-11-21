use crate::paramater::Parameter;
use super::rand_string;

pub struct Class {
    pub name: String,
    pub id: String,
}

impl Class {
    pub fn new(
        name: String,
    ) -> Self
    {
        Self {
            name,
            id: rand_string(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                10
            )
        }
    }
}


pub struct ClassStorage {
    pub classes: Vec<Class>,
    pub get_class: fn(&str) -> Option<Class>
}

impl ClassStorage {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            get_class: |name: &str| { None }
        }
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }
}


pub struct Parser {
    pub class_manager: ClassStorage,
}
impl Parser {
    fn new() -> Self {
        Self {
            class_manager: ClassStorage::new()
        }
    } 
}