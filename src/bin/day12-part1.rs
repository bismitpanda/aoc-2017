use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let connections = include_str!(r"..\..\input\day12.txt")
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

    let mut visited = HashSet::new();
    let mut pipes = VecDeque::from([0]);

    while !pipes.is_empty() {
        let program = pipes.pop_front().unwrap();

        for pipe in &connections[&program] {
            if !visited.contains(pipe) {
                pipes.push_back(*pipe);
            }
        }

        visited.insert(program);
    }

    let count = visited.len();

    dbg!(count);
}
