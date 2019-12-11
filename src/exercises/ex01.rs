use crate::helpers::file::{read, write};

pub fn ex01() {
    let e = "01";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
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
