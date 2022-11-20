use crate::decorator::{Decorator, Config};

pub struct Bold {}
impl Decorator for Bold {
    fn get_decorators(&self) -> Vec<String> {
        vec!["**".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Bold {})
    }
}



pub struct Italic {}
impl Decorator for Italic {
    fn get_decorators(&self) -> Vec<String> {
        vec!["*".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
    
    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Italic {})
    }
}



pub struct Underline {}
impl Decorator for Underline {
    fn get_decorators(&self) -> Vec<String> {
        vec!["__".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Underline {})
    }
}



pub struct Strikethrough {}
impl Decorator for Strikethrough {
    fn get_decorators(&self) -> Vec<String> {
        vec!["~~".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Strikethrough {})
    }
}



pub struct Code {}
impl Decorator for Code {
    fn get_decorators(&self) -> Vec<String> {
        vec!["`".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Code {})
    }
}



pub struct CodeBlock {}
impl Decorator for CodeBlock {
    fn get_decorators(&self) -> Vec<String> {
        vec!["```".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(CodeBlock {})
    }
}



pub struct Link {}
impl Decorator for Link {
    fn get_decorators(&self) -> Vec<String> {
        vec!["[]()".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Link {})
    }
}



pub struct Image {}
impl Decorator for Image {
    fn get_decorators(&self) -> Vec<String> {
        vec!["![]()".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }

    fn get_config(&self) -> Config {
        Config::new()
    }

    fn clone(&self) -> Box<dyn Decorator> {
        Box::new(Image {})
    }
}