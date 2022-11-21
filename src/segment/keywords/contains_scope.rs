use crate::decorator::config::DecoratorType;
use super::Keys;

impl Keys {
    pub fn contains_scope(&self, text: &str) -> bool
    {
        // -- Check if the text contains a scope
        if !text.contains(Keys::SCOPE_START) { 
            return false; }

        
        // -- Get the position of the scope
        let position = text.find(Keys::SCOPE_START).unwrap();


        // -- Make sure the scope is not escaped
        if text[position - 1..position] == Keys::ESCAPE.to_owned() {
            return false; }


        // -- make sure the scope is touching a key
        for config in self.decorator_configurations.iter() {
            match &config.decorator {
                // -- If the decorator is a wrapper, check the end key
                // instead of the start key, as thats where the user
                // will be able to add parameters
                DecoratorType::Wrapper(_start, end) => {
                    let formated = format!("{}{}", end, Keys::SCOPE_START);
                    if text.contains(&formated) { return true; }
                },


                DecoratorType::Single(key) => {
                    let formated = format!("{}{}", key, Keys::SCOPE_START);
                    if text.starts_with(&formated) { return true; }
                },
            }
        }
        
        // -- Return none
        false
    }
}