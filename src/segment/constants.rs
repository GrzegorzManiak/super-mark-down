use crate::decorator::{
    DecoratorMap, 
    get_all_decorators, 
    Parameter, 
    DecoratorType, 
    Config
};

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

    pub fn is_meta(&self, line: &str) -> bool {
        line.starts_with(Keys::META)
    }


    pub fn starts_with_key(&self, text: &str) -> Option<String>
    {
        if self.is_meta(text) { return Some(
            Keys::META.to_string()) }
    

        for config in self.decorator_configurations.iter() {
            match &config.decorator {
                DecoratorType::Single(key) => {
                    if text.starts_with(key) {
                        let split = self.split_at(text, Keys::SCOPE_START);
                        let split = self.split_at(&split[0], Keys::ASSIGNMENT);
                        let split = self.split_at(&split[0], Keys::SPACE);
                        let split = self.split_at(&split[0], Keys::TAB);

                        // -- Check if the key is a scope key
                        if split[0] == key.to_owned() {
                            return Some(key.to_owned());
                        }
                    }
                },

                _ => {}
            }
        }

        // -- Return none
        None
    }

    
    fn contains_key(&self, text: &str) -> Option<String> 
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
                    if text.contains(start) {
                        // -- Get the postion of the key in the text
                        let position = text.find(start).unwrap();
                        let end_position = position + start.len();
                        
                        match &config.allow_touching {
                            true => return Some(start.to_owned()),
                            false => {
                                // make sure the key is not touching
                                // any text on either side
                                if text.len() < end_position { return None; }


                                let before = &text[position - 1..position];
                                let after = &text[end_position..end_position + 1];


                                if before == Keys::EMPTY && 
                                    after == Keys::EMPTY 
                                {
                                    return Some(start.to_owned());
                                }
                            }
                        }
                    }
                },

                _ => {}
            }
        }
        

        // -- Return none
        None
    }


    pub fn contains_key_and_scope(&self, text: &str) -> bool
    {
        // -- Check if its a meta key
        if text.starts_with(Keys::META) {
            return true;
        }

        // -- Check if the text starts with any of the keys
        match self.starts_with_key(text) {
            None => {
                // -- Check if the text contains any of the keys
                return self.contains_key(text) != None;
            }
            Some(key) => {
                // -- Check if the key is imeediately 
                // followed by a scope opening
                if text.contains(&format!("{}{}", key, Keys::SCOPE_START)) {
                    // -- Return the key and scope
                    return true;
                }
            }
        }

        // -- Return none
        false
    }

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

    pub fn validate_scope (
        line: &str,
    ) -> Option<Vec<(usize, usize)>> 
    {
        //
        // We need to make sure that the scope is valid
        // we also need to account for the fact that a parameter
        // in the scope might contain a scope opening or closing 
        // character, and it might be escaped.
        //
        let mut scopes = Vec::new();

       
        // -- A singular line might contain multiple scopes
        // so we need to check for all of them
        let mut scope_start = 0;
        let mut scope_end = 0;
        let mut scope_depth = 0;
        let mut escape = false;

        let mut opened = 0;
        let mut closed = 0;
        
        for (i, c) in line.chars().enumerate() {
            if c.to_string() == Keys::ESCAPE {
                escape = true;
                continue;
            }

            if c.to_string() == Keys::SCOPE_START {
                if escape {
                    escape = false;
                    continue;
                }

                if scope_depth == 0 {
                    scope_start = i;
                }

                scope_depth += 1;
                opened += 1;
            }

            if c.to_string() == Keys::SCOPE_END {
                if escape {
                    escape = false;
                    continue;
                }

                scope_depth -= 1;
                closed += 1;

                if scope_depth == 0 {
                    scope_end = i;
                    scopes.push((scope_start, scope_end));
                }
            }
        }

        
        // -- If we have more scopes opened than closed
        // we need to return an error 
        if opened != closed { return None; }
        else if scopes.len() > 0 { return Some(scopes) }
        
        None
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // 
    // Contains scope
    //
    #[test]
    fn contains_scope_valid() {
        let line = "im just < some >[] text";
        let keys = Keys::new();
        let res = keys.contains_scope(line);

        assert_eq!(res, true);
    }

    #[test]
    fn contains_scope_invalid() {
        let line = "im just < some [] text";
        let keys = Keys::new();
        let res = keys.contains_scope(line);

        assert_eq!(res, false);
    }

    #[test]
    fn contains_scope_escaped() {
        let line = "im just < some >\\[ text";
        let keys = Keys::new();
        let res = keys.contains_scope(line);

        assert_eq!(res, false);
    }

    #[test]
    fn contains_scope_one_valid_one_invalid() {
        let line = "im just < some >[] text < some > [ text";
        let keys = Keys::new();
        let res = keys.contains_scope(line);

        assert_eq!(res, true);
    }


    // 
    // Contains key
    //
    #[test]
    fn contains_key_anywhere_touple() {
        let line = "im just < some >[] text";
        let keys = Keys::new();
        let res = keys.contains_key(line);

        assert_eq!(res, Some("<".to_string()));
    }


    //
    // Keys::META
    //

    #[test]
    fn test_starts_with_meta() {
        let line = "@test = \"test\"";
        let keys = Keys::new();
        let res = keys.starts_with_key(line);

        assert_eq!(res, Some(Keys::META.to_string()));
    }

    #[test]
    fn test_starts_with_none() {
        let line = "test = \"test\"";
        let keys = Keys::new();
        let res = keys.starts_with_key(line);

        assert_eq!(res, None);
    }

    #[test]
    fn test_starts_with_heading() {
        let line = "###[somethingEEE = 'd'] Im a smaller x2 heading";
        let keys = Keys::new();
        let res = keys.starts_with_key(line);

        assert_eq!(res, Some("###".to_string()));
    }

    #[test]
    fn test_starts_with_heading_2() {
        let line = "### ss";
        let keys = Keys::new();
        let res = keys.starts_with_key(line);

        assert_eq!(res, Some("###".to_string()));
    }


    //
    // Scope
    //
    #[test]
    fn valid_scope() {
        let line = "import = [\"test\"]";
        let res = Keys::validate_scope(line);
        assert_eq!(res, Some(vec![(9, 16)]));
    }

    #[test]
    fn invalid_scope_r() {
        let line = "import = [\"test\"";
        let res = Keys::validate_scope(line);
        assert_eq!(res, None);
    }

    #[test]
    fn invalid_scope_l() {
        let line = "import = \"test\"]";
        let res = Keys::validate_scope(line);
        assert_eq!(res, None);
    }

    #[test]
    fn invalid_scope_lr() {
        let line = "import = \"test\"";
        let res = Keys::validate_scope(line);
        assert_eq!(res, None);
    }

    #[test]
    fn invalid_scope_rl() {
        let line = "import = ]asdfasdf[";
        let res = Keys::validate_scope(line);
        assert_eq!(res, None);
    }

    #[test]
    fn valid_heading_scope() {
        let line = "###[somethingEEE = 'd'] Im a smaller x2 heading";
        let res = Keys::validate_scope(line);
        assert_eq!(res, Some(vec![(3, 22)]));
    }

    #[test]
    fn valid_two_scopes() {
        let line = "import = [\"test\"] import = [\"test\"]";
        let res = Keys::validate_scope(line);
        assert_eq!(res, Some(vec![(9, 16), (27, 34)]));
    }

    #[test]
    fn one_valid_one_invalid_scope() {
        let line = "import = [\"test\"] import = [\"test\"";
        let res = Keys::validate_scope(line);
        assert_eq!(res, None);
    }


    //
    // Starts with key and scope
    //
    #[test]
    fn contains_key_and_scope_inline() {
        let line = "###[\"test\"]";
        let keys = Keys::new();
        let res = keys.contains_key_and_scope(line);

        assert_eq!(res, true);
    }

    #[test]
    fn contains_key_and_scope_meta() {
        let line = "@gdd = [\"test\"]";
        let keys = Keys::new();
        let res = keys.contains_key_and_scope(line);

        assert_eq!(res, true);
    }

    #[test]
    fn contains_key_and_scope_invalid() {
        let line = "###\"test\"]";
        let keys = Keys::new();
        let res = keys.contains_key_and_scope(line);

        assert_eq!(res, false);
    }
}
