use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let mut connections = include_str!(r"..\..\input\day12.txt")
        .lines()
        .map(|line| {
            let (from, tos) = line.split_once(" <-> ").unwrap();

            let from = from.parse::<usize>().unwrap();
            let tos = tos
                .split(", ")
                .map(|pipe| pipe.parse::<usize>().unwrap())
                .collect_vec();

            (from, tos)
        })
        .collect::<HashMap<_, _>>();

    let mut groups = 0;

    while !connections.is_empty() {
        groups += 1;
        let mut visited = HashSet::new();
        let mut pipes = VecDeque::from([*connections.keys().next().unwrap()]);

        while !pipes.is_empty() {
            let program = pipes.pop_front().unwrap();
            visited.insert(program);

            for pipe in &connections[&program] {
                if !visited.contains(pipe) {
                    pipes.push_back(*pipe);
                }
            }
        }

        connections = connections
            .into_iter()
            .filter(|&(k, _)| !visited.contains(&k))
            .collect();
    }

    dbg!(groups);
}
