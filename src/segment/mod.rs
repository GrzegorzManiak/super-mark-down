use crate::decorator::Decorator;
use crate::file;
use crate::paramater::Parameter;

pub mod constants;
pub mod segments;
pub mod singlefile;

// 
// Segmenter, This module is responsible for segmenting the SMD source markdown
// into whats text, whats classes etc.
//

pub struct TextSegment {
    pub raw: String,
    pub formated: String,
    pub class: String,

    pub decorators: Vec<Box<dyn Decorator>>,
    pub parameters: Vec<Box<dyn Parameter>>,
}

pub struct ImportSegment {
    pub raw: String,
    pub path: String,
    pub classes: Vec<String>,

    pub decorators: Vec<Box<dyn Decorator>>,
    pub parameters: Vec<Box<dyn Parameter>>,
}

pub struct PropertySegment {
    pub raw: String,
    pub name: String,
    pub properties: Vec<String>,

    pub decorators: Vec<Box<dyn Decorator>>,
    pub parameters: Vec<Box<dyn Parameter>>,
}

pub struct Segments {
    // -- Where we got the segments from
    pub source_path: String,
    pub source: String,

    // -- The segments
    pub imports: Vec<ImportSegment>,
    pub properties: Vec<PropertySegment>,
    pub text: Vec<TextSegment>,
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
            source,
            imports: vec![],
            properties: vec![],
            text: vec![],
        }
    }

    pub fn parse(&mut self) {
        // -- Split the source into lines
        let lines = self.source.split(
            constants::Keys::EOL
        ).collect::<Vec<&str>>();


        // -- Create a buffer, and an output buffer
        let mut buffer = Vec::new();
        let mut output = Vec::new();

        // -- Loop through the lines
        for line in lines {
            // -- Parse the line
            singlefile::parse(
                &mut buffer,
                &mut output,
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
