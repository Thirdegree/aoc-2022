use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::opt,
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    inspected: usize,
    operation: Operation,
    test_div_by: u128,
    throw_if_true: usize,
    throw_if_false: usize,
}

#[derive(Debug, Clone, Copy)]
enum OptVal {
    Val(u128),
    Old,
}

impl OptVal {
    fn from_str(input: &str) -> IResult<&str, OptVal> {
        let (input, found) = alt((digit1, tag("old")))(input)?;
        Ok((
            input,
            match found {
                "old" => OptVal::Old,
                val => OptVal::Val(val.parse().unwrap()),
            },
        ))
    }
}

#[derive(Debug, Clone, Copy)]
enum MulOrAdd {
    Mul,
    Add,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    mora: MulOrAdd,
    a: OptVal,
    b: OptVal,
}

impl Operation {
    fn exec(&self, old: u128) -> u128 {
        let e = if let MulOrAdd::Mul = self.mora {
            |a, b| a * b
        } else {
            |a, b| a + b
        };
        match (&self.a, &self.b) {
            (&OptVal::Old, &OptVal::Old) => e(old, old),
            (&OptVal::Val(a), &OptVal::Old) => e(a, old),
            (&OptVal::Old, &OptVal::Val(b)) => e(old, b),
            (&OptVal::Val(a), &OptVal::Val(b)) => e(a, b),
        }
    }
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (a, op, b)) = preceded(
        tag("  Operation: new = "),
        tuple((
            OptVal::from_str,
            alt((tag(" * "), tag(" + "))),
            OptVal::from_str,
        )),
    )(input)?;
    Ok((
        input,
        match op {
            " * " => Operation {
                mora: MulOrAdd::Mul,
                a,
                b,
            },
            " + " => Operation {
                mora: MulOrAdd::Add,
                a,
                b,
            },
            _ => unreachable!(),
        },
    ))
}

impl Monkey {
    fn turn(&mut self, remainder_mul: u128) -> Vec<(usize, u128)> {
        let item_iter = self.items.iter();
        let mut throws = Vec::new();
        for &item in item_iter {
            self.inspected += 1;
            let new_item = self.operation.exec(item) % remainder_mul;
            if new_item % self.test_div_by == 0 {
                throws.push((self.throw_if_true, new_item));
            } else {
                throws.push((self.throw_if_false, new_item));
            }
        }
        throws
    }

    fn catch(&mut self, item: u128) {
        self.items.push(item);
    }

    fn from_str(input: &str) -> IResult<&str, Self> {
        // they're in order, so don't care
        let (input, _) = delimited(tag("Monkey "), digit1, tag(":\n"))(input)?;
        let (input, elems) = terminated(
            preceded(
                tag("  Starting items: "),
                many1(terminated(digit1, opt(tag(", ")))),
            ),
            newline,
        )(input)?;
        let (input, operation) = terminated(parse_operation, newline)(input)?;
        let (input, test_div_by) =
            terminated(preceded(tag("  Test: divisible by "), digit1), newline)(input)?;
        let (input, throw_if_true) = terminated(
            preceded(tag("    If true: throw to monkey "), digit1),
            newline,
        )(input)?;
        let (input, throw_if_false) = terminated(
            preceded(tag("    If false: throw to monkey "), digit1),
            newline,
        )(input)?;
        Ok((
            input,
            Monkey {
                items: elems.iter().map(|e| e.parse().unwrap()).collect(),
                inspected: 0,
                operation,
                test_div_by: test_div_by.parse().unwrap(),
                throw_if_true: throw_if_true.parse().unwrap(),
                throw_if_false: throw_if_false.parse().unwrap(),
            },
        ))
    }
}

pub fn main() {
    let (input, mut monks) =
        many1(terminated(Monkey::from_str, opt(newline)))(include_str!("data/day11.txt")).unwrap();
    assert_eq!("", input);
    let remainder_mul = monks.iter().map(|m| m.test_div_by).fold(1, |acc, x| acc * x);
    for _round in 0..10000 {
        for monkidx in 0..monks.len() {
            let throws = monks[monkidx].turn(remainder_mul);
            for (t, item) in throws {
                monks[t].catch(item);
            }
            monks[monkidx].items.clear();
        }
    }
    println!("{:#?}", monks);
    let mut ins: Vec<usize> = monks.iter().map(|m| m.inspected).collect();
    ins.sort();
    println!("{}", ins.iter().rev().take(2).fold(1, |acc, x| acc * x ));
}
