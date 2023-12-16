use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{algo::toposort, graph::DiGraph};
use regex::Regex;

struct Node {
    weight: usize,
    total: usize,
}

fn main() {
    let mut graph = DiGraph::new();
    let mut programs = HashMap::new();
    let mut indices = HashMap::new();

    let line_pattern = Regex::new(r"^(\w+) \((\d+)\)( -> )?((\w+,? ?)+)?$").unwrap();
    for line in include_str!(r"..\..\input\day7.txt").lines() {
        if let Some(captures) = line_pattern.captures(line) {
            let name = captures[1].to_string();
            let weight = captures[2].parse::<usize>().unwrap();

            let node_idx = graph.add_node(Node {
                weight,
                total: weight,
            });

            indices.insert(name.clone(), node_idx);

            if let Some(children) = captures.get(4) {
                programs.insert(
                    name,
                    children
                        .as_str()
                        .to_string()
                        .split(", ")
                        .map(ToString::to_string)
                        .collect_vec(),
                );
            }
        }
    }

    for (node, children) in programs {
        for child in children {
            graph.add_edge(
                *indices.get(&node).unwrap(),
                *indices.get(&child).unwrap(),
                (),
            );
        }
    }

    let sorted = toposort(&graph, None).unwrap();

    let mut unbalanced_weight = 0;
    for &node in sorted.iter().rev() {
        if !graph.neighbors(node).map(|n| graph[n].total).all_equal() {
            let (min, max) = graph
                .neighbors(node)
                .map(|n| graph[n].total)
                .minmax()
                .into_option()
                .unwrap();

            let (left, right): (Vec<_>, Vec<_>) =
                graph.neighbors(node).partition(|&n| graph[n].total == min);

            let unbalanced = if left.len() == 1 {
                &graph[left[0]]
            } else {
                &graph[right[0]]
            };

            unbalanced_weight = unbalanced.weight + min - max;
            break;
        }

        graph[node].total += graph.neighbors(node).map(|n| graph[n].total).sum::<usize>();
    }

    dbg!(unbalanced_weight);
}
