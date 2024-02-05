use bf_compiler::{parse, run_program};

fn main() {
    let file = std::env::args().nth(1).expect("Please specify a BF file");
    println!("{file}");
    let source = std::fs::read_to_string(file).expect("Failed to read BF file");
    run_program(&parse(&source));
}
