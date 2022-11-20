use super::constants::Keys;



pub fn parse(
    buffer: &mut Vec<String>,
    output: &mut Vec<String>,
    keys: &Keys,
    line: &str,
) {
    // 
    // This function is responsible for making sure that each line
    // contains all the data for it to be parsed correctly.
    //
    // aka, If a we have a class definition that spans multiple lines
    // we want to make sure that we have all the lines in one string
    // so that we can parse it correctly.
    //
    // eg:
    // @class_name = [
    //  some_property = "some_value",  
    // ]
    //
    // to:
    // @class_name=[some_property="some_value",]
    //


    match buffer.len() {
        0 => {
            // -- If the buffer is empty, we start from scratch
            // By checking if the line starts with a key word

            match keys.starts_with_key_and_scope(line) {
                true => {
                    // -- If the line starts with a key word, we add it to the buffer
                    buffer.push(line.to_string());
                }

                false => {
                    // -- If the line doesn't start with a key word, we add it to the output
                    output.push(line.to_string());

                    // -- We also need to make sure that the buffer is empty
                    buffer.clear();
                    return;
                }
            }



            // -- Lets attempt to clean the buffer
            let full_line = buffer.join("");

            // -- Check if the full line is valid
            match keys.starts_with_key_and_scope(&full_line) {
                true => {
                    match Keys::validate_scope(&full_line) {
                        Some((_scope_start, scope_end)) => {
                            //
                            // -- If the scope is valid, we add it to the output
                            // and clear the buffer, if the key is a meta key
                            // we split the line into two parts on the scope end
                            // if the key is not a meta key, we don't split the line
                            //
                            let split = match keys.is_meta(&full_line) {
                                true => full_line.split_at(scope_end + 1),
                                false => (full_line.as_str(), ""),
                            };

                            output.push(split.0.to_string());
                            buffer.clear();

                            // -- Check if the split.2 has any data
                            if split.1.len() > 0 {
                                // -- If it does, we add it to the buffer
                                buffer.push(split.1.to_string());
                            }
                        }

                        None => {}
                    }
                }

                false => {
                    // -- If the full line dose not start with a key word, 
                    // we add it to the output
                    output.push(full_line);

                    return;
                }
            }
        }



        
        _ => {
            // -- Combine the buffer
            let mut combined = buffer.join("");

            // -- Add the current line to the combined string
            combined.push_str(line);

            // -- Check if the combined string is valid
            match keys.starts_with_key_and_scope(&combined) {

                // THIS IS a key word
                true => {
                    match Keys::validate_scope(&combined) {
                        Some((_scope_start, scope_end)) => {
                            //
                            // Same as above, if the scope is valid, we add it to the output
                            // and clear the buffer, if the key is a meta key
                            // we split the line into two parts on the scope end
                            // if the key is not a meta key, we don't split the line
                            //
                            let split = match keys.is_meta(&combined) {
                                true => combined.split_at(scope_end + 1),
                                false => (combined.as_str(), ""),
                            };

                            output.push(split.0.to_string());
                            buffer.clear();

                            // -- Check if the split.2 has any data
                            if split.1.len() > 0 {
                                // -- If it does, we add it to the buffer
                                buffer.push(split.1.to_string());
                            }
                        }
        
                        None => {
                            // -- If the combined string is not valid, we add it to the buffer
                            buffer.push(line.to_string());
                        }
                    }
                }


                // THIS IS NOT a key word
                false => {
                    // -- If the combined string dose not start with a key word, 
                    // we add it to the output
                    output.push(combined);

                    // -- Clear the buffer
                    buffer.clear();
                }
            }
        }
    }
}