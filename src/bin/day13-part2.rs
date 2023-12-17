use itertools::Itertools;

fn main() {
    let firewall = include_str!(r"..\..\input\day13.txt")
        .lines()
        .map(|line| {
            line.split(": ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec();

    let mut delay = 0;

    while firewall
        .iter()
        .any(|&(layer, depth)| (layer + delay) % ((depth - 1) * 2) == 0)
    {
        delay += 1;
    }

    dbg!(delay);
}
