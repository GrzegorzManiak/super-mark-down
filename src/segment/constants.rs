use std::collections::HashMap;

pub struct Keys {}
impl Keys {
    pub const IMPORT: &str = "import";
    pub const PROPERTIES: &str = "properties";
    pub const PROPS: &str = "props";

    pub const SCOPE_START: &str = "[";
    pub const SCOPE_END: &str = "]";

    pub const EOL: &str = "\n";
    pub const SPACE: &str = " ";
    pub const TAB: &str = "\t";
    pub const EMPTY: &str = "";

    pub const ESCAPE: &str = "\\";
}
