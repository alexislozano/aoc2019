#[derive(Debug)]
pub struct Computer {
    name: String,
    program: Vec<i64>,
    counter: usize,
    base: i64,
    state: State,
    output: i64,
}

impl Computer {
    pub fn new(name: &str, program: Vec<i64>) -> Self {
        Self {
            name: name.to_string(),
            program,
            counter: 0,
            base: 0,
            state: State::Initialized,
            output: 0,
        }
    }

    pub fn set_program(&mut self, address: usize, value: i64) {
        if address >= self.program.len() {
            self.program.resize(1 + address, 0);
        }
        self.program[address] = value;
    }

    pub fn program(&self, address: usize) -> i64 {
        if address >= self.program.len() {
            0
        } else {
            self.program[address]
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn output(&self) -> i64 {
        self.output
    }

    pub fn run(&mut self, input: i64) {
        let mut code = Code::new(self.program(self.counter));
        let mut was_input_used = false;

        self.state = loop {
            match code.opcode {
                Opcode::Exit => {
                    break State::Terminated;
                }
                _ => {
                    let index0 = match code.modes.0 {
                        Mode::Position => {
                            self.program(self.counter + 1) as usize
                        }
                        Mode::Immediate => self.counter + 1,
                        Mode::Relative => {
                            (self.program(self.counter + 1) + self.base)
                                as usize
                        }
                    };
                    match code.opcode {
                        Opcode::Add
                        | Opcode::Multiply
                        | Opcode::LessThan
                        | Opcode::Equals
                        | Opcode::JumpIfTrue
                        | Opcode::JumpIfFalse => {
                            let index1 = match code.modes.1 {
                                Mode::Position => {
                                    self.program(self.counter + 2) as usize
                                }
                                Mode::Immediate => self.counter + 2,
                                Mode::Relative => {
                                    (self.program(self.counter + 2)
                                        + self.base)
                                        as usize
                                }
                            };
                            match code.opcode {
                                Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                                    self.counter = match code.opcode {
                                        Opcode::JumpIfTrue => {
                                            if self.program(index0) != 0 {
                                                self.program(index1)
                                                    as usize
                                            } else {
                                                self.counter + 3
                                            }
                                        }
                                        Opcode::JumpIfFalse => {
                                            if self.program(index0) == 0 {
                                                self.program(index1)
                                                    as usize
                                            } else {
                                                self.counter + 3
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
                                        Mode::Position => self
                                            .program(self.counter + 3)
                                            as usize,
                                        Mode::Immediate => self.counter + 3,
                                        Mode::Relative => {
                                            (self.program(self.counter + 3)
                                                + self.base)
                                                as usize
                                        }
                                    };
                                    self.set_program(
                                        index2,
                                        match code.opcode {
                                            Opcode::Add => {
                                                self.program(index0)
                                                    + self.program(index1)
                                            }
                                            Opcode::Multiply => {
                                                self.program(index0)
                                                    * self.program(index1)
                                            }
                                            Opcode::LessThan => {
                                                if self.program(index0)
                                                    < self.program(index1)
                                                {
                                                    1
                                                } else {
                                                    0
                                                }
                                            }
                                            Opcode::Equals => {
                                                if self.program(index0)
                                                    == self.program(index1)
                                                {
                                                    1
                                                } else {
                                                    0
                                                }
                                            }
                                            _ => unreachable!(),
                                        },
                                    );
                                    self.counter += 4;
                                }
                                _ => unreachable!(),
                            }
                        }
                        Opcode::Input | Opcode::Output | Opcode::AdjustBase => {
                            match code.opcode {
                                Opcode::Input => if was_input_used {
                                    break State::Input
                                } else {
                                    self.set_program(index0, input);
                                    was_input_used = true;
                                },
                                Opcode::Output => {
                                    self.output = self.program(index0)
                                },
                                Opcode::AdjustBase => {
                                    self.base += self.program(index0);
                                }
                                _ => unreachable!(),
                            }
                            self.counter += 2;
                        }
                        _ => unreachable!(),
                    }
                }
            };

            code = Code::new(self.program(self.counter));
        }
    }
}

#[derive(Debug)]
pub enum State {
    Initialized,
    Terminated,
    Input,
}

#[derive(Debug)]
struct Code {
    opcode: Opcode,
    modes: (Mode, Mode, Mode),
}

impl Code {
    fn new(code: i64) -> Code {
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
    Relative,
}

impl Mode {
    fn new(s: &str) -> Mode {
        match s {
            "0" => Mode::Position,
            "1" => Mode::Immediate,
            "2" => Mode::Relative,
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
    AdjustBase,
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
            "09" => Opcode::AdjustBase,
            "99" => Opcode::Exit,
            _ => panic!("Opcode does not exist"),
        }
    }
}