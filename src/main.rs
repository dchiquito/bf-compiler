use bf_compiler::impl1::{parse, Execution};

fn main() {
    let mut exec = Execution::new(parse("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."));
    exec.run()
}
