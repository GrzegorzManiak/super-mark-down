use crate::decorator::{
    get_all_decorators,
    config::{ 
        DecoratorMap, 
        Config
    }
};

mod tests;
mod contains_key;
mod contains_scope;
mod validate_scope;
mod starts_with_key;


pub struct Keys {
    decorators: DecoratorMap,
    decorator_configurations: Vec<Config>
}

impl Keys {
    pub const IMPORT: &str = "import";

    pub const SCOPE_START: &str = "[";
    pub const SCOPE_END: &str = "]";

    pub const CLASS_ASSIGNMENT: &str = ":";

    pub const ASSIGNMENT: &str = "=";
    pub const META: &str = "@";
    pub const SEPARATOR: &str = ",";

    pub const EOL: &str = "\n";
    pub const SPACE: &str = " ";
    pub const TAB: &str = "\t";
    pub const EMPTY: &str = "";

    pub const ESCAPE: &str = "\\";



    pub fn new() -> Self {
        let decorators = get_all_decorators();
        let mut configurations = Vec::new();


        // -- Create a list of all the scope keys
        for decorator in decorators.values() {
            let config = decorator.get_config();
            configurations.push(config);
        }

        
        Self {
            decorators,
            decorator_configurations: configurations,
        }
    }


    pub fn split_at(&self, text: &str, separator: &str) -> Vec<String> {
        let mut split = Vec::new();
        let mut current = String::new();

        for c in text.chars() {
            if c.to_string() == separator {
                split.push(current.clone());
                current = String::new();
            } else {
                current.push(c);
            }
        }

        split.push(current);
        split
    }




    
}
