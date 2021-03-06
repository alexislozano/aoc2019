use crate::helpers::file::{read, write};

pub fn ex04() {
    let e = "04";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i32 {
    let range = s.split('-').collect::<Vec<&str>>();
    let min = range[0].parse::<i32>().unwrap_or(0);
    let max = range[1].parse::<i32>().unwrap_or(0);
    passwords(min, max)
}

pub fn sub2(s: &str) -> i32 {
    let range = s.split('-').collect::<Vec<&str>>();
    let min = range[0].parse::<i32>().unwrap_or(0);
    let max = range[1].parse::<i32>().unwrap_or(0);
    new_passwords(min, max)
}

fn passwords(min: i32, max: i32) -> i32 {
    let mut x = 0;
    for p in min..=max {
        if criteria(p) {
            x += 1;
        }
    }
    x
}

fn criteria(p: i32) -> bool {
    let mut numbers = vec![];
    for i in 0..=5 {
        let n = p / 10_i32.pow(5 - i) % 10;
        numbers.push(n);
    }
    let mut adjacent = false;
    for i in 1..=5 {
        if numbers[i] == numbers[i - 1] {
            adjacent = true;
            break;
        }
    }
    let mut sorted = numbers.clone();
    sorted.sort();
    adjacent && sorted == numbers
}

fn new_passwords(min: i32, max: i32) -> i32 {
    let mut x = 0;
    for p in min..=max {
        if new_criteria(p) {
            x += 1;
        }
    }
    x
}

fn new_criteria(p: i32) -> bool {
    let mut numbers = vec![];
    for i in 0..=5 {
        let n = p / 10_i32.pow(5 - i) % 10;
        numbers.push(n);
    }
    let mut adjacent = false;
    for i in 1..=5 {
        if numbers[i] == numbers[i - 1]
            && (i as i32 - 2 < 0 || numbers[i - 2] != numbers[i])
            && (i as i32 + 1 > 5 || numbers[i + 1] != numbers[i])
        {
            adjacent = true;
            break;
        }
    }
    let mut sorted = numbers.clone();
    sorted.sort();
    adjacent && sorted == numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(criteria(111_111), true);
    }

    #[test]
    fn sub12() {
        assert_eq!(criteria(223_450), false);
    }

    #[test]
    fn sub13() {
        assert_eq!(criteria(123_789), false);
    }

    #[test]
    fn sub21() {
        assert_eq!(new_criteria(112_233), true);
    }

    #[test]
    fn sub22() {
        assert_eq!(new_criteria(123_444), false);
    }

    #[test]
    fn sub23() {
        assert_eq!(new_criteria(111_122), true);
    }
}
