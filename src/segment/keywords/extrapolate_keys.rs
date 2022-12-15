use uuid::{Uuid, uuid};
use crate::decorator::config::DecoratorType;
use super::Keys;

#[derive(Debug)]
pub struct ExtrapolatedKey {
    pub decorator_type: DecoratorType,
    pub postion: usize,
    pub id: Uuid,
    parent: Uuid,
    pub end: Option<usize>
}

impl ExtrapolatedKey {
    pub fn new(
        decorator_type: DecoratorType, 
        postion: usize, 
        parent: Uuid,
        end: Option<usize>
    ) -> Self {
        Self {
            decorator_type,
            postion,
            id: Uuid::new_v4(),
            parent,
            end,
        }
    }

    pub fn set_parent(&mut self, parent: Uuid) {
        self.parent = parent;
    }

    pub fn get_parent(&self) -> Uuid {
        self.parent
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = Some(end);
    }

    pub fn clone(&self) -> Self {
        Self {
            decorator_type: self.decorator_type.clone(),
            postion: self.postion,
            id: Uuid::new_v4(),
            parent: self.parent,
            end: self.end
        }
    }
}

type ExtrapolatedKeysVec = Vec<ExtrapolatedKey>;

#[derive(Debug)]
pub struct ExtrapolatedKeys {
    pub root_id: Uuid,
    pub keys: Vec<ExtrapolatedKey>
}

impl ExtrapolatedKeys {
    pub fn new() -> Self {
        Self {
            root_id: Uuid::new_v4(),
            keys: Vec::new()
        }
    }

    pub fn add_key(&mut self, key: ExtrapolatedKey) {
        self.keys.push(key);
    }
}


impl Keys {
    pub fn check_escaped(
        &self,
        text: &str,  // -- The text to check
        key: &str,   // -- The key to check
        index: usize // -- The index of the key in the text
    ) -> bool
    {
        // -- Get all instances of the key
        let possitions = text.match_indices(key).collect::<Vec<_>>();

        // -- Check if our index exists
        if possitions.len() <= index { return false; }

        // -- Get the index of the key
        let possition = possitions[index].0;

        // -- Get all the escape characters before the key
        // end at the last non-escape character
        let mut escape_chars = 0;
        for c in (0..possition).rev() {
            if text.chars().nth(c).unwrap().to_string() == Self::ESCAPE {
                escape_chars += 1;
            } else { break; }
        }

        // -- Check if the escape characters are even
        if escape_chars % 2 == 0 { return false; }
        else { return true; }
    }


    pub fn extrapolate_keys(&self, text: &str) -> ExtrapolatedKeys
    {   
        // -- Variable to store the extrapolated keys
        //    and a list of potential keys
        let mut keys: ExtrapolatedKeys = ExtrapolatedKeys::new();
        let mut potential_possitions: ExtrapolatedKeysVec = Vec::new();


        // -- Loop through all the decorator configurations
        // and identify all the potential keys
        for config in self.decorator_configurations.iter() {
            match &config.decorator {
                DecoratorType::Wrapper(start, end) => {
                    // -- Check if we contain the start and end key
                    if !text.contains(start) || !text.contains(end) 
                    { continue; }


                    // -- Find all instances of the start and end key
                    let start_possitions = text.match_indices(start).collect::<Vec<_>>();
                    let end_possitions = text.match_indices(end).collect::<Vec<_>>();

                    // -- make sure they arent escaped, continue if they are
                    for (index, _) in start_possitions.iter().enumerate() {
                        if self.check_escaped(text, start, index) { continue; }
                        
                        // -- Add the key to the list
                        potential_possitions.push(ExtrapolatedKey::new(
                            config.decorator.clone(),
                            index,
                            keys.root_id,
                            None
                        ));
                    }

                    for (index, _) in end_possitions.iter().enumerate() {
                        // -- Make sure that the end key isint one
                        // already covered by a start key
                        if start_possitions.contains(&end_possitions[index]) { continue; }
                        if self.check_escaped(text, end, index) { continue; }
                        
                        // -- Add the key to the list
                        potential_possitions.push(ExtrapolatedKey::new(
                            config.decorator.clone(),
                            index,
                            keys.root_id,
                            None
                        ));
                    }
                },


                // 
                // We only need to check the start of the key
                // as a single decorator can only be at the start
                // of the text.
                // 
                DecoratorType::Single(key) => {
                    if !text.starts_with(key) { continue; }

                    // -- Check if the key is escaped
                    if text[key.len()..key.len() + 1] == Keys::ESCAPE.to_owned() 
                    { continue; }

                    // -- Add the key to the list
                    potential_possitions.push(ExtrapolatedKey::new(
                        config.decorator.clone(),
                        0,
                        keys.root_id,
                        None
                    ));
                },
            }
        }
        


        //
        // So the method of extrapolating keys I've dreamt up is
        // to find all the start keys, and then find its end key
        // by checking the next value in the potential_possitions
        // list. If its the end key, then we add it to the keys
        // its considered a root key.
        // 
        // if the user nests decotators, then the next value in
        // the potential_possitions list will be the start key
        // of the next decorator, and so on, until we find the
        // end key of the deepest decorator.
        //

        let mut index = 0;
        let mut used_indexes: Vec<usize> = Vec::new();

        loop {
            // Check if we have used all the keys
            if used_indexes.len() >= potential_possitions.len() { break; }
            if used_indexes.contains(&index) { index += 1; continue; }

            // If the index is greater than the length of the keys
            // set it back to 0
            if index >= potential_possitions.len() { index = 0; }


            // -- Get the current key
            let cur_key = &potential_possitions[index];

            // -- Check if the key is a start key
            if let DecoratorType::Wrapper(start, end) = &cur_key.decorator_type 
            {
                // -- Check if we can get the next key, and if so
                // get it
                if index + 1 >= potential_possitions.len() { break; }
                let next_key = &potential_possitions[index + 1];


                // -- Check if the next key matches the end key
                if let DecoratorType::Wrapper(next_start, next_end) = &next_key.decorator_type 
                {   
                    // -- If they do match, remove them from the list
                    if next_start == start && next_end == end {
                        // -- Set the end of the first key to the end of the second key
                        let mut cur_key = cur_key.clone();
                        cur_key.set_end(next_key.postion);
                        keys.keys.push(cur_key);

                        // -- Add the indexes to the used indexes
                        used_indexes.push(index);
                        used_indexes.push(index + 1);
                    }
                }
            }

            // -- increment the index
            index += 1;
        }

        // // Print the potential keys
        // for key in potential_possitions.iter() {
        //     println!("P => {:?} ", key)
        // }

        // // TODO: Add parental references to the keys
        // for key in keys.keys.iter_mut() {
        //     println!("{:?} ", key)
        // }

        // -- Return none
        keys
    }
}