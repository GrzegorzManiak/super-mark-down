use crate::decorator::{
    Decorator, 
    Config, 
    DecoratorType, 
    Parameter
};

pub struct Class {
    
}
impl Decorator for Class {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::None,
            decorator: DecoratorType::Single("@class".to_string()),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Class {})
    }
}

pub struct Assignment {}
impl Decorator for Assignment {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::None,
            decorator: DecoratorType::Wrapper(
                "=".to_string(),
                "".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Assignment {})
    }
}

pub struct Selector {}
impl Decorator for Selector {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::Both,
            decorator: DecoratorType::Wrapper(
                "<".to_string(),
                ">".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Selector {})
    }
}