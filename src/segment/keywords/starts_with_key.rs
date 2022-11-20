use crate::decorator::DecoratorType;
use super::Keys;

impl Keys {
    pub fn starts_with_key(&self, text: &str) -> Option<String>
    {
        if text.starts_with(Keys::META) { return Some(
            Keys::META.to_string()) }
    

        for config in self.decorator_configurations.iter() {
            match &config.decorator 
            {
                DecoratorType::Single(key) => 
                {
                    if !text.starts_with(key) { continue };

                    let split = self.split_at(text, Keys::SCOPE_START);
                    let split = self.split_at(&split[0], Keys::ASSIGNMENT);
                    let split = self.split_at(&split[0], Keys::SPACE);
                    let split = self.split_at(&split[0], Keys::TAB);

                    // -- Check if the key is a scope key
                    if split[0] == key.to_owned() {
                        return Some(key.to_owned());
                    }
                },

                _ => {}
            }
        }

        // -- Return none
        None
    }
}