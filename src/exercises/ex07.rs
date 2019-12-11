use std::collections::{HashMap, HashSet};
use crate::helpers::file::{read, write};
use crate::helpers::computer::{Computer, State};

pub fn ex07() {
    let e = "07";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut signal: i64 = 0;
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
                            let amp_a = Amplifier::new(
                                Computer::new("A", program.clone()), 
                                s1,
                                AmpKind::Basic
                            );
                            let amp_b = Amplifier::new(
                                Computer::new("B", program.clone()), 
                                s2,
                                AmpKind::Basic
                            );
                            let amp_c = Amplifier::new(
                                Computer::new("C", program.clone()),
                                s3,
                                AmpKind::Basic
                            );
                            let amp_d = Amplifier::new(
                                Computer::new("D", program.clone()),
                                s4,
                                AmpKind::Basic
                            );
                            let amp_e = Amplifier::new(
                                Computer::new("E", program.clone()),
                                s5,
                                AmpKind::Output
                            );

                            let mut system: System = vec![
                                amp_a, 
                                amp_b, 
                                amp_c, 
                                amp_d, 
                                amp_e,
                            ];

                            let mut graph = Graph::new();
                            graph.add_child(0, 1);
                            graph.add_child(1, 2);
                            graph.add_child(2, 3);
                            graph.add_child(3, 4);

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

pub fn sub2(s: &str) -> i64 {
    let program = s
        .split(',')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<i64>>();
    let mut signal: i64 = 0;
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
                            let amp_a = Amplifier::new(
                                Computer::new("A", program.clone()),
                                s1,
                                AmpKind::Basic
                            );
                            let amp_b = Amplifier::new(
                                Computer::new("B", program.clone()),
                                s2,
                                AmpKind::Basic
                            );
                            let amp_c = Amplifier::new(
                                Computer::new("C", program.clone()),
                                s3,
                                AmpKind::Basic
                            );
                            let amp_d = Amplifier::new(
                                Computer::new("D", program.clone()),
                                s4,
                                AmpKind::Basic
                            );
                            let amp_e = Amplifier::new(
                                Computer::new("E", program.clone()),
                                s5,
                                AmpKind::Output
                            );

                            let mut system: System = vec![
                                amp_a, 
                                amp_b, 
                                amp_c, 
                                amp_d, 
                                amp_e,
                            ];

                            let mut graph = Graph::new();
                            graph.add_child(0, 1);
                            graph.add_child(1, 2);
                            graph.add_child(2, 3);
                            graph.add_child(3, 4);
                            graph.add_child(4, 0);

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

fn run(graph: &Graph, system: &mut System, from: usize, input: i64) -> i64 {
    system[from].run(input);
    let output = system[from].output();
    match graph.connections.get(&from) {
        Some(child_index) => match &system[from].kind {
            AmpKind::Basic => run(graph, system, *child_index, output),
            AmpKind::Output => {
                match &system[*child_index].state() {
                    State::Input => run(graph, system, *child_index, output),
                    _ => output
                }
            }
        },
        None => output,
    }
}

type System = Vec<Amplifier>;

#[derive(Debug)]
struct Graph {
    connections: HashMap<usize, usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn add_child(&mut self, from: usize, to: usize) {
        self.connections.insert(from, to);
    }
}

enum AmpKind {
    Basic,
    Output
}

struct Amplifier {
    computer: Computer,
    setting: i64,
    kind: AmpKind
}


impl Amplifier {
    fn new(computer: Computer, setting: i64, kind: AmpKind) -> Self {
        Self { computer, setting, kind }
    }

    fn run(&mut self, input: i64) {
        if let State::Initialized = self.computer.state() {
            self.computer.run(self.setting);
        }
        self.computer.run(input);
    }

    fn output(&self) -> i64 {
        self.computer.output()
    }

    fn state(&self) -> &State {
        self.computer.state()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
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
