use itertools::Itertools;

fn main() {
    let mut banks = include_str!(r"..\..\input\day6.txt")
        .trim()
        .split('\t')
        .map(|num| num.parse::<usize>().unwrap())
        .collect_vec();

    let num_banks = banks.len();
    let mut prev_states = vec![banks.clone()];
    let mut steps = 0;

    loop {
        steps += 1;
        let max_pos = banks
            .iter()
            .enumerate()
            .max_set_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .first()
            .unwrap()
            .0;

        let mut prev_value = banks[max_pos];
        banks[max_pos] = 0;
        let mut idx = (max_pos + 1) % num_banks;
        while prev_value > 0 {
            banks[idx] += 1;
            idx = (idx + 1) % num_banks;
            prev_value -= 1;
        }

        if prev_states.contains(&banks) {
            break;
        }
        prev_states.push(banks.clone());
    }

    dbg!(steps);
}
