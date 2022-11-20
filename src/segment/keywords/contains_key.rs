use crate::decorator::DecoratorType;
use super::Keys;

impl Keys {
    pub fn contains_key(&self, text: &str) -> Option<String> 
    {

        //
        // This differs from starts_with_key
        // as this returns true if the text contains the key
        // anywhere in the text, caused by the decorator::Position 
        // enum.
        //
        // We still need to check if its the full key, not just
        // a part of it.
        //

        for config in self.decorator_configurations.iter() {
            match &config.decorator {
                DecoratorType::Wrapper(start, _end) => {
                    if !text.contains(start) { continue };

                    // -- Get the postion of the key in the text
                    let position = text.find(start).unwrap();
                    let end_position = position + start.len();
                    
                    if &config.allow_touching == &true {
                        return Some(start.to_owned());
                    } 

                    // make sure the key is not touching
                    // any text on either side
                    if text.len() < end_position { return None; }


                    let before = &text[position - 1..position];
                    let after = &text[end_position..end_position + 1];

                    if before == Keys::EMPTY && 
                        after == Keys::EMPTY 
                    { return Some(start.to_owned()); }
                },

                _ => {}
            }
        }
        

        // -- Return none
        None
    }
}