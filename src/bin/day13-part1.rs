use itertools::Itertools;

fn main() {
    let mut severity = 0;
    for (layer, depth) in include_str!(r"..\..\input\day13.txt").lines().map(|line| {
        line.split(": ")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        if layer % ((depth - 1) * 2) == 0 {
            severity += layer * depth;
        }
    }

    dbg!(severity);
}
