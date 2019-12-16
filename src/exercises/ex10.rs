use crate::helpers::file::{read, write};
use crate::num::Integer;
use std::cmp::Ordering;
use std::f64::consts::PI;

pub fn ex10() {
    let e = "10";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s, 200, None).to_string());
}

pub fn sub1(s: &str) -> i32 {
    let mut asteroids: Vec<Asteroid> = vec![];
    for (h, line) in s.split('\n').enumerate() {
        for (w, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroids.push(Asteroid::new(w as i32, h as i32));
            }
        }
    }
    nb_in_best_location(asteroids)
}

pub fn sub2(s: &str, th: i32, origin_index: Option<usize>) -> i32 {
    let mut asteroids: Vec<Asteroid> = vec![];
    for (h, line) in s.split('\n').enumerate() {
        for (w, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroids.push(Asteroid::new(w as i32, h as i32));
            }
        }
    }
    vaporized(asteroids, th, origin_index)
}

fn vaporized(
    mut asteroids: Vec<Asteroid>,
    th: i32,
    origin_index: Option<usize>,
) -> i32 {
    let (index, mut detected_asteroids) = match origin_index {
        None => best_location(&asteroids),
        Some(index) => (index, rotation(&asteroids, asteroids[index])),
    };
    let origin = asteroids[index];
    let mut nb_of_detected_asteroids;

    detected_asteroids.sort_by(|a, b| a.compare(*b, origin));
    nb_of_detected_asteroids = detected_asteroids.len();

    let th_asteroid: Asteroid = if nb_of_detected_asteroids >= th as usize {
        detected_asteroids[th as usize - 1]
    } else {
        loop {
            for asteroid in detected_asteroids.iter() {
                let asteroid_index =
                    asteroids.iter().position(|a| a == asteroid).unwrap();
                asteroids.remove(asteroid_index);
            }
            detected_asteroids = rotation(&asteroids, origin);
            detected_asteroids.sort_by(|a, b| a.compare(*b, origin));
            nb_of_detected_asteroids += detected_asteroids.len();

            if nb_of_detected_asteroids >= th as usize {
                break detected_asteroids[th as usize
                    - 1
                    - (nb_of_detected_asteroids - detected_asteroids.len())];
            }
        }
    };

    th_asteroid.x * 100 + th_asteroid.y
}

fn nb_in_best_location(asteroids: Vec<Asteroid>) -> i32 {
    best_location(&asteroids).1.len() as i32
}

fn best_location(asteroids: &[Asteroid]) -> (usize, Vec<Asteroid>) {
    let mut detected_asteroids: Vec<Asteroid> = vec![];
    let mut max_index = 0;

    for i in 0..asteroids.len() {
        let rotation = rotation(asteroids, asteroids[i]);
        if rotation.len() > detected_asteroids.len() {
            detected_asteroids = rotation;
            max_index = i;
        }
    }

    (max_index, detected_asteroids)
}

fn rotation(asteroids: &[Asteroid], origin: Asteroid) -> Vec<Asteroid> {
    let mut detected_asteroids: Vec<Asteroid> = vec![];
    let mut movs: Vec<(i32, i32)> = vec![];

    for j in 0..asteroids.len() {
        if asteroids[j].x != origin.x || asteroids[j].y != origin.y {
            let mut mov =
                (asteroids[j].x - origin.x, asteroids[j].y - origin.y);
            let gcd = mov.0.gcd(&mov.1);
            mov = (mov.0 / gcd, mov.1 / gcd);
            if !movs.contains(&mov) {
                let mut i = 1;
                let detected_asteroid = loop {
                    let detected_asteroid = Asteroid::new(
                        origin.x + i * mov.0,
                        origin.y + i * mov.1,
                    );
                    if asteroids.contains(&detected_asteroid) {
                        break detected_asteroid;
                    }
                    i += 1;
                };
                detected_asteroids.push(detected_asteroid);
                movs.push(mov);
            }
        }
    }

    detected_asteroids
}

#[derive(Debug, Clone, Copy)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn compare(self, other: Self, origin: Self) -> Ordering {
        let u = (self.x - origin.x, self.y - origin.y);
        let v = (other.x - origin.x, other.y - origin.y);

        let num_u = f64::from(-u.1);
        let den_u = f64::from(u.0.pow(2) + u.1.pow(2)).sqrt();

        let num_v = f64::from(-v.1);
        let den_v = f64::from(v.0.pow(2) + v.1.pow(2)).sqrt();

        let a_theta_u = num_u / den_u;
        let a_theta_v = num_v / den_v;

        let theta_u = if u.0 < 0 {
            2.0 * PI - a_theta_u.acos()
        } else {
            a_theta_u.acos()
        };
        let theta_v = if v.0 < 0 {
            2.0 * PI - a_theta_v.acos()
        } else {
            a_theta_v.acos()
        };

        theta_u.partial_cmp(&theta_v).unwrap()
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1(".#..#\n.....\n#####\n....#\n...##"), 8);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####"), 33);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###."), 35);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#.."), 41);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"), 210);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##", 1, Some(28)), 801);
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##", 30, Some(28)), 700);
    }

    #[test]
    fn sub23() {
        assert_eq!(sub2(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##", 31, Some(28)), 800);
    }
}
