#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Add(u8),
    Shift(usize),
    Loop(Program),
    Read,
    Write,
}

type Program = Vec<Operation>;

pub fn parse(source: &str) -> Program {
    let (program, i) = parse_loop(&source.chars().collect::<Vec<char>>(), false);
    if i < source.len() {
        panic!("Unmatched ]");
    }
    program
}

pub fn parse_loop(source: &[char], actual_loop: bool) -> (Program, usize) {
    let mut program = vec![];
    let mut add = 0;
    let mut shift = 0;
    let mut i = 0;
    while i < source.len() {
        let symbol = source[i];
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
            '[' => {
                let (the_loop, new_i) = parse_loop(&source[i + 1..], true);
                i += 1 + new_i;
                program.push(Operation::Loop(the_loop));
            }
            ']' => break,
            ',' => program.push(Operation::Read),
            '.' => program.push(Operation::Write),
            _ => (),
        };
        i += 1;
    }
    if actual_loop && i == source.len() {
        panic!("Unmatched [");
    }
    if add != 0 {
        program.push(Operation::Add(add))
    }
    if shift != 0 {
        program.push(Operation::Shift(shift))
    }
    (program, i)
}

const TAPE_SIZE: usize = 100_000;

pub fn run_program(program: &Program) {
    let mut tape = [0; TAPE_SIZE];
    let mut pointer = 0;
    run_loop(program, &mut tape, &mut pointer, false);
}
pub fn run_loop(program: &Program, tape: &mut [u8], pointer: &mut usize, actually_loop: bool) {
    while (!actually_loop) || tape[*pointer] != 0 {
        for op in program.iter() {
            match op {
                Operation::Add(a) => tape[*pointer] = tape[*pointer].wrapping_add(*a),
                Operation::Shift(s) => *pointer = pointer.wrapping_add(*s) % TAPE_SIZE,
                Operation::Loop(the_loop) => run_loop(the_loop, tape, pointer, true),
                Operation::Read => todo!("Implement reading"),
                Operation::Write => print!("{}", tape[*pointer] as char),
            }
        }
        if !actually_loop {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!(parse("+"), vec![Operation::Add(1)]);
        assert_eq!(parse("+++"), vec![Operation::Add(3)]);
        assert_eq!(parse("-"), vec![Operation::Add(255)]);
        assert_eq!(parse("---"), vec![Operation::Add(253)]);
        assert_eq!(parse("<"), vec![Operation::Shift(usize::MAX)]);
        assert_eq!(parse("<<<"), vec![Operation::Shift(usize::MAX - 2)]);
        assert_eq!(parse(">"), vec![Operation::Shift(1)]);
        assert_eq!(parse(">>>"), vec![Operation::Shift(3)]);
        assert_eq!(parse("[+]"), vec![Operation::Loop(vec![Operation::Add(1)])]);
    }
    #[test]
    #[should_panic(expected = "Unmatched [")]
    fn test_unmatched_start_of_loop() {
        parse("[");
    }
    #[test]
    #[should_panic(expected = "Unmatched ]")]
    fn test_unmatched_end_of_loop() {
        parse("]");
    }
}
