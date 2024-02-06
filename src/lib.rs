use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Add(u8),
    Shift(usize),
    Loop(Program),
    PureLoop(PureLoop),
    Zero,
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

#[derive(Debug, Eq, PartialEq)]
pub struct PureLoop {
    adds: HashMap<usize, u8>,
    shift: usize,
}

pub fn pure_loop_effects(program: &Program) -> Option<PureLoop> {
    println!("Loopin {:?}", program);
    let mut pure_loop = PureLoop {
        adds: HashMap::default(),
        shift: 0,
    };
    let mut pointer: usize = 0;
    for op in program.iter() {
        match op {
            Operation::Add(a) => {
                if let Some(b) = pure_loop.adds.get(&pointer) {
                    pure_loop.adds.insert(pointer, a.wrapping_add(*b));
                } else {
                    pure_loop.adds.insert(pointer, *a);
                }
            }
            Operation::Shift(s) => {
                pointer = pointer.wrapping_add(*s);
                pure_loop.shift = pure_loop.shift.wrapping_add(*s);
            }
            // The internal loop might operate on some other index, so its effects on this loop are
            // unknowable (for now)
            Operation::PureLoop(pl) => return None,
            // These all have side affects, making the loop impure
            Operation::Zero => return None,
            Operation::Loop(_) => return None,
            Operation::Read => return None,
            Operation::Write => return None,
        }
    }
    println!("Loopd {:?}", pure_loop);
    if pure_loop.shift == 0 {
        let incr = pure_loop.adds.get(&0);
        if incr.is_none() || incr == Some(&0) {
            panic!("Detected an infinite loop")
        }
    }
    Some(pure_loop)
}

pub fn optimize(program: &mut Program) {
    (0..program.len()).for_each(|i| {
        if let Operation::Loop(l) = &mut program[i] {
            if let Some(pl) = pure_loop_effects(l) {
                if pl.shift == 0
                    && (pl.adds == HashMap::from([(0, 1)]) || pl.adds == HashMap::from([(0, 255)]))
                {
                    program[i] = Operation::Zero;
                } else {
                    program[i] = Operation::PureLoop(pl);
                }
            } else {
                optimize(l);
            }
        }
    });
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
                Operation::PureLoop(pl) => {
                    if pl.shift == 0 {
                        let incr = pl.adds.get(&0).expect("Loop must increment");
                        let expected = 0_u8.wrapping_sub(tape[*pointer]);
                        // TODO something better than this dumb ass O(n) algorithm
                        let mut reps: u8 = 0;
                        while reps.wrapping_mul(*incr) != expected {
                            reps += 1;
                        }
                        for (offset, add) in pl.adds.iter() {
                            let p = pointer.wrapping_add(*offset) % TAPE_SIZE;
                            tape[p] = tape[p].wrapping_add(add.wrapping_mul(reps));
                        }
                    } else {
                        while tape[*pointer] != 0 {
                            for (offset, add) in pl.adds.iter() {
                                let p = pointer.wrapping_add(*offset);
                                tape[p] = tape[p].wrapping_add(*add);
                            }
                            *pointer = pointer.wrapping_add(pl.shift) % TAPE_SIZE;
                        }
                    }
                }
                Operation::Zero => tape[*pointer] = 0,
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
