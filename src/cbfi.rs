#![feature(phase)]
#[phase(link, plugin)] extern crate log;

use std::io::File;
use std::os;


#[deriving(PartialEq, Show)]
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
    fn to_char(&self) -> char {
        match *self {
            Left  => '<',
            Right => '>',
            Inc   => '+',
            Dec   => '-',
            Out   => '.',
            In    => ',',
            FJump => '[',
            BJump => ']',
        }
    }

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

    fn is_jump(&self) -> bool {
        *self == FJump || *self == BJump
    }
}

#[deriving(Show)]
struct Insn {
    op: Op,
    count: uint
}

impl Insn {
    fn num_ops(&self) -> uint {
        if self.op.is_jump() {
            1
        } else {
            self.count
        }
    }
}

fn parse(input: &str) -> Vec<Insn> {
    let mut prog = Vec::new();

    let mut prev = ' ';
    let mut count = 0;

    for c in input.as_slice().chars() {
        debug!("c = {}, count = {}, prev = {}", c, count, prev);

        if c == prev {
            count += 1;
            continue;
        }

        let op = match Op::from_char(prev) {
            Some(o) => o,
            None => { prev = c; count = 1; continue }
        };

        if op == FJump || op == BJump {
            for _ in range(0, count) {
                prog.push(Insn { op: op, count: 0 });
            }
        } else {
            prog.push(Insn { op: op, count: count });
        }

        prev = c;
        count = 1;
    }

    // fix up braces
    for i in range(0, prog.len()) {
        let op = prog.get(i).op;

        match op {
            FJump => {
                let mut num_braces = 1i32;
                for j in range(i + 1, prog.len()) {
                    let op2 = prog.get(j).op;

                    match op2 {
                        FJump => num_braces += 1,
                        BJump => num_braces -= 1,
                        _     => { }
                    }

                    if num_braces == 0 {
                        prog.get_mut(i).count = j;
                        prog.get_mut(j).count = i;
                        break;
                    }
                }
            },
            _ => { }
        }
    }

    prog
}

fn dump(prog: &[Insn]) {
    let mut indent = 0;
    for (idx, i) in prog.iter().enumerate() {
        match i.op {
            FJump => {
                println!("{}: {}{} {}", idx, " ".repeat(indent), '[', i.count);
                indent += 4;
                continue
            }
            BJump => indent -= 4,
            _     => { }
        }

        println!("{}: {}{} {}", idx, " ".repeat(indent), i.op.to_char(), i.count);
    }
}

fn reproduce(prog: &[Insn]) {
    for i in prog.iter() {
        for _ in range(0, i.num_ops()) {
            print!("{}", i.op.to_char());
        }
    }
    println!("");
}

fn main() {
    let input = File::open(&Path::new(os::args().get(1).clone()))
        .read_to_string().unwrap();

    let mut stdin = std::io::stdio::stdin_raw();
    let mut stdout = std::io::stdout();

    let prog = parse(input.as_slice());

    if os::getenv("dump").is_some() {
        dump(prog.as_slice());
        return;
    }

    if os::getenv("repro").is_some() {
        reproduce(prog.as_slice());
        return;
    }

    let mut state = [0u8, ..3000];
    let mut pointer = 0;
    let mut idx = 0;

    while idx < prog.len() {
        let insn = prog.get(idx);

        match insn.op {
            Left  => pointer -= insn.count,
            Right => pointer += insn.count,

            Inc => state[pointer] += insn.count as u8,
            Dec => state[pointer] -= insn.count as u8,

            Out => {
                for _ in range(0, insn.count) {
                    stdout.write_u8(state[pointer]);
                }
            },
            In  => {
                for _ in range(0, insn.count) {
                    state[pointer] = match stdin.read_u8() {
                        Ok(b) => b,
                        Err(_) => 0
                    }
                }
            },

            FJump => if state[pointer] == 0 { idx = insn.count },
            BJump => if state[pointer] != 0 { idx = insn.count }
        }

        idx += 1;
    }
}
