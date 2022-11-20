pub mod decorators;

use std::collections::HashMap;

use self::decorators::headings;
use self::decorators::text;


pub enum Position { Anywhere, Start, End } // -- Where the decorator can be used

// -- A configuration that a decorator has to implement
// which gives the compiler the information it needs to
// parse the decorator
pub struct Config {
    pub start: Position,
}

impl Config {
    pub fn new() -> Config {
        Config { start: Position::Anywhere }
    }
}


pub trait Decorator {
    fn get_decorators(&self) -> Vec<String>;
    fn parse(&self, text: &str) -> String;
    fn get_config(&self) -> Config;
    fn clone(&self) -> Box<dyn Decorator>;
}


pub fn get_all_decorators() -> HashMap<String, Box<dyn Decorator>> {
    let mut decorators: HashMap<String, Box<dyn Decorator>> = HashMap::new();


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
    add_to_hashmap(&mut decorators, Box::new(text::Code {}));
    add_to_hashmap(&mut decorators, Box::new(text::Link {}));
    add_to_hashmap(&mut decorators, Box::new(text::Image {}));
    

    decorators
}

pub fn add_to_hashmap(
    hashmap: &mut HashMap<String, Box<dyn Decorator>>, 
    decorator: Box<dyn Decorator>) 
{
    let decorators = decorator.get_decorators();

    for decorator_name in decorators {
        hashmap.insert(decorator_name, decorator.clone());
    }
}