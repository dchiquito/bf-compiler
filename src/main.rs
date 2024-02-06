use bf_compiler::{optimize, parse, run_program};

fn main() {
    let file = std::env::args().nth(1).expect("Please specify a BF file");
    println!("{file}");
    let source = std::fs::read_to_string(file).expect("Failed to read BF file");
    let mut program = parse(&source);
    optimize(&mut program);
    run_program(&program);
}
