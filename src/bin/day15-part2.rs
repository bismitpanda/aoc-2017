use itertools::Itertools;

const SET_16_BITS: usize = 0xFFFF;

fn main() {
    let (mut a, mut b) = include_str!(r"..\..\input\day15.txt")
        .lines()
        .map(|line| line.split(' ').last().unwrap().parse::<usize>().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut total = 0_usize;
    let mut pair = (None, None);

    let mut i = 0;
    while i < 5_000_000 {
        if pair.0.is_none() {
            a = (a * 16_807) % 2_147_483_647;
            if a % 4 == 0 {
                pair.0 = Some(a);
            }
        }
        if pair.1.is_none() {
            b = (b * 48_271) % 2_147_483_647;
            if b % 8 == 0 {
                pair.1 = Some(b);
            }
        }

        if let (Some(a), Some(b)) = pair {
            if a & SET_16_BITS == b & SET_16_BITS {
                total += 1;
            }

            pair = (None, None);
            i += 1;
        }
    }

    dbg!(total);
}
