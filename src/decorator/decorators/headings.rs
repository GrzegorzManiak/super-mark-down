use crate::decorator::{
    Decorator, 
    Config, 
    Parameter, 
    DecoratorType
};


pub struct H1 {}
impl Decorator for H1 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "#".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H1 {})
    }
}



pub struct H2 {}
impl Decorator for H2 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "##".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H2 {})
    }
}



pub struct H3 {}
impl Decorator for H3 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "###".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H3 {})
    }
}



pub struct H4 {}
impl Decorator for H4 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "####".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H4 {})
    }
}



pub struct H5 {}
impl Decorator for H5 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "#####".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H5 {})
    }
}



pub struct H6 {}
impl Decorator for H6 {
    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config { 
            allow_params: Parameter::Both,
            decorator: DecoratorType::Single(
                "######".to_string()
            ),
            allow_touching: false
        }
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(H6 {})
    }
}