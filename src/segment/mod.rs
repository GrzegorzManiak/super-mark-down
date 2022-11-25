use crate::file;
use self::keywords::Keys;

pub mod keywords;
pub mod singlefile;

// 
// Segmenter, This module is responsible for segmenting the SMD source markdown
// into whats text, whats classes etc.
//
pub struct Segments {
    // -- Where we got the segments from
    pub source_path: String,
    pub source: String,
}

impl Segments {
    pub fn new(source_path: &str) -> Self {

        // -- Open the file
        let source = file::open::open(
            &std::path::Path::new(source_path)
        );

        // -- Create the segments
        Self {
            source_path: source_path.to_string(),
            source
        }
    }

    pub fn parse(&mut self) {
        // -- Split the source into lines
        let lines = self.source.split(
            keywords::Keys::EOL
        ).collect::<Vec<&str>>();


        // -- Create a buffer, and an output buffer
        let mut buffer = Vec::new();
        let mut output = Vec::new();
        let keys = Keys::new();


        // -- Loop through the lines
        for line in lines {
            // -- Parse the line
            singlefile::parse(
                &mut buffer,
                &mut output,
                &keys,
                line
            );
        }


        // -- Loop through the output
        for segment in output {
            println!("{:?}", segment);
        }
    }

    pub fn log(&self) {
        println!("
            {}
        ", self.source);
    }
}