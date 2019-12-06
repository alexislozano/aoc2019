use std::fs;

pub fn ex05() {
    match fs::read_to_string("src/inputs/ex05.txt") {
        Ok(s) => println!(
            "Ex 05.1: {} | Ex 05.2: {}",
            sub1(&s, None, 1),
            sub2(&s, None, 5)
        ),
        _ => println!("Cannot read ex05.txt"),
    }
}

pub fn sub1(s: &str, args: Option<(i32, i32)>, input: i32) -> i32 {
    let mut program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    run(&mut program, args, input)
}

pub fn sub2(s: &str, args: Option<(i32, i32)>, input: i32) -> i32 {
    let mut program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    run(&mut program, args, input)
}

fn run(program: &mut Vec<i32>, args: Option<(i32, i32)>, input: i32) -> i32 {
    if let Some((n, v)) = args {
        program[1] = n;
        program[2] = v;
    }

    let mut counter = 0;
    let mut code = Code::new(program[counter]);
    let mut output = 0;

    loop {
        match code.opcode {
            Opcode::Exit => {
                break;
            }
            _ => {
                let index0 = match code.modes.0 {
                    Mode::Position => program[counter + 1] as usize,
                    Mode::Immediate => counter + 1,
                };
                match code.opcode {
                    Opcode::Add
                    | Opcode::Multiply
                    | Opcode::LessThan
                    | Opcode::Equals
                    | Opcode::JumpIfTrue
                    | Opcode::JumpIfFalse => {
                        let index1 = match code.modes.1 {
                            Mode::Position => program[counter + 2] as usize,
                            Mode::Immediate => counter + 2,
                        };
                        match code.opcode {
                            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                                counter = match code.opcode {
                                    Opcode::JumpIfTrue => {
                                        if program[index0] != 0 {
                                            program[index1] as usize
                                        } else {
                                            counter + 3
                                        }
                                    }
                                    Opcode::JumpIfFalse => {
                                        if program[index0] == 0 {
                                            program[index1] as usize
                                        } else {
                                            counter + 3
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            Opcode::Add
                            | Opcode::Multiply
                            | Opcode::LessThan
                            | Opcode::Equals => {
                                let index2 = match code.modes.2 {
                                    Mode::Position => {
                                        program[counter + 3] as usize
                                    }
                                    Mode::Immediate => counter + 3,
                                };
                                program[index2] = match code.opcode {
                                    Opcode::Add => {
                                        program[index0] + program[index1]
                                    }
                                    Opcode::Multiply => {
                                        program[index0] * program[index1]
                                    }
                                    Opcode::LessThan => {
                                        if program[index0] < program[index1] {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                    Opcode::Equals => {
                                        if program[index0] == program[index1] {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                counter += 4;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Opcode::Input | Opcode::Output => {
                        match code.opcode {
                            Opcode::Input => {
                                program[index0] = input;
                            }
                            Opcode::Output => {
                                output = program[index0];
                            }
                            _ => unreachable!(),
                        }
                        counter += 2;
                    }
                    _ => unreachable!(),
                }
            }
        };

        code = Code::new(program[counter]);
    }

    output
}

#[derive(Debug)]
struct Code {
    opcode: Opcode,
    modes: (Mode, Mode, Mode),
}

impl Code {
    fn new(code: i32) -> Code {
        let code_str =
            format!("{}{}", "0".repeat(5 - code.to_string().len()), code);
        Code {
            opcode: Opcode::new(&code_str[3..5]),
            modes: (
                Mode::new(&code_str[2..3]),
                Mode::new(&code_str[1..2]),
                Mode::new(&code_str[0..1]),
            ),
        }
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn new(s: &str) -> Mode {
        match s {
            "0" => Mode::Position,
            "1" => Mode::Immediate,
            _ => panic!("Mode does not exist"),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Exit,
}

impl Opcode {
    fn new(s: &str) -> Opcode {
        match s {
            "01" => Opcode::Add,
            "02" => Opcode::Multiply,
            "03" => Opcode::Input,
            "04" => Opcode::Output,
            "05" => Opcode::JumpIfTrue,
            "06" => Opcode::JumpIfFalse,
            "07" => Opcode::LessThan,
            "08" => Opcode::Equals,
            "99" => Opcode::Exit,
            _ => panic!("Opcode does not exist"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("3,0,4,0,99", None, 24), 24);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("1002,4,3,4,33", None, 0), 0);
    }

    #[test]
    fn sub21() {
        let s = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(sub2(s, None, 8), 1);
        assert_eq!(sub2(s, None, 0), 0);
    }

    #[test]
    fn sub22() {
        let s = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(sub2(s, None, 7), 1);
        assert_eq!(sub2(s, None, 8), 0);
    }

    #[test]
    fn sub23() {
        let s = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(sub2(s, None, 8), 1);
        assert_eq!(sub2(s, None, 0), 0);
    }

    #[test]
    fn sub24() {
        let s = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(sub2(s, None, 7), 1);
        assert_eq!(sub2(s, None, 8), 0);
    }

    #[test]
    fn sub25() {
        let s = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(sub2(s, None, 0), 0);
        assert_eq!(sub2(s, None, 1), 1);
    }

    #[test]
    fn sub26() {
        let s = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(sub2(s, None, 0), 0);
        assert_eq!(sub2(s, None, 1), 1);
    }

    #[test]
    fn sub27() {
        let s = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(sub2(s, None, 7), 999);
        assert_eq!(sub2(s, None, 8), 1000);
        assert_eq!(sub2(s, None, 9), 1001);
    }
}
