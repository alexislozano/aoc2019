use crate::helpers::computer::{Computer, State};
use crate::helpers::file::{read, write};

pub fn ex05() {
    let e = "05";
    let s = read(e);
    write(e, &sub1(&s, 1).to_string(), &sub2(&s, 5).to_string());
}

pub fn sub1(s: &str, input: i64) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut computer = Computer::new("computer", program);
    computer.set_input(input);
    loop {
        computer.run();
        if let State::Terminated = computer.state() {
            break computer.output();
        }
    }
}

pub fn sub2(s: &str, input: i64) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut computer = Computer::new("computer", program);
    computer.set_input(input);
    computer.run();
    loop {
        computer.run();
        if let State::Terminated = computer.state() {
            break computer.output();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("3,0,4,0,99", 24), 24);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("1002,4,3,4,33", 0), 0);
    }

    #[test]
    fn sub21() {
        let s = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(sub2(s, 8), 1);
        assert_eq!(sub2(s, 0), 0);
    }

    #[test]
    fn sub22() {
        let s = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(sub2(s, 7), 1);
        assert_eq!(sub2(s, 8), 0);
    }

    #[test]
    fn sub23() {
        let s = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(sub2(s, 8), 1);
        assert_eq!(sub2(s, 0), 0);
    }

    #[test]
    fn sub24() {
        let s = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(sub2(s, 7), 1);
        assert_eq!(sub2(s, 8), 0);
    }

    #[test]
    fn sub25() {
        let s = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(sub2(s, 0), 0);
        assert_eq!(sub2(s, 1), 1);
    }

    #[test]
    fn sub26() {
        let s = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(sub2(s, 0), 0);
        assert_eq!(sub2(s, 1), 1);
    }

    #[test]
    fn sub27() {
        let s = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(sub2(s, 7), 999);
        assert_eq!(sub2(s, 8), 1000);
        assert_eq!(sub2(s, 9), 1001);
    }
}
