pub mod file;
pub mod segment;

pub mod decorator;
pub mod paramater;

fn main() {

    // -- Open a test file
    let smd = file::open::open(
        &std::path::Path::new("./examples/headings.smd")
    );

    // -- Print the contents
    println!("{}", smd);
}