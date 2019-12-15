use crate::helpers::computer::Computer;
use crate::helpers::file::{read, write};

pub fn ex02() {
    let e = "02";
    let s = read(e);
    write(
        e,
        &sub1(&s, Some((12, 2))).to_string(),
        &sub2(&s).to_string(),
    );
}

pub fn sub1(s: &str, args: Option<(i64, i64)>) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut computer = Computer::new("computer", program);
    if let Some((noun, verb)) = args {
        computer.set_program(1, noun);
        computer.set_program(2, verb);
    }
    computer.run(0);
    computer.program(0)
}

pub fn sub2(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut noun = 0;
    let mut verb = 0;
    'noun: for n in 0..99 {
        for v in 0..99 {
            let mut computer = Computer::new("computer", program.clone());
            computer.set_program(1, n);
            computer.set_program(2, v);
            computer.run(0);
            if computer.program(0) == 19_690_720 {
                noun = n;
                verb = v;
                break 'noun;
            }
        }
    }
    100 * noun + verb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("1,9,10,3,2,3,11,0,99,30,40,50", None), 3500);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("1,0,0,0,99", None), 2);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("2,3,0,3,99", None), 2);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("2,4,4,5,99,0", None), 2);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1("1,1,1,4,99,5,6,0,99", None), 30);
    }
}
