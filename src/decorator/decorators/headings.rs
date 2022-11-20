use crate::decorator::Decorator;

pub struct H1 {}
impl Decorator for H1 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["#".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}



pub struct H2 {}
impl Decorator for H2 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["##".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}



pub struct H3 {}
impl Decorator for H3 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["###".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}



pub struct H4 {}
impl Decorator for H4 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["####".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}



pub struct H5 {}
impl Decorator for H5 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["#####".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}



pub struct H6 {}
impl Decorator for H6 {
    fn get_decorators(&self) -> Vec<String> {
        vec!["######".to_string()]
    }

    fn parse(&self, text: &str) -> String {
        format!("{}", text)
    }
}