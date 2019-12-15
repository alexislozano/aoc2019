use crate::helpers::file::{read, write};
use std::cmp::{min, Ordering};

pub fn ex03() {
    let e = "03";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i32 {
    let wires = s
        .split('\n')
        .map(|w| {
            w.split(',')
                .map(|x| Segment::from_str(x))
                .collect::<Vec<Segment>>()
        })
        .collect::<Vec<Vec<Segment>>>();
    let mut points1 = points(&wires[0]);
    let mut points2 = points(&wires[1]);
    min_manhattan(&mut points1, &mut points2)
}

pub fn sub2(s: &str) -> i32 {
    let wires = s
        .split('\n')
        .map(|w| {
            w.split(',')
                .map(|x| Segment::from_str(x))
                .collect::<Vec<Segment>>()
        })
        .collect::<Vec<Vec<Segment>>>();
    let mut points1 = points(&wires[0]);
    let mut points2 = points(&wires[1]);
    min_distance(&mut points1, &mut points2)
}

fn sort_manhattan(a: (i32, i32), b: (i32, i32)) -> Ordering {
    (a.0.abs() + a.1.abs()).cmp(&(b.0.abs() + b.1.abs()))
}

fn min_distance(v1: &mut Vec<(i32, i32)>, v2: &mut Vec<(i32, i32)>) -> i32 {
    let mut distance = v1.len() + v2.len();
    'outer: for i in 1..v1.len() {
        if distance <= i {
            break 'outer;
        }
        for j in 1..min(distance - i, v2.len()) {
            let (x, y) = (v1[i], v2[j]);
            if x == y {
                distance = i + j;
            }
        }
    }
    distance as i32
}

fn min_manhattan(v1: &mut Vec<(i32, i32)>, v2: &mut Vec<(i32, i32)>) -> i32 {
    v1.sort_by(|a, b| sort_manhattan(*a, *b));
    v2.sort_by(|a, b| sort_manhattan(*a, *b));
    let mut man = 0;
    'outer: for i in 1..v1.len() {
        for j in 1..v2.len() {
            let (x, y) = (v1[i], v2[j]);
            if x == y {
                man = x.0.abs() + x.1.abs();
                break 'outer;
            }
        }
    }
    man
}

fn points(segments: &[Segment]) -> Vec<(i32, i32)> {
    let mut point = (0, 0);
    let mut points = vec![point];
    for segment in segments {
        for _ in 1..=segment.length {
            point = match &segment.direction {
                Direction::Up => (point.0, point.1 + 1),
                Direction::Down => (point.0, point.1 - 1),
                Direction::Left => (point.0 - 1, point.1),
                Direction::Right => (point.0 + 1, point.1),
            };
            points.push(point);
        }
    }
    points
}

#[derive(Debug)]
struct Segment {
    direction: Direction,
    length: u32,
}

impl Segment {
    fn from_str(s: &str) -> Segment {
        let d = &s[0..1];
        let l = &s[1..];
        Segment {
            direction: match d {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Direction is not U/D/L/R."),
            },
            length: l.parse::<u32>().unwrap_or(0),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    }

    #[test]
    fn sub12() {
        assert_eq!(
            sub1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
    }

    #[test]
    fn sub13() {
        assert_eq!(
            sub1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    }

    #[test]
    fn sub22() {
        assert_eq!(
            sub2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
    }

    #[test]
    fn sub23() {
        assert_eq!(
            sub2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
