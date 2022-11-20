use crate::decorator::Decorator;

pub struct Bold {}
impl Decorator for Bold {
    fn get_decorators(&self) -> Vec<String> {
        vec!["**".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
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
}



pub struct Underline {}
impl Decorator for Underline {
    fn get_decorators(&self) -> Vec<String> {
        vec!["__".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
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
}



pub struct Code {}
impl Decorator for Code {
    fn get_decorators(&self) -> Vec<String> {
        vec!["`".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
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
}



pub struct Link {}
impl Decorator for Link {
    fn get_decorators(&self) -> Vec<String> {
        vec!["[]()".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
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
}