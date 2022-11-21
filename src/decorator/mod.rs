pub mod decorators;
pub mod config;

use std::collections::HashMap;

use self::decorators::headings;
use self::decorators::meta;
use self::decorators::text;

use self::config::{
    Config,
    DecoratorMap,
    DecoratorType,
    Parameter,
};


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


    // -- Meta
    add_to_hashmap(&mut decorators, Box::new(meta::Class {}));
    add_to_hashmap(&mut decorators, Box::new(meta::Assignment {}));
    

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