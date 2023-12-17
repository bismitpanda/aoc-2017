use itertools::Itertools;

const SET_16_BITS: usize = 0xFFFF;

fn main() {
    let (mut a, mut b) = include_str!(r"..\..\input\day15.txt")
        .lines()
        .map(|line| line.split(' ').last().unwrap().parse::<usize>().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut total = 0_usize;

    for _ in 0..40_000_000 {
        a = (a * 16_807) % 2_147_483_647;
        b = (b * 48_271) % 2_147_483_647;

        if a & SET_16_BITS == b & SET_16_BITS {
            total += 1;
        }
    }

    dbg!(total);
}
