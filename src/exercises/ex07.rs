use std::fs;
use std::collections::{HashSet, HashMap};

pub fn ex07() {
    match fs::read_to_string("src/inputs/ex07.txt") {
        Ok(s) => println!(
            "Ex 07.1: {} | Ex 07.2: {}",
            sub1(&s),
            sub2(&s)
        ),
        _ => println!("Cannot read ex07.txt"),
    }
}

pub fn sub1(s: &str) -> i32 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    let mut signal = 0;
    for s1 in 0..=4 {
        for s2 in 0..=4 {
            for s3 in 0..=4 {
                for s4 in 0..=4 {
                    for s5 in 0..=4 {
                        let mut h = HashSet::new();
                        h.insert(s1);
                        h.insert(s2);
                        h.insert(s3);
                        h.insert(s4);
                        h.insert(s5);
                        if h.len() == 5 {
                            let amp_a = Computer::new(program.clone(), s1);
                            let amp_b = Computer::new(program.clone(), s2);
                            let amp_c = Computer::new(program.clone(), s3);
                            let amp_d = Computer::new(program.clone(), s4);
                            let amp_e = Computer::new(program.clone(), s5);
                            
                            let mut system = System::new(vec![
                                Node::Input(0),
                                Node::Computer(amp_a),
                                Node::Computer(amp_b),
                                Node::Computer(amp_c),
                                Node::Computer(amp_d),
                                Node::Computer(amp_e),
                                Node::Output
                            ]);

                            let mut graph = Graph::new();
                            graph.add_child(0, 1);
                            graph.add_child(1, 2);
                            graph.add_child(2, 3);
                            graph.add_child(3, 4);
                            graph.add_child(4, 5);
                            graph.add_child(5, 6);
                            
                            let new_signal = run(&graph, &mut system, 0, 0);
                            
                            if new_signal > signal {
                                signal = new_signal;
                            }
                        }
                    }
                }
            }
        }    
    }
    signal
}

pub fn sub2(s: &str) -> i32 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();
    let mut signal = 0;
    for s1 in 5..=9 {
        for s2 in 5..=9 {
            for s3 in 5..=9 {
                for s4 in 5..=9 {
                    for s5 in 5..=9 {
                        let mut h = HashSet::new();
                        h.insert(s1);
                        h.insert(s2);
                        h.insert(s3);
                        h.insert(s4);
                        h.insert(s5);
                        if h.len() == 5 {
                            let amp_a = Computer::new(program.clone(), s1);
                            let amp_b = Computer::new(program.clone(), s2);
                            let amp_c = Computer::new(program.clone(), s3);
                            let amp_d = Computer::new(program.clone(), s4);
                            let amp_e = Computer::new(program.clone(), s5);
                            
                            let mut system = System::new(vec![
                                Node::Input(0),
                                Node::Computer(amp_a),
                                Node::Computer(amp_b),
                                Node::Computer(amp_c),
                                Node::Computer(amp_d),
                                Node::Computer(amp_e),
                                Node::Output
                            ]);

                            let mut graph = Graph::new();
                            graph.add_child(0, 1);
                            graph.add_child(1, 2);
                            graph.add_child(2, 3);
                            graph.add_child(3, 4);
                            graph.add_child(4, 5);
                            graph.add_child(5, 1);
                            graph.add_child(5, 6);
                            
                            let new_signal = run(&graph, &mut system, 0, 0);
                            
                            if new_signal > signal {
                                signal = new_signal;
                            }
                        }
                    }
                }
            }
        }    
    }
    signal
}

fn run(graph: &Graph, system: &mut System, from: usize, input: i32) -> i32 {
    let node = &mut system.nodes[from];
    match node {
        Node::Output => input,
        _ => {
            let value = match node {
                Node::Output => unreachable!(),
                Node::Input(v) => *v,
                Node::Computer(c) => {
                    c.set_input(input);
                    c.run()
                }
            };
            let children = match graph.connections.get(&from) {
                Some(c) => c,
                None => unreachable!(),
            };
            if children.len() > 1 {
                match node {
                    Node::Computer(c) => match c.state {
                        State::Terminated => run(graph, system, children[1], value),
                        _ => run(graph, system, children[0], value)
                    },
                    _ => unreachable!(),
                }
            } else {
                run(graph, system, children[0], value)
            }
        }
    }
}

