use crate::helpers::file::{read, write};

pub fn ex12() {
    let e = "12";
    let s = read(e);
    write(e, &sub1(&s, 1000).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str, steps: i32) -> i32 {
    let moons = s.split('\n').map(|m| Moon::from_str(m)).collect::<Vec<Moon>>();
    let mut system = System::new(moons);
    for _ in 0..steps {
        system.update_velocities();
        system.update_positions();
    }
    system.energy()
}

pub fn sub2(s: &str) -> i64 {
    s.len() as i64
}

#[derive(Debug)]
struct System {
    moons: Vec<Moon>
}

impl System {
    fn new(moons: Vec<Moon>) -> Self {
        Self { moons }
    }

    fn update_velocities(&mut self) {
        for i in 0..self.moons.len() {
            for j in i..self.moons.len() {
                self.update_velocity(i, j);
            }
        }
    }

    fn update_velocity(&mut self, i: usize, j: usize) {
        if self.moons[i].position.0 > self.moons[j].position.0 {
            self.moons[i].velocity.0 -= 1;
            self.moons[j].velocity.0 += 1;
        } else if self.moons[i].position.0 < self.moons[j].position.0 {
            self.moons[i].velocity.0 += 1;
            self.moons[j].velocity.0 -= 1;
        }
        if self.moons[i].position.1 > self.moons[j].position.1 {
            self.moons[i].velocity.1 -= 1;
            self.moons[j].velocity.1 += 1;
        } else if self.moons[i].position.1 < self.moons[j].position.1 {
            self.moons[i].velocity.1 += 1;
            self.moons[j].velocity.1 -= 1;
        }
        if self.moons[i].position.2 > self.moons[j].position.2 {
            self.moons[i].velocity.2 -= 1;
            self.moons[j].velocity.2 += 1;
        } else if self.moons[i].position.2 < self.moons[j].position.2 {
            self.moons[i].velocity.2 += 1;
            self.moons[j].velocity.2 -= 1;
        }
    }

    fn update_positions(&mut self) {
        for i in 0..self.moons.len() {
            self.update_position(i);
        }
    }

    fn update_position(&mut self, i: usize) {
        self.moons[i].position.0 += self.moons[i].velocity.0;
        self.moons[i].position.1 += self.moons[i].velocity.1;
        self.moons[i].position.2 += self.moons[i].velocity.2;
    }

    fn energy(&self) -> i32 {
        self.moons.iter().map(|moon| (
                moon.position.0.abs() + 
                moon.position.1.abs() + 
                moon.position.2.abs()
            ) * (
                moon.velocity.0.abs() + 
                moon.velocity.1.abs() + 
                moon.velocity.2.abs()
        )).sum()
    }
}

#[derive(Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn from_str(s: &str) -> Self {
        let mut string = s.to_string();
        string.remove(0);
        string.remove(string.len() - 1);
        let p = string.split(", ").map(|v|
            v[2..v.len()].parse::<i32>().unwrap_or(0)
        ).collect::<Vec<i32>>();
        Self { position: (p[0], p[1], p[2]), velocity: (0, 0, 0) }
    }
}

impl PartialEq for Moon {
    fn eq(&self, other: &Moon) -> bool {
        self.position == other.position && self.velocity == other.velocity
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    impl Moon {
        fn new(x: i32, y: i32, z: i32) -> Self {
            Self { position: (x, y, z), velocity: (0, 0, 0) }
        }
    }

    #[test]
    fn sub11() {
        assert_eq!(
            Moon::from_str("<x=-1, y=0, z=2>"),
            Moon::new(-1, 0, 2)
        );
    }

    #[test]
    fn sub12() {
        assert_eq!(
            sub1("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>", 10),
            179
        );
    }

    #[test]
    fn sub13() {
        assert_eq!(
            sub1("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>", 100),
            1940
        );
    }

    #[test]
    fn sub21() {

        let moons0 = vec![
            (-1, 0, 2, 0, 0, 0),
            (2, -10, -7, 0, 0, 0),
            (4, -8, 8, 0, 0, 0),
            (3, 5, -1, 0, 0, 0),
        ];
        let moons1 = vec![
            (2, -1, 1, 3, -1, -1),
            (3, -7, -4, 1, 3, 3),
            (1, -7, 5, -3, 1, -3),
            (2, 2, 0, -1, -3, 1),
        ];
        let moons2 = vec![
            (5, -3, -1, 3, -2, -2),
            (1, -2, 2, -2, 5, 6),
            (1, -4, -1, 0, 3, -6),
            (1, -4, 2, -1, -6, 2),
        ];
        let mut m0 = 0;
        let mut m1 = 0;
        let mut m2 = 0;
        for x in moons0.iter() {
            m0 += x.0 + x.1 + x.2 + x.3 + x.4 + x.5;
        }
        for x in moons1.iter() {
            m1 += x.0 + x.1 + x.2 + x.3 + x.4 + x.5;
        }
        for x in moons2.iter() {
            m2 += x.0 + x.1 + x.2 + x.3 + x.4 + x.5;
        }
        assert_eq!(
            m0, m1
        );
        assert_eq!(
            m0, m2
        );
    }
}
