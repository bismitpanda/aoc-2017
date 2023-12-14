use itertools::Itertools;

fn main() {
    let sum = include_str!(r"..\..\input\day2.txt")
        .lines()
        .map(|line| {
            let nums = line
                .split('\t')
                .map(|num| num.parse::<usize>().unwrap())
                .sorted()
                .collect_vec();

            *nums.last().unwrap() - *nums.first().unwrap()
        })
        .sum::<usize>();

    dbg!(sum);
}