struct System {
    nodes: Vec<Node>,
}

impl System {   
    fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }
}

#[derive(Debug)]
struct Graph {
    connections: HashMap<usize, Vec<usize>>,
}

impl Graph {   
    fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    fn add_child(&mut self, from: usize, to: usize) {
        self.connections.entry(from).or_insert_with(|| vec![]);
        match self.connections.get_mut(&from) {
            None => unreachable!(),
            Some(child) => child.push(to)
        }
    }
}

#[derive(Debug)]
enum Node {
    Computer(Computer),
    Input(i32),
    Output
}

#[derive(Debug)]
struct Computer {
    program: Vec<i32>,
    setting: i32,
    state: State,
    counter: usize,
    input: Option<i32>,
}

impl Computer {
    fn new(program: Vec<i32>, setting: i32) -> Self {
        Self { 
            program, 
            setting, 
            state: State::Initialized,
            counter: 0,
            input: None,
        }
    }

    fn set_input(&mut self, input: i32) {
        self.input = Some(input);
    }

    fn run(&mut self) -> i32 {
        let mut code = Code::new(self.program[self.counter]);
        let mut output = 0;

        loop {
            match code.opcode {
                Opcode::Exit => {
                    self.state = State::Terminated;
                    break;
                }
                _ => {
                    let index0 = match code.modes.0 {
                        Mode::Position => self.program[self.counter + 1] as usize,
                        Mode::Immediate => self.counter + 1,
                    };
                    match code.opcode {
                        Opcode::Add
                        | Opcode::Multiply
                        | Opcode::LessThan
                        | Opcode::Equals
                        | Opcode::JumpIfTrue
                        | Opcode::JumpIfFalse => {
                            let index1 = match code.modes.1 {
                                Mode::Position => self.program[self.counter + 2] as usize,
                                Mode::Immediate => self.counter + 2,
                            };
                            match code.opcode {
                                Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                                    self.counter = match code.opcode {
                                        Opcode::JumpIfTrue => {
                                            if self.program[index0] != 0 {
                                                self.program[index1] as usize
                                            } else {
                                                self.counter + 3
                                            }
                                        }
                                        Opcode::JumpIfFalse => {
                                            if self.program[index0] == 0 {
                                                self.program[index1] as usize
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
                                        Mode::Position => {
                                            self.program[self.counter + 3] as usize
                                        }
                                        Mode::Immediate => self.counter + 3,
                                    };
                                    self.program[index2] = match code.opcode {
                                        Opcode::Add => {
                                            self.program[index0] + self.program[index1]
                                        }
                                        Opcode::Multiply => {
                                            self.program[index0] * self.program[index1]
                                        }
                                        Opcode::LessThan => {
                                            if self.program[index0] < self.program[index1] {
                                                1
                                            } else {
                                                0
                                            }
                                        }
                                        Opcode::Equals => {
                                            if self.program[index0] == self.program[index1] {
                                                1
                                            } else {
                                                0
                                            }
                                        }
                                        _ => unreachable!(),
                                    };
                                    self.counter += 4;
                                }
                                _ => unreachable!(),
                            }
                        }
                        Opcode::Input | Opcode::Output => {
                            match code.opcode {
                                Opcode::Input => {
                                    match self.state {
                                        State::Initialized => {
                                            self.program[index0] = self.setting;
                                            self.state = State::Running;
                                        },
                                        State::Running => {
                                            match self.input {
                                                Some(input) => {
                                                    self.program[index0] = input;
                                                    self.input = None;
                                                },
                                                None => break
                                            }
                                        },
                                        State::Terminated => unreachable!()
                                    }
                                }
                                Opcode::Output => {
                                    output = self.program[index0];
                                }
                                _ => unreachable!(),
                            }
                            self.counter += 2;
                        }
                        _ => unreachable!(),
                    }
                }
            };

            code = Code::new(self.program[self.counter]);
        }

        output
    }
}

#[derive(Debug)]
enum State {
    Initialized,
    Running,
    Terminated,
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
        assert_eq!(sub1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139_629_729);
    }

    #[test]
    fn sub22() {
        assert_eq!(sub2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
