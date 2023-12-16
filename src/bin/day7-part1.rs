use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let mut programs = HashMap::new();
    let line_pattern = Regex::new(r"(\w+) \(\d+\) -> ((\w+,? ?)+)").unwrap();
    for line in include_str!(r"..\..\input\day7.txt").lines() {
        if let Some(captures) = line_pattern.captures(line) {
            let name = captures[1].to_string();

            if let Some(children) = captures.get(2) {
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

    let programs_clone = programs.clone();

    for values in programs_clone.values() {
        for value in values {
            programs.remove(value);
        }
    }

    let root = programs.keys().next().unwrap();

    dbg!(root);
}
