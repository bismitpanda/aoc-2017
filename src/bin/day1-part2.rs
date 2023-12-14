use itertools::Itertools;

fn main() {
    let input = include_str!(r"..\..\input\day1.txt").trim();
    let sum = input
        .as_bytes()
        .iter()
        .cycle()
        .take(input.len() + input.len() / 2)
        .map(|ch| (*ch - b'0') as usize)
        .collect_vec()
        .windows(input.len() / 2 + 1)
        .map(|window| {
            if *window.first().unwrap() == *window.last().unwrap() {
                *window.first().unwrap()
            } else {
                0
            }
        })
        .sum::<usize>();

    dbg!(sum);
}
