use crate::decorator::{
    DecoratorMap, 
    get_all_decorators, 
    Parameter
};

pub struct Keys {
    decorators: DecoratorMap,
    scope_keys: Vec<String>,
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
        let mut scope_keys = Vec::new();


        // -- Create a list of all the scope keys
        for decorator in decorators.values() {
            
            // -- Check if the decorator has a scope key
            match decorator.get_config().allow_params {
                Parameter::None => continue,
                _ => {
                    let keys = decorator.get_decorators();
                    for key in keys {
                        scope_keys.push(key);
                    }
                }
            }
        }

        
        Self {
            decorators,
            scope_keys,
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

    pub fn starts_with_key(&self, text: &str) -> 
        Option<String> 
    {
        // -- keys to check by default
        // other keys can be added by decorators
        let mut keys = vec![
            Keys::IMPORT.to_string(),
            Keys::META.to_string()
        ];


        // -- Append the scope keys to the list
        keys.append(&mut self.scope_keys.clone());


        // -- Check if the text starts with any of the keys
        for key in keys {
            //
            // since we are checking if the text starts with a key
            // so we can check for scope, the character right after
            // the key should be:
            // 1: a space / tab     (Not scope)
            // 2: Keys::SCOPE_START (Scope)
            // 3: Keys::ASSIGNMENT  (Not scope)
            //
            // EXCEPTIONS:
            // 1: Meta keys, they can be followed by anything
            //
            // if the character is not one of the above, then it is
            // not this key
            //

            // Lets split the text into two parts
            // The first part is the key
            // The second part is the rest of the text
            
            // Split at:
            // - Keys::SCOPE_START
            // - Keys::ASSIGNMENT
            // - Keys::SPACE
            // - Keys::TAB
            
            if text.starts_with(&key) {
                let split = self.split_at(text, Keys::SCOPE_START);
                let split = self.split_at(&split[0], Keys::ASSIGNMENT);
                let split = self.split_at(&split[0], Keys::SPACE);
                let split = self.split_at(&split[0], Keys::TAB);

                // -- Check if the key is a meta key
                if key == Keys::META {
                    return Some(key);
                }

                // -- Check if the key is a scope key
                if split[0] == key {
                    return Some(key);
                }
            }

            
        }


        // -- Return none
        None
    }

    pub fn starts_with_key_and_scope(&self, text: &str) -> 
        bool
    {
        // -- Check if its a meta key
        if text.starts_with(Keys::META) {
            return true;
        }

        // -- Check if the text starts with any of the keys
        match self.starts_with_key(text) {
            None => return false,
            Some(key) => {
                // -- Check if the key is imeediately 
                // followed by a scope opening
                if text.starts_with(&format!("{}{}", key, Keys::SCOPE_START)) {
                    // -- Return the key and scope
                    return true;
                }
            }
        }

        // -- Return none
        false
    }

    pub fn validate_scope (
        line: &str,
    ) -> Option<(usize, usize)> 
    {
        //
        // We need to make sure that the scope is valid
        // we also need to account for the fact that a parameter
        // in the scope might contain a scope opening or closing 
        // character, and it might be escaped.
        //

        // -- The scope start should be the first 
        // Key::SCOPE_START character
        let scope_start = match line.find(Keys::SCOPE_START) {
            Some(index) => index,
            None => return None,
        };


        // -- The scope end should be the last
        // Key::SCOPE_END character
        let scope_end = match line.rfind(Keys::SCOPE_END) {
            Some(index) => index,
            None => return None,
        };


        // -- Check if the scope is valid
        if scope_start > scope_end { return None; }


        // -- Return the scope start and end positions
        Some((scope_start, scope_end))
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    //
    // Keys::IMPORT,
    // Keys::PROPERTIES,
    // Keys::PROPS,
    // Keys::META
    //
    #[test]
    fn test_starts_with_import() {
        let line = "import = \"test\"";
        let keys = Keys::new();
        let res = keys.starts_with_key(line);

        assert_eq!(res, Some(Keys::IMPORT.to_string()));
    }

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
        assert_eq!(res, Some((9, 16)));
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
        assert_eq!(res, Some((3, 22)));
    }


    //
    // Starts with key and scope
    //
    #[test]
    fn starts_with_key_and_scope_inline() {
        let line = "###[\"test\"]";
        let keys = Keys::new();
        let res = keys.starts_with_key_and_scope(line);

        assert_eq!(res, true);
    }

    #[test]
    fn starts_with_key_and_scope_meta() {
        let line = "@gdd = [\"test\"]";
        let keys = Keys::new();
        let res = keys.starts_with_key_and_scope(line);

        assert_eq!(res, true);
    }

    #[test]
    fn starts_with_key_and_scope_invalid() {
        let line = "###\"test\"]";
        let keys = Keys::new();
        let res = keys.starts_with_key_and_scope(line);

        assert_eq!(res, false);
    }
}
