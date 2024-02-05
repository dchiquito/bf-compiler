#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    LoopStart,
    LoopEnd,
    Read,
    Write,
}

impl TryInto<Symbol> for &char {
    type Error = ();

    fn try_into(self) -> Result<Symbol, Self::Error> {
        match self {
            '+' => Ok(Symbol::Increment),
            '-' => Ok(Symbol::Decrement),
            '<' => Ok(Symbol::MoveLeft),
            '>' => Ok(Symbol::MoveRight),
            '[' => Ok(Symbol::LoopStart),
            ']' => Ok(Symbol::LoopEnd),
            ',' => Ok(Symbol::Read),
            '.' => Ok(Symbol::Write),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Add(u8),
    Shift(usize),
    LoopStart(usize),
    LoopEnd(usize),
    Read,
    Write,
}

type Program = Vec<Symbol>;

pub fn parse(source: &str) -> Program {
    source
        .chars()
        .filter_map(|c| TryInto::<Symbol>::try_into(&c).ok())
        .collect()
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
            Symbol::Increment => self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1),
            Symbol::Decrement => {
                self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(u8::MAX)
            }
            Symbol::MoveLeft => self.pointer = (self.pointer + TAPE_SIZE - 1) % TAPE_SIZE,
            Symbol::MoveRight => self.pointer = (self.pointer + 1) % TAPE_SIZE,
            Symbol::LoopStart => {
                if self.tape[self.pointer] == 0 {
                    let mut nesting = 0;
                    for i in self.pc..self.program.len() {
                        match self.program[i] {
                            Symbol::LoopStart => nesting += 1,
                            Symbol::LoopEnd => {
                                nesting -= 1;
                                if nesting == 0 {
                                    self.pc = i;
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                    if nesting != 0 {
                        panic!("Unmatched [")
                    }
                }
            }
            Symbol::LoopEnd => {
                if self.tape[self.pointer] != 0 {
                    let mut nesting = 0;
                    for i in (0..=self.pc).rev() {
                        match self.program[i] {
                            Symbol::LoopEnd => nesting += 1,
                            Symbol::LoopStart => {
                                nesting -= 1;
                                if nesting == 0 {
                                    self.pc = i;
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                    if nesting != 0 {
                        panic!("Unmatched ]")
                    }
                }
            }
            Symbol::Read => todo!("Implement reading"),
            Symbol::Write => print!("{}", self.tape[self.pointer] as char),
        }
        self.pc += 1;
    }
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step();
        }
    }
}

