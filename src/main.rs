use segment::Segments;

pub mod file;
pub mod segment;

pub mod decorator;
pub mod paramater;

fn main() {
    let mut seg = Segments::new("./examples/headings.smd");

    seg.parse();
} 