use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn ex06() {
    match fs::read_to_string("src/inputs/ex06.txt") {
        Ok(s) => println!("Ex 06.1: {} | Ex 06.2: {}", sub1(&s), sub2(&s)),
        _ => println!("Cannot read ex06.txt"),
    }
}

pub fn sub1(s: &str) -> i32 {
    let orbits_str = s.split('\n').collect::<Vec<&str>>();
    let orbits = orbits_str
        .iter()
        .map(|o| Orbit::new(o))
        .collect::<Vec<Orbit>>();
    let (graph, root) = graph(orbits);
    count_paths(&graph, &root)
}

pub fn sub2(s: &str) -> i32 {
    let orbits_str = s.split('\n').collect::<Vec<&str>>();
    let orbits = orbits_str
        .iter()
        .map(|o| Orbit::new(o))
        .collect::<Vec<Orbit>>();
    let (graph, _) = graph(orbits);
    min_path(&graph, "YOU", "SAN")
}

fn graph(orbits: Vec<Orbit>) -> (Graph, String) {
    let mut graph = Graph::new();
    let mut centers = HashSet::new();
    let mut satellites = HashSet::new();
    for orbit in orbits.iter() {
        graph.add_child(&orbit.center, &orbit.satellite);
        centers.insert(&orbit.center);
        satellites.insert(&orbit.satellite);
    }
    let diff = centers.difference(&satellites).map(|n|
        n.to_string()
    ).collect::<Vec<String>>();
    let root = diff[0].to_string();
    (graph, root)
}

fn count_paths(graph: &Graph, root: &str) -> i32 {
    if graph.children(root).is_empty() {
        0
    } else {
        count_children(graph, root) + graph.children(root).iter().map(|c|
            count_paths(graph, c)
        ).sum::<i32>()
    }
}

fn count_children(graph: &Graph, root: &str) -> i32 {
    if graph.children(root).is_empty() {
        0
    } else {
        graph.children(root).len() as i32 + graph.children(root).iter().map(|c| 
            count_children(graph, c)
        ).sum::<i32>()
    }
}

fn min_path(graph: &Graph, from: &str, to: &str) -> i32 {
    discover(graph, from, to, vec![], 0) - 2
}

fn discover(graph: &Graph, from: &str, to: &str, seen: Vec<String>, steps: i32) -> i32 {
    if from == to {
        steps
    } else {
        let mut new_seen = seen.clone();
        new_seen.push(from.to_string());
        graph.family(from).iter().map(|n| {
            if !seen.contains(n) {
                discover(graph, n, to, new_seen.clone(), steps + 1)
            } else {
                0
            }
        }).sum()
    }
}

#[derive(Debug)]
struct Orbit {
    center: String,
    satellite: String,
}

impl Orbit {
    fn new(orbit_str: &str) -> Orbit {
        let args = orbit_str.split(')').collect::<Vec<&str>>();
        Orbit {
            center: args[0].to_string(),
            satellite: args[1].to_string(),
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Vec<String>>
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }
    
    fn add(&mut self, node: &str) {
        if !self.nodes.contains_key(node) {
            self.nodes.insert(node.to_string(), vec![]);
        }
    }
    
    fn add_child(&mut self, parent: &str, child: &str) {
        self.add(parent);
        match self.nodes.get_mut(parent) {
            None => unreachable!(),
            Some(children) => children.push(child.to_string())    
        }
    }

    fn children(&self, parent: &str) -> Vec<String> {
        match self.nodes.get(parent) {
            None => vec![],
            Some(children) => children.iter().map(|c| 
                c.to_string()
            ).collect::<Vec<String>>()
        }
    }

    fn parents(&self, child: &str) -> Vec<String> {
        let mut parents = vec![];
        for (key, value) in self.nodes.iter() {
            if value.contains(&child.to_string()) {
                parents.push(key.to_string());
            }
        }
        parents
    }

    fn family(&self, node: &str) -> Vec<String> {
        let mut family = vec![];
        for c in self.children(node).iter() {
            family.push(c.to_string());
        }
        for p in self.parents(node).iter() {
            family.push(p.to_string());
        }
        family
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub11() {
        assert_eq!(
            sub1("A)B\nB)E\nA)F\nC)D\nB)C"),
            9
        );
    }

    #[test]
    fn sub12() {
        assert_eq!(
            sub1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
            42
        );
    }

    #[test]
    fn sub22() {
        assert_eq!(
            sub2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
            4
        );
    }
}
