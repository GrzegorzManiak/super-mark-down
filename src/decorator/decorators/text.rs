use crate::decorator::{
    Decorator, 
    Config, 
    DecoratorType, 
    Parameter
};

pub struct Bold {}
impl Decorator for Bold {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::Both,
            decorator: DecoratorType::Wrapper(
                "**".to_string(),
                "**".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Bold {})
    }
}



pub struct Italic {}
impl Decorator for Italic {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
    
    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::Both,
            decorator: DecoratorType::Wrapper(
                "*".to_string(),
                "*".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Italic {})
    }
}



pub struct Underline {}
impl Decorator for Underline {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::Both,
            decorator: DecoratorType::Wrapper(
                "__".to_string(),
                "__".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Underline {})
    }
}



pub struct Strikethrough {}
impl Decorator for Strikethrough {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config {
            allow_params: Parameter::Both,
            decorator: DecoratorType::Wrapper(
                "~~".to_string(),
                "~~".to_string()
            ),
            allow_touching: true
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Strikethrough {})
    }
}





pub struct Image {}
impl Decorator for Image {
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
        Box::new(Image {})
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