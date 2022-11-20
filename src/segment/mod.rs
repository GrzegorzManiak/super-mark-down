use crate::decorator::Decorator;
use crate::paramater::Parameter;

pub mod constants;


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