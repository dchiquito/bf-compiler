use bf_compiler::{parse, Execution};

fn main() {
    let file = std::env::args().nth(1).expect("Please specify a BF file");
    println!("{file}");
    let source = std::fs::read_to_string(file).expect("Failed to read BF file");
    let mut exec = Execution::new(parse(&source));
    exec.run()
}
