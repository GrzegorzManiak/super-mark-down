pub mod decorators;

use std::collections::HashMap;

use self::decorators::headings;
use self::decorators::text;

pub type DecoratorMap = HashMap<String, Box<dyn Decorator>>;
pub enum Parameter { Inline, Class, Both, None } // -- What parameters the decorator can have
pub enum DecoratorType { Wrapper(String, String), Single(String) }

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



pub trait Decorator {
    fn parse(&self, text: &str) -> String;
    fn get_config(&self) -> Config;
    fn clone(&self) -> Box<dyn Decorator>;
}


pub fn get_all_decorators() -> DecoratorMap {
    let mut decorators: DecoratorMap = HashMap::new();


    // -- Headings
    add_to_hashmap(&mut decorators, Box::new(headings::H1 {}));
    add_to_hashmap(&mut decorators, Box::new(headings::H2 {}));
    add_to_hashmap(&mut decorators, Box::new(headings::H3 {}));
    add_to_hashmap(&mut decorators, Box::new(headings::H4 {}));
    add_to_hashmap(&mut decorators, Box::new(headings::H5 {}));
    add_to_hashmap(&mut decorators, Box::new(headings::H6 {}));

    // -- Text
    add_to_hashmap(&mut decorators, Box::new(text::Bold {}));
    add_to_hashmap(&mut decorators, Box::new(text::Italic {}));
    add_to_hashmap(&mut decorators, Box::new(text::Underline {}));
    add_to_hashmap(&mut decorators, Box::new(text::Strikethrough {}));
    add_to_hashmap(&mut decorators, Box::new(text::Image {}));
    

    decorators
}

pub fn add_to_hashmap(
    hashmap: &mut DecoratorMap,
    decorator: Box<dyn Decorator>) 
{
    match decorator.get_config().decorator {
        DecoratorType::Wrapper(start, _) => {
            hashmap.insert(start, decorator);
        }

        DecoratorType::Single(name) => {
            hashmap.insert(name, decorator);
        }
    }
        
}