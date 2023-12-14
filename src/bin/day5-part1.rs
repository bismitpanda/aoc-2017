use itertools::Itertools;

fn main() {
    let mut jumps = include_str!(r"..\..\input\day5.txt")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec();

    let mut index = 0;
    let mut steps = 0;

    while let Some(offset) = jumps.get_mut(usize::try_from(index).unwrap()) {
        index += *offset;
        *offset += 1;
        steps += 1;
    }

    dbg!(steps);
}
