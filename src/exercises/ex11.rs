use crate::helpers::computer::{Computer, State};
use crate::helpers::file::{read, write};
use std::collections::HashMap;

pub fn ex11() {
    let e = "11";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut board: HashMap<(i32, i32), i64> = HashMap::new();
    let mut robot = Robot::new(Computer::new("robot", program));
    loop {
        let (paint, turn) = robot.run(&mut board);
        match robot.state() {
            State::Terminated => break,
            _ => {
                robot.paint(&mut board, paint);
                robot.turn(turn);
                robot.forward();
            }
        }
    }
    board.len() as i64
}

pub fn sub2(s: &str) -> String {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut board: HashMap<(i32, i32), i64> = HashMap::new();
    board.insert((0, 0), 1);
    let mut robot = Robot::new(Computer::new("robot", program));
    loop {
        let (paint, turn) = robot.run(&mut board);
        match robot.state() {
            State::Terminated => break,
            _ => {
                robot.paint(&mut board, paint);
                robot.turn(turn);
                robot.forward();
            }
        }
    }
    image(&board)
}

fn image(board: &HashMap<(i32, i32), i64>) -> String {
    let mut s = "".to_string();
    for i in 0..8 {
        for j in 0..41 {
            match board.get(&(
                j as i32, 
                1 - i as i32
            )) {
                Some(color) => s.push_str(&format!("{} ", color)),
                None => s.push_str("0 "),
            }
        }
        s.push_str("\n");
    }
    s
}

struct Robot {
    computer: Computer,
    direction: Direction,
    position: (i32, i32),
}

impl Robot {
    fn new(computer: Computer) -> Self {
        Self {
            computer,
            direction: Direction::North,
            position: (0, 0),
        }
    }

    fn run(&mut self, board: &mut HashMap<(i32, i32), i64>) -> (i64, i64) {
        let input = match board.get(&self.position) {
            Some(color) => *color,
            None => 0,
        };
        self.computer.set_input(input);
        self.computer.run();
        let output0 = self.computer.output();
        self.computer.run();
        let output1 = self.computer.output();
        (output0, output1)
    }

    fn state(&self) -> &State {
        &self.computer.state()
    }

    fn paint(&mut self, board: &mut HashMap<(i32, i32), i64>, color: i64) {
        board.insert(self.position, color);
    }

    fn turn(&mut self, turn: i64) {
        self.direction = self.direction.turn(turn);
    }

    fn forward(&mut self) {
        self.position = match self.direction {
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 - 1),
            Direction::West => (self.position.0 - 1, self.position.1),
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, value: i64) -> Self {
        match self {
            Self::North => {
                if value == 0 {
                    Self::West
                } else {
                    Self::East
                }
            }
            Self::East => {
                if value == 0 {
                    Self::North
                } else {
                    Self::South
                }
            }
            Self::South => {
                if value == 0 {
                    Self::East
                } else {
                    Self::West
                }
            }
            Self::West => {
                if value == 0 {
                    Self::South
                } else {
                    Self::North
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("104,1,104,0,104,0,104,0,104,1,104,0,104,1,104,0,104,0,104,1,104,1,104,0,104,1,104,0,99"), 6);
    }
}
