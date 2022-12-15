use std::collections::HashMap;
use super::Decorator;

pub type DecoratorMap = HashMap<String, Box<dyn Decorator>>;
pub enum Parameter { Inline, Class, Both, None } // -- What parameters the decorator can have
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DecoratorType { Wrapper(String, String), Single(String) }
impl DecoratorType {
    pub fn clone(&self) -> Self {
        match self {
            DecoratorType::Wrapper(start, end) => DecoratorType::Wrapper(start.clone(), end.clone()),
            DecoratorType::Single(text) => DecoratorType::Single(text.clone())
        }
    }
}

// -- A configuration that a decorator has to implement
// which gives the compiler the information it needs to
// parse the decorator
pub struct Config {
    pub allow_params: Parameter,  // -- can the decorator have parameters and where
    pub decorator: DecoratorType, // -- The decorator type
    pub allow_touching: bool      // -- Can the decorator be touching the text
}


impl Config {
    pub fn new(wrapper: DecoratorType) -> Config {
        Config { 
            allow_params: Parameter::Both, // -- Should this decorator be able to have parameters?
            decorator: wrapper,            // -- Whether this decorator is a wrapper, eg **bold**
            allow_touching: false          // -- Whether this decorator can be touching the text
        }
    }
}