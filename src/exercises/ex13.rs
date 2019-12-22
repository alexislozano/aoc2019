use crate::helpers::file::{read, write};
use crate::helpers::computer::{Computer, State};
use std::collections::HashMap;


pub fn ex13() {
    let e = "13";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i32 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    create_game(program).iter().map(|(_, v)|
        if *v == 2 { 1 } else { 0 }
    ).sum()
}

pub fn sub2(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    play_game(program)
}

fn play_game(program: Vec<i64>) -> i64 {
    let mut game = HashMap::new();
    let mut computer = Computer::new("computer", program);
    let mut score = 0;
    computer.set_program(0, 2);
    loop {
        computer.run();
        match computer.state() {
            State::Output => {
                let x = computer.output();
                computer.run();
                let y = computer.output();
                computer.run();
                match (x, y) {
                    (-1, 0) => score = computer.output(),
                    _ => {
                        game.insert((x, y), computer.output());
                    },
                }
            },
            State::Input => {
                let ball_x = find(&game, 4).0;
                let hpad_x = find(&game, 3).0;
                computer.set_input(if ball_x > hpad_x {
                    1
                } else if ball_x < hpad_x {
                    -1
                } else {
                    0
                });
            },
            State::Terminated => break,
            _ => (),
        }
    }
    score
}

fn find(game: &HashMap<(i64, i64), i64>, id: i64) -> (i64, i64) {
    let mut position = (0, 0);
    for (k, v) in game.iter() {
        if *v == id { 
            position = *k;
            break;
        }
    }
    position
}

fn create_game(program: Vec<i64>) -> HashMap<(i64, i64), i64> {
    let mut game = HashMap::new();
    let mut computer = Computer::new("computer", program);
    loop {
        computer.run();
        match computer.state() {
            State::Output => {
                let x = computer.output();
                computer.run();
                let y = computer.output();
                computer.run();
                let tile = computer.output();
                game.insert((x, y), tile);
            },
            State::Terminated => break,
            _ => panic!("Dafuk")
        }
    }
    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("104,1,104,2,104,3,104,6,104,5,104,4,99"), 0);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("104,1,104,2,104,2,104,6,104,5,104,4,99"), 1);
    }
}
