use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!(r"..\..\input\day3.txt")
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut map = HashMap::from([((0, 0), 1)]);
    let mut i = 0;
    let mut j = 0;

    for s in (1..).step_by(2) {
        for (ds, di, dj) in [(0, 1, 0), (0, 0, -1), (1, -1, 0), (1, 0, 1)] {
            for _ in 0..s + ds {
                i += di;
                j += dj;

                let value = (j - 1..j + 2)
                    .cartesian_product(i - 1..i + 2)
                    .map(|(l, k)| map.get(&(k, l)).copied().unwrap_or(0))
                    .sum();

                map.insert((i, j), value);

                if value > input {
                    dbg!(value);
                    return;
                }
            }
        }
    }
}
