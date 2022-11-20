use super::Keys;

impl Keys {
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
                    scopes.push((scope_start, i));
                }
            }
        }


        // -- If we have more scopes opened than closed
        // we need to return an error 
        if opened != closed { return None; }
        else if scopes.len() > 0 { return Some(scopes) }
        
        // -- Else return none
        None
    }
}