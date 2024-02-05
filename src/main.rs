use bf_compiler::{parse, Execution};

fn main() {
    let mut exec = Execution::new(parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."));
    // let mut exec = Execution::new(parse("[]"));
    exec.run()
}
