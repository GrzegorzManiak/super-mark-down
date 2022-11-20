
pub struct Keys {}
impl Keys {
    pub const IMPORT: &str = "import";
    pub const PROPERTIES: &str = "properties";
    pub const PROPS: &str = "props";

    pub const SCOPE_START: &str = "[";
    pub const SCOPE_END: &str = "]";

    pub const ASSIGNMENT: &str = "=";
    pub const META: &str = "@";
    pub const SEPARATOR: &str = ",";

    pub const EOL: &str = "\n";
    pub const SPACE: &str = " ";
    pub const TAB: &str = "\t";
    pub const EMPTY: &str = "";

    pub const ESCAPE: &str = "\\";



    pub fn starts_with_key(text: &str) -> 
        Option<&str> 
    {
        // -- keys
        let keys = vec![
            Keys::IMPORT,
            Keys::PROPERTIES,
            Keys::PROPS,
            Keys::META
        ];

        // -- Loop through the keys
        for key in keys {
            // -- Check if the text starts with the key
            if text.starts_with(key) {
                return Some(key);
            }
        }

        // -- Return none
        None
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
        let res = Keys::starts_with_key(line);

        assert_eq!(res, Some(Keys::IMPORT));
    }

    #[test]
    fn test_starts_with_properties() {
        let line = "properties = \"test\"";
        let res = Keys::starts_with_key(line);

        assert_eq!(res, Some(Keys::PROPERTIES));
    }

    #[test]
    fn test_starts_with_props() {
        let line = "props = \"test\"";
        let res = Keys::starts_with_key(line);

        assert_eq!(res, Some(Keys::PROPS));
    }

    #[test]
    fn test_starts_with_meta() {
        let line = "@test = \"test\"";
        let res = Keys::starts_with_key(line);

        assert_eq!(res, Some(Keys::META));
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
}
