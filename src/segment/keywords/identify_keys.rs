use crate::decorator::config::DecoratorType;

use super::Keys;
use super::KeyLocation;

impl Keys {
    pub fn identify_keys(&self, text: &str) -> Vec<KeyLocation>
    {
        //
        // The goal here is to identify all the keys in the text
        // and return them as a vector of tuples,
        // where the tuple contains the key, the start position
        // and the end position.
        //

        let mut keys: Vec<KeyLocation> = Vec::new();
        // for config in self.decorator_configurations.iter() {
        //     match &config.decorator {
        //         DecoratorType::Wrapper(start, _end) => {
        //             if !text.contains(start) { continue };

        //             // -- Get the postion of the key in the text
        //             let position = text.find(start).unwrap();
        //             let end_position = position + start.len();
                    
        //             if &config.allow_touching == &true {
        //                 return Some(start.to_owned());
        //             } 

        //             // make sure the key is not touching
        //             // any text on either side
        //             if text.len() < end_position { return None; }


        //             let before = &text[position - 1..position];
        //             let after = &text[end_position..end_position + 1];

        //             if before == Keys::EMPTY && 
        //                 after == Keys::EMPTY 
        //             { return Some(start.to_owned()); }
        //         },

        //         _ => {}
        //     }
        // }
        
        for config in self.decorator_configurations.iter() {
            match &config.decorator {
                DecoratorType::Wrapper(start, _end) => {
                    if !text.contains(start) { continue };

                    // -- Get the postion of the key in the text
                    let position = text.find(start).unwrap();
                    let end_position = position + start.len();
                    
                    if &config.allow_touching == &true {
                        keys.push((
                            start.to_owned(),
                            position,
                            end_position
                        ));
                    } 

                    // make sure the key is not touching
                    // any text on either side
                    if text.len() < end_position { continue; }

                },

                DecoratorType::Single(string) => {
                    if !text.contains(string) { continue };

                    // -- Get the postion of the key in the text
                    let position = text.find(string).unwrap();
                    let end_position = position + string.len();
                    
                    if &config.allow_touching == &true {
                        keys.push((
                            string.to_owned(),
                            position,
                            end_position
                        ));
                    } 

                    // make sure the key is not touching
                    // any text on either side
                    if text.len() < end_position { continue; }
                },

                _ => {}
            
            }   

        }

        // -- Return none
        keys
    }
}