use super::keywords::Keys;

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

    // -- Create a combined buffer
    let combined_buffer = buffer.join("") + line;


    // -- Check if the line contains the start of a scope
    match keys.contains_scope(&combined_buffer) 
    {
        true => {
            // -- Check if we have any scopes
            match Keys::validate_scope(&combined_buffer) {
                Some(_scopes) => {
                    // -- Add the output to the output buffer
                    output.push(combined_buffer);

                    // -- Clear the buffer
                    buffer.clear();
                }


                None => {
                    // -- we have a started scope, but no closing scope
                    // so we need to add the line to the buffer
                    buffer.push(line.to_string());
                }
            }
        }


        false => {
            // -- Add the line to output
            output.push(line.to_string());
            
            // -- and clear the buffer
            buffer.clear();
        }
    }
}