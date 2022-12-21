use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::opt,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
enum ValveState {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    rate: usize,
    state: ValveState,
}
fn parse_line(line: &str) -> IResult<&str, (Valve, Vec<&str>)> {
    let (line, (id, rate)) = tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), digit1),
    ))(line)?;
    let (line, valves) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        many1(terminated(alpha1, opt(tag(", ")))),
    )(line)?;
    let (line, _) = opt(newline)(line)?;
    Ok((
        line,
        (
            Valve {
                id: id.to_string(),
                rate: rate.parse().unwrap(),
                state: ValveState::Closed,
            },
            valves,
        ),
    ))
}

fn solve_dumbly(nodes: HashMap<String, Valve>, edges: &HashMap<String, Vec<String>>) {
    let mut nodes = nodes;
    let mut tot_released: usize = 0;
    let mut cur_valve = "AA".to_string();
    for minute in 1..=30 {
        println!("== Minute {} ==", minute);
        let open = nodes
            .values()
            .filter(|v| {
                if let ValveState::Open = v.state {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        println!("Open: {:?}", open);
        tot_released += open.iter().map(|v| v.rate).sum::<usize>();
        for next in &edges[&cur_valve] {
            if let ValveState::Closed = nodes[next].state {
                nodes.get_mut(next).unwrap().state = ValveState::Open;
                cur_valve = nodes[next].id.clone();
                break;
            }
        }
    }
    println!("Got {} released", tot_released)
}

fn _solve_slightly_less_dumbly(nodes: HashMap<String, Valve>, edges: &HashMap<String, Vec<String>>) {
    let mut nodes = nodes;
    let mut tot_released: usize = 0;
    let mut cur_valve = "AA".to_string();
    for minute in 1..=30 {
        println!("== Minute {} ==", minute);
        let open = nodes
            .values()
            .filter(|v| {
                if let ValveState::Open = v.state {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();
        println!("Open: {:?}", open);
        tot_released += open.iter().map(|v| v.rate).sum::<usize>();
        for next in &edges[&cur_valve] {
            if let ValveState::Closed = nodes[next].state {
                nodes.get_mut(next).unwrap().state = ValveState::Open;
                cur_valve = nodes[next].id.clone();
                break;
            }
        }
    }
    println!("Got {} released", tot_released)
}

pub fn main() {
    let mut nodes = HashMap::new();
    let mut edges: HashMap<_, Vec<_>> = HashMap::new();
    let (rest, nodes_and_edges) = many1(parse_line)(include_str!("data/day16.txt")).unwrap();
    println!("{}", rest);
    assert_eq!(rest.len(), 0);
    for (valve, valve_conns) in &nodes_and_edges {
        nodes.insert(valve.id.clone(), valve.clone());
        edges.insert(valve.id.clone(), valve_conns.iter().map(|e| e.to_string()).collect());
    }
    println!("{:?}", nodes);
    println!("{:?}", edges);
    solve_dumbly(nodes, &edges);
}
