use itertools::Itertools;

fn main() {
    let sum = include_str!(r"..\..\input\day1.txt")
        .trim()
        .as_bytes()
        .iter()
        .map(|ch| (*ch - b'0') as usize)
        .circular_tuple_windows::<(_, _)>()
        .map(|(ch1, ch2)| if ch1 == ch2 { ch1 } else { 0 })
        .sum::<usize>();

    dbg!(sum);
}
