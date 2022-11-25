use crate::decorator::config::DecoratorType;
use super::Keys;

impl Keys {
    pub fn starts_with_key(&self, text: &str) -> Option<String>
    {
        for config in self.decorator_configurations.iter() {
            match &config.decorator 
            {
                DecoratorType::Single(key) => 
                {
                    if !text.starts_with(key) { continue };

                    let split = self.split_at(text.to_string(), Keys::SCOPE_START);
                    let split = self.split_at(split[0].clone(), Keys::ASSIGNMENT);
                    let split = self.split_at(split[0].clone(), Keys::SPACE);
                    let split = self.split_at(split[0].clone(), Keys::TAB);

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