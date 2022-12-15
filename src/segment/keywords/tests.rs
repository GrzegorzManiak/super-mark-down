#[cfg(test)]
mod tests {
    use crate::{segment::keywords::{
        Keys,
        KeyLocation
    }, decorator::config::DecoratorType};
    // 
    // Naming convention:
    // function_name__test_name
    // 



    //
    // Key extrapolation
    // 
    #[test]
    fn key_extrapolation__single_key() {
        let line = "im just < some >[] text";
        let keys = Keys::new();
        let res = keys.extrapolate_keys(line);

        assert_eq!(res.keys.len(), 1);
    }

    #[test]
    fn key_extrapolation__multiple_keys() {
        let line = "im just < some > *text*";
        let keys = Keys::new();
        let res = keys.extrapolate_keys(line);

        assert_eq!(res.keys.len(), 2);
    }

    #[test]
    fn key_extrapolation__semi_nested_keys() {
        let line = "im just < some > *text* < some >";
        let keys = Keys::new();
        let res = keys.extrapolate_keys(line);

        assert_eq!(res.keys.len(), 3);
    }

    #[test]
    fn key_extrapolation__nested_keys() {
        let line = "im just < some *text* some >";
        let keys = Keys::new();
        let res = keys.extrapolate_keys(line);

        assert_eq!(res.keys.len(), 2);
    }
    


    //
    // Escape
    //
    #[test]
    fn check_escaped__escaped() {
        let line = "im \\escaped text";
        let key = "escaped";
        let index = 0;
        let keys = Keys::new();
        let res = keys.check_escaped(line, key, index);

        assert_eq!(res, true);
    }

    #[test]
    fn check_escaped__not_escaped() {
        let line = "im escaped text";
        let key = "escaped";
        let index = 0;
        let keys = Keys::new();
        let res = keys.check_escaped(line, key, index);

        assert_eq!(res, false);
    }

    #[test]
    fn check_escaped__escaped_escape() {
        let line = "im \\\\escaped text";
        let key = "escaped";
        let index = 0;
        let keys = Keys::new();
        let res = keys.check_escaped(line, key, index);

        assert_eq!(res, false);
    }

    #[test]
    fn check_escaped__start_of_line() {
        let line = "\\escaped text";
        let key = "escaped";
        let index = 0;
        let keys = Keys::new();
        let res = keys.check_escaped(line, key, index);

        assert_eq!(res, true);
    }

    #[test]
    fn check_escaped__end_of_line() {
        let line = "escaped text\\";
        let key = "escaped";
        let index = 0;
        let keys = Keys::new();
        let res = keys.check_escaped(line, key, index);

        assert_eq!(res, false);
    }

    #[test]
    fn check_escaped__multiple_same_keys() {
        let line = "im \\escaped \\escaped text";
        let key = "escaped";

        let keys = Keys::new();

        let res1 = keys.check_escaped(line, key, 0);
        let res2 = keys.check_escaped(line, key, 1);

        assert_eq!(res1, true);
        assert_eq!(res2, true);
    }

    #[test]
    fn check_escaped__multiple_different_keys() {
        let line = "im \\escaped \\other text";
        let key1 = "escaped";
        let key2 = "other";

        let keys = Keys::new();

        let res1 = keys.check_escaped(line, key1, 0);
        let res2 = keys.check_escaped(line, key2, 0);

        assert_eq!(res1, true);
        assert_eq!(res2, true);
    }


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

        assert_eq!(res, true);
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
    fn identify_keys_single() {
        let line = "@class ss";
        let keys = Keys::new();

        let mut out: Vec<KeyLocation> = Vec::new();
        let dec_type = DecoratorType::Single("@class".to_string());

        out.push(keys.new_keylocation(dec_type, 0, 5));
        let res = keys.identify_keys(line.to_string());

        
        // -- Assert
        for (_i, out_key) in out.iter().enumerate() {
            for (_j, res_key) in res.iter().enumerate() {
                let out = (out_key.start, out_key.end);
                let res = (res_key.start, res_key.end);

                assert_eq!(out, res);
            }
        }
    }

    #[test]
    fn identify_keys_wrapper() {
        let line = "some *text*";
        let keys = Keys::new();

        let mut exp: Vec<KeyLocation> = Vec::new();
        let dec_type = DecoratorType::Wrapper(
            "*".to_string(), 
            "*".to_string()
        );

        exp.push(
            keys.new_keylocation(dec_type, 5, 10)
        );
        let res = keys.identify_keys(line.to_string());


        // -- Assert
        for (_i, exp_key) in exp.iter().enumerate() {
            for (_j, res_key) in res.iter().enumerate() {
                let exp = (exp_key.start, exp_key.end);
                let res = (res_key.start, res_key.end);

                assert_eq!(res, exp);
            }
        }
    }


    #[test]
    fn identify_multiple_key_wrappers() {
        let line = "some *text* and <more>";
        let keys = Keys::new();

        let mut exp: Vec<KeyLocation> = Vec::new();

        let bold = DecoratorType::Wrapper("*".to_string(), "*".to_string());
        let sele = DecoratorType::Wrapper("<".to_string(), ">".to_string());

        exp.push(keys.new_keylocation(bold, 5, 10));
        exp.push(keys.new_keylocation(sele, 16, 21));

        let res = keys.identify_keys(line.to_string());

        
        // -- Assert
        for (i, exp_key) in exp.iter().enumerate() {
            let res_key = &res[i];

            let exp = (i, exp_key.start, exp_key.end);
            let res = (i, res_key.start, res_key.end);

            assert_eq!(res, exp);
        }
    }


    #[test]
    fn identify_multiple_same_wrappers() {
        let line = "some *text* and *more*";
        let keys = Keys::new();

        let mut exp: Vec<KeyLocation> = Vec::new();

        let bold1 = DecoratorType::Wrapper("*".to_string(), "*".to_string());
        let bold2 = DecoratorType::Wrapper("*".to_string(), "*".to_string());

        exp.push(keys.new_keylocation(bold1, 5, 10));
        exp.push(keys.new_keylocation(bold2, 16, 21));

        let res = keys.identify_keys(line.to_string());

        
        // -- Assert
        for (i, exp_key) in exp.iter().enumerate() {
            let res_key = &res[i];

            let exp = (i, exp_key.start, exp_key.end);
            let res = (i, res_key.start, res_key.end);

            assert_eq!(res, exp);
        }
    }
}
