pub mod parameters;
pub mod config;

use std::collections::HashMap;
use self::config::{
    Config,
    ParamaterMap
};


pub trait Parameter {
    fn config(&self) -> Config;
    fn parse(&self, text: &str) -> String;
    fn clone(&self) -> Box<dyn Parameter>;
}

pub fn get_all_parameters() -> ParamaterMap {
    let mut parameters: ParamaterMap = HashMap::new();

    add_to_hashmap(&mut parameters, Box::new(parameters::link::Link {}));

    parameters
}

pub fn add_to_hashmap(
    hashmap: &mut ParamaterMap,
    parameter: Box<dyn Parameter>) 
{
    hashmap.insert(parameter.config().name, parameter);  
}