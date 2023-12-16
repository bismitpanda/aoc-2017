use itertools::Itertools;

const NUM_COUNT: usize = 256;

fn main() {
    let lengths = include_str!(r"..\..\input\day10.txt")
        .trim()
        .bytes()
        .map(|byte| byte as usize)
        .collect_vec();

    let lengths = [lengths, vec![17, 31, 73, 47, 23]].concat();

    let mut numbers = (0..NUM_COUNT).collect_vec();
    let mut start = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for length in &lengths {
            let length = *length;

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
            skip += 1;
        }
    }

    let hash = hex::encode(
        numbers
            .chunks_exact(16)
            .map(|chunk| {
                u8::try_from(chunk.iter().copied().reduce(|acc, el| acc ^ el).unwrap()).unwrap()
            })
            .collect_vec(),
    );

    dbg!(hash);
}
