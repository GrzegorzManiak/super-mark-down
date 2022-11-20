pub trait Parameter {
    fn get_parameter(&self) -> Vec<String>;
    fn parse(&self, text: &str) -> String;
}
