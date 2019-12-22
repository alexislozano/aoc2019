use crate::helpers::file::{read, write};
use crate::helpers::computer::Computer;

pub fn ex15() {
    let e = "15";
    let s = read(e);
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
        let mut game = Game::new(program);
    let result = game.run();
    write(e, &result.0.to_string(), &result.1.to_string());
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_number(self) -> i64 {
        match self {
            Self::Up => 1,
            Self::Down => 2,
            Self::Left => 3,
            Self::Right => 4
        }
    }

    fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right
        }
    }

    fn mov(self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::Right => (1, 0),
            Self::Down => (0, -1),
            Self::Left => (-1, 0)
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Clone)]
struct Tile {
    position: (i32, i32),
    directions: Vec<Direction>,
}

impl Tile {
    fn new(position: (i32, i32), directions: Vec<Direction>) -> Self {
        Self { position, directions }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

struct Game {
    current_tile: Tile,
    computer: Computer,
}

impl Game {
    fn new(program: Vec<i64>) -> Self {
        Self {  
            current_tile: Tile::new((0, 0), vec![]),
            computer: Computer::new("C", program) 
        }
    }

    fn run(&mut self) -> (i32, i32) {
        let mut frontier: Vec<Tile> = vec![self.current_tile.clone()];
        let mut trash: Vec<Tile> = vec![];
        let directions_to_oxygen = loop {
            let tile = frontier.remove(0);
            let tile_copy = tile.clone();
            self.move_to(tile);
            trash.push(tile_copy);
            
            if self.add_neighbours(&mut frontier, &trash) {
                let tile = frontier.remove(0);
                self.move_to(tile);
                break self.current_tile.directions.len();
            }
        };
        frontier = vec![self.current_tile.clone()];
        trash = vec![];
        let mut time;
        loop {
            let tile = frontier.remove(0);
            time = tile.directions.len();
            let tile_copy = tile.clone();
            self.move_to(tile);
            trash.push(tile_copy);
            
            self.fill_neighbours(&mut frontier, &trash);
            if frontier.is_empty() {
                break;
            }
        };
        (directions_to_oxygen as i32, time as i32 + 2)
    }

    fn add_neighbours(&mut self, frontier: &mut Vec<Tile>, trash: &[Tile]) -> bool {
        let mut found = false;
        for d in DIRECTIONS.iter() {
            match self.check_direction(*d) {
                res @ 1..=2 => {
                    let mov = d.mov();
                    let new_position = (
                        self.current_tile.position.0 + mov.0,
                        self.current_tile.position.1 + mov.1,
                    );
                    match frontier.iter().find(|tile| 
                        tile.position == new_position
                    ) {
                        Some(_) => (),
                        None => match trash.iter().find(|tile| 
                            tile.position == new_position
                        ) {
                            Some(_) => (),
                            None => {
                                let mut directions = self.current_tile.directions.clone();
                                directions.push(*d);
                                frontier.push(Tile::new(new_position, directions))
                            }
                        }
                    }
                    if res == 2 {
                        found = true;
                    }
                } 
                0 => (),
                _ => unreachable!()
            }
        }
        found
    }

    fn fill_neighbours(&mut self, frontier: &mut Vec<Tile>, trash: &[Tile]) {
        for d in DIRECTIONS.iter() {
            match self.check_direction(*d) {
                1..=2 => {
                    let mov = d.mov();
                    let new_position = (
                        self.current_tile.position.0 + mov.0,
                        self.current_tile.position.1 + mov.1,
                    );
                    match frontier.iter().find(|tile| 
                        tile.position == new_position
                    ) {
                        Some(_) => (),
                        None => match trash.iter().find(|tile| 
                            tile.position == new_position
                        ) {
                            Some(_) => (),
                            None => {
                                let mut directions = self.current_tile.directions.clone();
                                directions.push(*d);
                                frontier.push(Tile::new(new_position, directions))
                            }
                        }
                    }
                } 
                0 => (),
                _ => unreachable!()
            }
        }
    }

    fn check_direction(&mut self, d: Direction) -> i64 {
        self.computer.run();
        self.computer.set_input(d.to_number());
        self.computer.run();
        let output = self.computer.output();
        if let 1..=2 = output {
            self.computer.run();
            self.computer.set_input(d.reverse().to_number());
            self.computer.run();
        }
        output
    }

    fn move_to(&mut self, tile: Tile) {
        for d in self.current_tile.directions.iter().rev() {
            self.computer.run();
            self.computer.set_input(d.reverse().to_number());
            self.computer.run();
        }
        for d in tile.directions.iter() {
            self.computer.run();
            self.computer.set_input(d.to_number());
            self.computer.run();
        }
        self.current_tile = tile;
    }
}
