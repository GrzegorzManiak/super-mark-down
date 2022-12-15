use std::cmp::Ordering;

use crate::decorator::{
    get_all_decorators,
    config::{ 
        DecoratorMap, 
        Config, DecoratorType
    }, Decorator
};

pub mod tests;
pub mod identify_keys;
pub mod contains_scope;
pub mod validate_scope;
pub mod starts_with_key;
pub mod extrapolate_keys;


// key, start, end
pub struct KeyLocation {
    pub start: usize,
    pub end: usize,
    pub decorator: Box<dyn Decorator>
}

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



        // -- We want to sort the decorator map by the length of the key
        //    because its the easiest way to make sure we get the longest
        //    key first. therefore you cant have a key that is a part of
        //    another key.
        configurations.sort_by(|a, b| {
            match a.decorator {
                DecoratorType::Wrapper(ref start, _) => {
                    match b.decorator {
                        DecoratorType::Wrapper(ref start2, _) => {
                            start.len().cmp(&start2.len()) },
                        _ => Ordering::Less
                    }
                },
                _ => Ordering::Less
            }
        });



        Self {
            decorators,
            decorator_configurations: configurations,
        }
    }


    pub fn new_keylocation(
        &self,
        decorator_key: DecoratorType,
        start: usize,
        end: usize
    ) -> KeyLocation {
        let decorator = match decorator_key {
            DecoratorType::Single(key) => {
                match self.decorators.get(&key) {
                    Some(decorator) => decorator,
                    None => panic!("Decorator not found")
                }.as_ref()
            },

            //
            // The reason we have two identical match statements
            // is because im future proofing this function, so that
            // you can have multiple wrapper decorators with the same
            // start OR end key. but not both. right now each decorator
            // has a unique start key.
            //
            DecoratorType::Wrapper(start, _end) => {
                match self.decorators.get(&start) {
                    Some(decorator) => decorator,
                    None => panic!("Decorator not found")
                }.as_ref()
            }
        };

        KeyLocation {
            start,
            end,
            decorator: decorator.clone()
        }
    }


    pub fn split_at(
        &self, 
        text: String, 
        separator: &str
    ) -> Vec<String> {
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


    pub fn find_without_escape(
        &self, 
        text: String, 
        key: &str,
        start: usize
    ) -> Option<usize> {
        let mut index = 0;
        let mut escape = false;
 
        for c in start..text.len() {
            let character = text.chars().nth(c).unwrap();
            let character_string = character.to_string();

            if character_string == Self::ESCAPE {
                escape = !escape;
            } else if character_string == key {
                if !escape {
                    return Some(index);
                }
            }

            index += 1;
        }

        None
    }


    pub fn is_empty(&self, text: &str) -> bool {
        text == Self::EMPTY || text == Self::SPACE || text == Self::TAB
    }


    pub fn is_touching(
        &self, 
        text: String, 
        index: usize
    ) -> bool {
        
        let before = match index {
            0 => Self::EMPTY.to_string(),
            _ => { match text.chars().nth(index - 1) {
                Some(c) => c.to_string(),
                None => Self::EMPTY.to_string()
            }}
        };


        let after = match text.chars().nth(index + 1) {
            Some(c) => c.to_string(),
            None => return false
        };


        
        if self.is_empty(&before) && 
           self.is_empty(&after) 
        {
            return false;
        }

        return true;
    }
}
