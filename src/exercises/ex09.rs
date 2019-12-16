use crate::helpers::computer::{Computer, State};
use crate::helpers::file::{read, write};

pub fn ex09() {
    let e = "09";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut c = Computer::new("computer", program.clone());
    c.set_input(1);
    loop {
        match c.state() {
            State::Terminated => break c.output(),
            _ => {
                c.run();
            }
        }
    }
}

pub fn sub2(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut c = Computer::new("computer", program.clone());
    c.set_input(2);
    loop {
        match c.state() {
            State::Terminated => break c.output(),
            _ => {
                c.run();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("104,1125899906842624,99"), 1_125_899_906_842_624);
    }

    #[test]
    fn sub12() {
        assert_eq!(
            sub1("1102,34915192,34915192,7,4,7,99,0").to_string().len(),
            16
        );
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("104,-2,99"), -2);
    }
}
