#[cfg(test)]
mod tests {
    use crate::segment::keywords::{
        Keys,
        KeyLocation
    };

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

    #[test]
    fn contains_meta_valid() {
        let line = "@config = [asdasd]";
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
    // identify_keys
    //
    #[test]
    fn identify_keys() {
        let line = "@class ss";
        let keys = Keys::new();

        let mut out: Vec<KeyLocation> = Vec::new();
        out.push((
            "@class".to_string(),
            0, 3
        ));
        let res = keys.identify_keys(line);

        assert_eq!(res, out);
    }

}
