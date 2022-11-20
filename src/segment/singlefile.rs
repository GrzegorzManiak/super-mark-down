use super::constants::Keys;



pub fn parse(
    buffer: &mut Vec<String>,
    output: &mut Vec<String>,
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

            match Keys::starts_with_key(line) {
                Some(_key) => {
                    // -- If the line starts with a key word, we add it to the buffer
                    buffer.push(line.to_string());
                }

                None => {
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
            match Keys::starts_with_key(&full_line) {
                Some(_key) => {
                    match Keys::validate_scope(&full_line) {
                        Some((_scope_start, scope_end)) => {
                            // -- Split the full line at the end of the scope
                            let split = full_line.split_at(scope_end);
        
                            // -- Add the first part of the split to the output
                            output.push(split.0.to_string());
        
                            // -- Set the buffer to the second part of the split
                            buffer.clear();
                           
                            // -- Check if the second part has any content
                            if split.1.len() > 0 {
                                buffer.push(split.1.to_string());
                            }
                        }

                        None => {}
                    }
                }

                None => {
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
            match Keys::starts_with_key(&combined) {

                // THIS IS a key word
                Some(_key) => {
                    match Keys::validate_scope(&combined) {
                        Some((_scope_start, scope_end)) => {
                            
                            // -- Split the combined string at the end of the scope
                            // we add one, as we want to include the scope end character
                            let split = combined.split_at(scope_end + 1);
        
                            // -- Add the first part of the split to the output
                            output.push(split.0.to_string());
        

                            // -- Clear the buffer and Set the buffer to the 
                            // second part of the split if it has any content               
                            buffer.clear();
                            if split.1.len() > 0 {
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
                None => {
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