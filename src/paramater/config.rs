use std::collections::HashMap;
use super::Parameter;

pub type ParamaterMap = HashMap<String, Box<dyn Parameter>>;

pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Config {
        Config {   
            name
        }
    }
}