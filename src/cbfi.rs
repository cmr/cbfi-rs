use std::fs::File;
use std::io::*;
use std::env;

use Op::*;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Op {
    Left,
    Right,
    Inc,
    Dec,
    Out,
    In,
    FJump,
    BJump
}

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '<' => Some(Left),
            '>' => Some(Right),
            '+' => Some(Inc),
            '-' => Some(Dec),
            '.' => Some(Out),
            ',' => Some(In),
            '[' => Some(FJump),
            ']' => Some(BJump),
            _   => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Insn {
    op: Op,
    count: u32,
}

fn parse(input: &str) -> Vec<Insn> {
    let mut prog = Vec::new();

    let mut prev = ' ';
    let mut count = 0;

    for c in input.chars() {
        if c == prev {
            count += 1;
            continue;
        }

        let op = match Op::from_char(prev) {
            Some(o) => o,
            None => { prev = c; count = 1; continue }
        };

        if op == FJump || op == BJump {
            for _ in 0 .. count {
                prog.push(Insn { op: op, count: 0 });
            }
        } else {
            prog.push(Insn { op: op, count: count });
        }

        prev = c;
        count = 1;
    }

    // fix up braces
    for i in 0 .. prog.len() {
        let op = prog[i].op;

        match op {
            FJump => {
                let mut num_braces = 1i32;
                for j in i + 1 .. prog.len() {
                    let op2 = prog[j].op;

                    match op2 {
                        FJump => num_braces += 1,
                        BJump => num_braces -= 1,
                        _     => { }
                    }

                    if num_braces == 0 {
                        prog[i].count = j as u32;
                        prog[j].count = i as u32;
                        break;
                    }
                }
            },
            _ => { }
        }
    }

    prog
}

fn main() {
    let mut args = env::args();
    let mut input = String::new();
    File::open(std::path::PathBuf::from(args.nth(1).unwrap())).unwrap().read_to_string(&mut input).unwrap();

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let prog = parse(&input);

    let mut state = [0u8; 3000];
    let mut pointer = 0usize;
    let mut idx = 0usize;

    while idx < prog.len() {
        let insn = prog[idx];

        match insn.op {
            Left  => pointer -= insn.count as usize,
            Right => pointer += insn.count as usize,

            Inc => state[pointer] += insn.count as u8,
            Dec => state[pointer] -= insn.count as u8,

            Out => {
                stdout.write(&state[pointer .. pointer + insn.count as usize]).unwrap();
            },
            In  => {
                assert!(stdin.read(&mut state[pointer .. pointer + insn.count as usize]).unwrap() == insn.count as usize);
            },

            FJump => if state[pointer] == 0 { idx = insn.count as usize },
            BJump => if state[pointer] != 0 { idx = insn.count as usize }
        }

        idx += 1;
    }
}
