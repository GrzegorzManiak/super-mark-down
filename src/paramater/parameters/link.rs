use crate::paramater::{
    Parameter, 
    config::Config
};

pub struct Link {
}

impl Parameter for Link {
    fn parse(&self, text: &str) -> String {
        format!("[{}]({})", text, text)
    }

    fn config(&self) -> Config {
        Config {
            name: "link".to_string()
        }
    }

    fn clone(&self) -> Box<dyn Parameter> {
        Box::new(Link {})
    }
}