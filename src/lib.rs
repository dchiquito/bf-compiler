#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Add(u8),
    Shift(usize),
    LoopStart(usize),
    LoopEnd(usize),
    Read,
    Write,
}

type Program = Vec<Operation>;

pub fn parse(source: &str) -> Program {
    let mut program = vec![];
    let mut add = 0;
    let mut shift = 0;
    for symbol in source.chars() {
        match symbol {
            // Anything but + and -
            '<' | '>' | '[' | ']' | ',' | '.' => {
                if add != 0 {
                    program.push(Operation::Add(add));
                    add = 0;
                }
            }
            _ => (),
        }
        match symbol {
            // Anything but < and >
            '+' | '-' | '[' | ']' | ',' | '.' => {
                if shift != 0 {
                    program.push(Operation::Shift(shift));
                    shift = 0;
                }
            }
            _ => (),
        }
        match symbol {
            '+' => add = add.wrapping_add(1),
            '-' => add = add.wrapping_add(u8::MAX),
            '<' => shift = shift.wrapping_add(usize::MAX),
            '>' => shift = shift.wrapping_add(1),
            '[' => program.push(Operation::LoopStart(0)),
            ']' => program.push(Operation::LoopEnd(0)),
            ',' => program.push(Operation::Read),
            '.' => program.push(Operation::Write),
            _ => (),
        }
    }
    if add != 0 {
        program.push(Operation::Add(add))
    }
    if shift != 0 {
        program.push(Operation::Shift(shift))
    }
    // Fill out the loop pointers, now that everything is parsed out and grouped
    let mut stack = vec![];
    for i in 0..program.len() {
        if let Operation::LoopStart(_) = program[i] {
            stack.push(i);
        }
        if let Operation::LoopEnd(_) = program[i] {
            let start = stack.pop().expect("Unmatched ]");
            let end = i;
            program[start] = Operation::LoopStart(end);
            program[end] = Operation::LoopEnd(start);
        }
    }
    if !stack.is_empty() {
        panic!("Unmatch [");
    }
    program
}

const TAPE_SIZE: usize = 100_000;

pub struct Execution {
    tape: [u8; TAPE_SIZE],
    pointer: usize,
    program: Program,
    pc: usize,
}
impl Execution {
    pub fn new(program: Program) -> Execution {
        Execution {
            tape: [0; TAPE_SIZE],
            pointer: 0,
            program,
            pc: 0,
        }
    }
    pub fn step(&mut self) {
        match self.program[self.pc] {
            Operation::Add(a) => self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(a),
            Operation::Shift(s) => self.pointer = self.pointer.wrapping_add(s) % TAPE_SIZE,
            Operation::LoopStart(end) => {
                if self.tape[self.pointer] == 0 {
                    self.pc = end;
                }
            }
            Operation::LoopEnd(start) => {
                if self.tape[self.pointer] != 0 {
                    self.pc = start;
                }
            }
            Operation::Read => todo!("Implement reading"),
            Operation::Write => print!("{}", self.tape[self.pointer] as char),
        }
        self.pc += 1;
    }
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }
}

