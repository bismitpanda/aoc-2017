use itertools::Itertools;

fn main() {
    let sum = include_str!(r"..\..\input\day2.txt")
        .lines()
        .map(|line| {
            let nums = &line
                .split('\t')
                .map(|num| num.parse::<usize>().unwrap())
                .permutations(2)
                .filter(|perm| perm[0] % perm[1] == 0)
                .collect_vec()[0];

            *nums.first().unwrap() / *nums.last().unwrap()
        })
        .sum::<usize>();

    dbg!(sum);
}
