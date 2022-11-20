pub mod decorators;

use self::decorators::headings;
use self::decorators::text;

pub trait Decorator {
    fn get_decorators(&self) -> Vec<String>;
    fn parse(&self, text: &str) -> String;
}

pub fn get_all_decorators() -> Vec<Box<dyn Decorator>> {
    vec![
        // -- Headings
        Box::new(headings::H1 {}),
        Box::new(headings::H2 {}),
        Box::new(headings::H3 {}),
        Box::new(headings::H4 {}),
        Box::new(headings::H5 {}),

        // -- Text
        Box::new(text::Bold {}),
        Box::new(text::Italic {}),
        Box::new(text::Underline {}),
        Box::new(text::Strikethrough {}),
        Box::new(text::Code {}),
        Box::new(text::CodeBlock {}),
        Box::new(text::Link {}),
        Box::new(text::Image {}),
    ]
}