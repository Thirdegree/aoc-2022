#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["noop"] => Self::Noop,
            ["addx", val] => Self::Addx(val.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Op {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct Program {
    ops: Vec<Op>,
}

impl From<&str> for Program {
    fn from(prog: &str) -> Self {
        Self {
            ops: prog.lines().map(|l| l.into()).collect(),
        }
    }
}

impl Program {
    fn run(&mut self) {
        let mut ops = self.ops.iter();
        let mut cur_instr = ops.next().unwrap();
        let mut waiting = cur_instr.cycles();
        let mut x = 1;
        for clock in 0.. {
            waiting -= 1;
            if clock != 0 && clock % 40 == 0 {
                print!("\n");
            }
            if (((clock % 40) - x) as i32).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            if waiting == 0 {
                match cur_instr {
                    Op::Noop => (),
                    Op::Addx(i) => x += i,
                }
                if let Some(o) = ops.next() {
                    cur_instr = o;
                } else {
                    break;
                }
                waiting = cur_instr.cycles();
            }
        }
    }
}

pub fn main() {
    let mut prog: Program = include_str!("data/day10.txt").into();
    prog.run();
}
