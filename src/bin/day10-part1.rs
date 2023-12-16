use itertools::Itertools;

const NUM_COUNT: usize = 256;

fn main() {
    let lengths = include_str!(r"..\..\input\day10.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect_vec();

    let mut numbers = (0..NUM_COUNT).collect_vec();
    let mut start = 0;

    for (skip, length) in lengths.into_iter().enumerate() {
        let mut head = start;
        let mut tail = (start + length - 1) % NUM_COUNT;

        let steps = (length + 1) / 2;

        for _ in 0..steps {
            numbers.swap(head, tail);

            head = (head + 1) % NUM_COUNT;
            if tail == 0 {
                tail = NUM_COUNT - 1;
            } else {
                tail = (tail - 1) % NUM_COUNT;
            }
        }

        start = (start + length + skip) % NUM_COUNT;
    }

    let product = numbers.into_iter().take(2).product::<usize>();

    dbg!(product);
}
