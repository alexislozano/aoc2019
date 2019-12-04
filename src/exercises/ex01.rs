use std::fs;

pub fn ex01() {
    match fs::read_to_string("src/inputs/ex01.txt") {
        Ok(s) => println!("Ex 01.1: {} | Ex 01.2: {}", sub1(&s), sub2(&s)),
        _ => println!("Cannot read ex01.txt"),
    }
}

pub fn sub1(s: &str) -> i32 {
    s.split('\n')
        .map(|mass| match mass.parse::<i32>() {
            Ok(m) => fuel(m),
            _ => 0,
        })
        .sum()
}

pub fn sub2(s: &str) -> i32 {
    s.split('\n')
        .map(|mass| match mass.parse::<i32>() {
            Ok(m) => rec_fuel(m),
            _ => 0,
        })
        .sum()
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn rec_fuel(mass: i32) -> i32 {
    let f = fuel(mass);
    if f <= 0 {
        0
    } else {
        f + rec_fuel(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("12"), 2);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("14"), 2);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("1969"), 654);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("100756"), 33583);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("14"), 2);
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2("1969"), 966);
    }

    #[test]
    fn sub23() {
        assert_eq!(sub2("100756"), 50346);
    }
}
