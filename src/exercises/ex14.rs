use crate::helpers::file::{read, write};
use std::collections::HashMap;


pub fn ex14() {
    let e = "14";
    let s = read(e);
    write(e, &sub1(&s).to_string(), &sub2(&s).to_string());
}

pub fn sub1(s: &str) -> i64 {
    let mut system = System::new();
    let mut unused = HashMap::new();
    for x in s.split('\n') {
        let element = Element::from_str(x);
        let name = &element.name.clone();
        system.insert(name.to_string(), element);
        unused.insert(name.to_string(), 0);
    }
    need(&system, 1, "FUEL", &mut unused)
}

pub fn sub2(s: &str) -> i64 {
    let mut system = System::new();
    let mut unused = HashMap::new();
    for x in s.split('\n') {
        let element = Element::from_str(x);
        let name = &element.name.clone();
        system.insert(name.to_string(), element);
        unused.insert(name.to_string(), 0);
    }
    let mut fuel = 0;
    let mut power = 6;
    loop {
        let mut unused = unused.clone();
        let ore = need(&system, fuel, "FUEL", &mut unused);
        if ore > 1_000_000_000_000 {
            if power > 0 {
                fuel -= 10_i64.pow(power);
                power -= 1;
            } else {
                fuel -= 1;
                break;
            }
        }
        fuel += 10_i64.pow(power);
    }
    fuel
}

fn need(system: &System, element_needed_nb: i64, element_name: &str, unused: &mut HashMap<String, i64>) -> i64 {
    if element_name == "ORE" {
        element_needed_nb
    } else {
        let element = system.get(element_name);
        let reaction = &element.reaction;
        let element_unused_nb = *unused.get(element_name).unwrap_or(&0);
        let (element_needed_nb, element_unused_nb) = if element_needed_nb > element_unused_nb {
            (element_needed_nb - element_unused_nb, 0)
        } else {
            (0, element_unused_nb - element_needed_nb)
        };
        let (reaction_nb, element_unused_nb) = if element_needed_nb == 0 {
            (0, element_unused_nb)
        } else {
            let reaction_nb = (element_needed_nb - 1) / reaction.output + 1;
            let element_created_nb = reaction_nb * reaction.output;
            (reaction_nb, element_created_nb - element_needed_nb)
        };
        unused.insert(element_name.to_string(), element_unused_nb);
        let mut nb = 0;
        for (index, parent) in system.parents(element_name).iter().enumerate() {
            let parent_nb = reaction.inputs[index];
            nb += need(system, reaction_nb * parent_nb, parent, unused);
        }
        nb
    }
}

#[derive(Debug)]
struct System {
    elements: HashMap<String, Element>
}

impl System {
    fn new() -> Self {
        Self { elements: HashMap::new() }
    }

    fn parents(&self, child: &str) -> Vec<String> {
        match self.elements.get(child) {
            Some(child) => child.parents.clone(),
            None => panic!("Element does not exist")
        }
    }

    fn insert(&mut self, name: String, element: Element) {
        self.elements.insert(name, element);
    }

    fn get(&self, name: &str) -> &Element {
        self.elements.get(name).unwrap()
    } 
}

#[derive(Debug)]
struct Reaction {
    output: i64,
    inputs: Vec<i64>,
}

impl Reaction {
    fn new(output: i64, inputs: Vec<i64>) -> Self {
        Self { output, inputs }
    } 
}

#[derive(Debug)]
struct Element {
    name: String,
    reaction: Reaction,
    parents: Vec<String>,
}

impl Element {
    fn from_str(s: &str) -> Self {
        let c = s.split(" => ").map(|x| 
            x.to_string()
        ).collect::<Vec<String>>();
        let inputs = c[0].split(", ").map(|input| {
            let x = input.split(' ').collect::<Vec<&str>>();
            (x[0].parse::<i64>().unwrap_or(0), x[1].to_string())
        }).collect::<Vec<(i64, String)>>();
        let (mut parents, mut inputs_nb) = (vec![], vec![]);
        for i in inputs.iter() {
            inputs_nb.push(i.0);
            parents.push(i.1.to_string());
        }
        let output = c[1].split(' ').collect::<Vec<&str>>();
        let output_nb = output[0].parse::<i64>().unwrap_or(0); 
        let output_name = output[1].to_string();
        Self { 
            name: output_name,
            reaction: Reaction::new(output_nb, inputs_nb),
            parents
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(sub1("7 ORE => 1 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 FUEL"), 99);
    }

    #[test]
    fn sub12() {
        assert_eq!(sub1("10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 FUEL"), 21);
    }

    #[test]
    fn sub13() {
        assert_eq!(sub1("10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL"), 31);
    }

    #[test]
    fn sub14() {
        assert_eq!(sub1("9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL"), 165);
    }

    #[test]
    fn sub15() {
        assert_eq!(sub1("157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312);
    }

    #[test]
    fn sub21() {
        assert_eq!(sub2("157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 82_892_753);
    }
}
