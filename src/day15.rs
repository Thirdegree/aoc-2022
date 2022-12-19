use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::digit1,
    combinator::opt,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(vals: (i32, i32)) -> Self {
        Self {
            x: vals.0,
            y: vals.1,
        }
    }
}

impl Point {
    fn from_str(input: &str) -> IResult<&str, Self> {
        let (input, (x, _, y)) = tuple((
            preceded(tag("x="), parse_signed_int),
            tag(", "),
            preceded(tag("y="), parse_signed_int),
        ))(input)?;
        Ok((input, Point { x, y }))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Sensor {
    loc: Point,
    closest_beacon: Point,
    dist_to_closest_beacon: i32,
}

fn parse_signed_int(input: &str) -> IResult<&str, i32> {
    let (input, (sign, number)) = tuple((opt(alt((tag("-"), tag("+")))), digit1))(input)?;
    Ok((
        input,
        match sign {
            None | Some("+") => number.parse().unwrap(),
            Some("-") => -1 * number.parse::<i32>().unwrap(),
            _ => unreachable!(),
        },
    ))
}

fn dist(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

impl Sensor {
    fn might_have_beacon(&self, point: &Point) -> bool {
        point == &self.closest_beacon || self.distance(point) > self.dist_to_closest_beacon
    }
    fn distance(&self, point: &Point) -> i32 {
        dist(&self.loc, point)
    }
    fn push_x_right(&self, point: &Point) -> Point {
        let dx = self.dist_to_closest_beacon - (self.loc.y - point.y).abs();
        Point {
            x: self.loc.x + dx,
            y: point.y,
        }
    }
    fn from_str(input: &str) -> IResult<&str, Self> {
        let (input, loc) = preceded(tag("Sensor at "), Point::from_str)(input)?;
        let (input, closest_beacon) =
            preceded(tag(": closest beacon is at "), Point::from_str)(input)?;
        let dist_to_closest_beacon = dist(&loc, &closest_beacon);
        Ok((
            input,
            Sensor {
                loc,
                closest_beacon,
                dist_to_closest_beacon,
            },
        ))
    }
}

fn _get_min_maxes(sensors: &Vec<&Sensor>) -> Option<(i32, i32, i32, i32)> {
    let min_x = sensors
        .iter()
        .flat_map(|sensor| {
            vec![
                sensor.loc.x - sensor.dist_to_closest_beacon,
                sensor.loc.x + sensor.dist_to_closest_beacon,
                sensor.closest_beacon.x - sensor.dist_to_closest_beacon,
                sensor.closest_beacon.x + sensor.dist_to_closest_beacon,
            ]
        })
        .min()?;
    let max_x = sensors
        .iter()
        .flat_map(|sensor| {
            vec![
                sensor.loc.x - sensor.dist_to_closest_beacon,
                sensor.loc.x + sensor.dist_to_closest_beacon,
                sensor.closest_beacon.x - sensor.dist_to_closest_beacon,
                sensor.closest_beacon.x + sensor.dist_to_closest_beacon,
            ]
        })
        .max()?;
    let min_y = sensors
        .iter()
        .flat_map(|sensor| {
            vec![
                sensor.loc.y - sensor.dist_to_closest_beacon,
                sensor.loc.y + sensor.dist_to_closest_beacon,
                sensor.closest_beacon.y - sensor.dist_to_closest_beacon,
                sensor.closest_beacon.y + sensor.dist_to_closest_beacon,
            ]
        })
        .min()?;
    let max_y = sensors
        .iter()
        .flat_map(|sensor| {
            vec![
                sensor.loc.y - sensor.dist_to_closest_beacon,
                sensor.loc.y + sensor.dist_to_closest_beacon,
                sensor.closest_beacon.y - sensor.dist_to_closest_beacon,
                sensor.closest_beacon.y + sensor.dist_to_closest_beacon,
            ]
        })
        .max()?;
    Some((min_x, max_x, min_y, max_y))
}

pub fn main() {
    // when you enter an area known by a sensor, you can use the distance to figure out how far you
    // can skip -- save a lot of time by just not checking large aress
    let sensors: HashMap<_, _> = include_str!("data/day15.txt")
        .lines()
        .map(|line| {
            let (input, sensor) = Sensor::from_str(line).unwrap();
            assert_eq!(input.len(), 0);
            (sensor.loc, sensor)
        })
        .collect();
    let beacons: HashSet<_> = sensors.values().map(|s| s.closest_beacon).collect();

    let (_min, max) = (0, 4000000);
    // let batch_size = 400000;
    let mut cur_pos = Point { x: 0, y: 0 };
    loop {
        match sensors
            .values()
            .filter(|s| !s.might_have_beacon(&cur_pos))
            .map(|s| s.push_x_right(&cur_pos))
            .max_by_key(|new_point| new_point.x)
        {
            None => {
                if beacons.contains(&cur_pos) {
                    cur_pos.x += 1;
                    continue;
                }
                println!(
                    "{:?} -> {}",
                    (cur_pos.x, cur_pos.y),
                    ((cur_pos.x as i64) * 4000000) + cur_pos.y as i64
                );
                break;
            }
            Some(new_cur_pos) => {
                if new_cur_pos == cur_pos {
                    // no new jump, so either linewrap or just step
                    if cur_pos.x >= max {
                        if cur_pos.y >= max {
                            break;
                        }
                        if (cur_pos.y + 1) % 1000 == 0 {
                            println!("{:?}", cur_pos);
                        }
                        cur_pos.x = 0;
                        cur_pos.y += 1;
                    } else {
                        cur_pos.x += 1;
                    }
                } else {
                    cur_pos = new_cur_pos
                }
            }
        }
    }
    //for y in min..=max {
    //    for x in min..=max {
    //        let point = Point { x, y };
    //        if !sensors.values().any(|s| !s.might_have_beacon(&point)) {
    //            println!("{:?} -> {}", (x, y), (x * 4000000) + y);
    //        }
    //    }
    //    if y % 1000 == 0 {
    //        println!("{}", y)
    //    }
    //}

    // let row_to_test = 2000000;
    // let (min_x, max_x, _min_y, _max_y) = get_min_maxes(&sensors.values().collect()).unwrap();
    // let mut do_not_have = 0;
    // for x in min_x..=max_x {
    //     if !sensors
    //         .values()
    //         .all(|s| s.might_have_beacon(&Point { x, y: row_to_test }))
    //     {
    //         do_not_have += 1;
    //     }
    // }
    // println!("{}", do_not_have);
}
