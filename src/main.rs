extern crate rand;
use rand::Rng;

use segment::Segments;

pub mod file;
pub mod parser;
pub mod segment;
pub mod decorator;
pub mod paramater;

fn main() {
    let mut seg = Segments::new("./examples/headings.smd");

    seg.parse();
} 


pub fn rand_string(
    char_set: &str,
    length: usize
) -> String {
    let mut rng = rand::thread_rng();
    let mut string = String::new();

    for _ in 0..length {
        let index = rng.gen_range(0..char_set.len());
        string.push(char_set.chars().nth(index).unwrap());
    }

    string
}
