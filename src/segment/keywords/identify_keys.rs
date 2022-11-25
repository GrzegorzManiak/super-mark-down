use crate::decorator::config::DecoratorType;

use super::Keys;
use super::KeyLocation;


// 
// 
// TODO: Refactor this, it's a mess, and i just dont have
// the time to do it right now :(
// 
// 
impl Keys {
    pub fn identify_keys(&self, mut text: String) -> Vec<KeyLocation>
    {
        let org = text.clone();
        //
        // The goal here is to identify all the keys in the text
        // and return them as a vector of tuples,
        // where the tuple contains the key, the start position
        // and the end position.
        //

        let mut keys: Vec<KeyLocation> = Vec::new();
        let mut removed = 0;
        loop {
            let mut continue_loop = false;

            for config in self.decorator_configurations.iter() {
                match &config.decorator {
                    DecoratorType::Wrapper(start, end) => { 
                        // -- Get the start and end positions
                        let start_position = match self.find_without_escape(
                            text.to_string(), 
                            start, 
                            0
                        ) {
                            Some(position) => position + removed,
                            None => continue
                        };
    
                        let end_position = match self.find_without_escape(
                            text.to_string(), 
                            end, 
                            start_position + 1
                        ) {
                            Some(position) => position + 1 + start_position,
                            None => continue
                        };
    
    
                        let is_touching = 
                            self.is_touching(org.to_string(), start_position) || 
                            self.is_touching(org.to_string(), end_position);
    
                        
                        if (is_touching && config.allow_touching) || !is_touching
                        {
                            keys.push(
                                self.new_keylocation(
                                    config.decorator.clone(),
                                    start_position,
                                    end_position
                                )
                            );

                            // -- Remove the start and end positions from the text 
                            let mut split = self.split_at(text, start);
                            split.remove(0);

                            let mut split = self.split_at(split.join(""), end);
                            split.remove(0);
                            
                            text = split.join(end);
                            removed += start.len() + end.len();

                            continue_loop = true;
                        }
    
                    },
    
    
                    DecoratorType::Single(string) => {
                        if !text.starts_with(string) { continue };
    
                        // -- Get the index of the last character
                        //    in the text
                        let end = string.len();
    
    
                        // -- Touching other text / not touching other text
                        if &config.allow_touching == &true || 
                           &text[end..end + 1] == Keys::SPACE
                        {
                            keys.push(
                                self.new_keylocation(
                                    config.decorator.clone(), 
                                    0, 
                                    end - 1
                                )
                            );

                            // -- Remove the key from the text
                            let mut split = self.split_at(text, string);
                            split.remove(0);
                            text = split.join(string);


                            continue_loop = true;
                        }
                    }
                }  
            }

            if !continue_loop { break };
        }

        // -- Sort the keys by their start position
        //    meaning that the keys that start first
        //    will be first in the vector
        keys.sort_by(|a, b| a.start.cmp(&b.start));

        keys
    }
}