use std::{path, io::Read};

pub fn open(
    path: &path::Path
) -> String {
    
    // -- Check if the file exists
    if !path.exists() {
        panic!("File does not exist");
    }

    // -- Check if the file is a file
    if !path.is_file() {
        panic!("Path is not a file");
    }

    // -- Open the file
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Could not open file: {}", error)
    };

    // -- Read the file
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(error) => panic!("Could not read file: {}", error)
    };

    // -- return the contents
    contents
}