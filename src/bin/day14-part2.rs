use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const NUM_COUNT: usize = 256;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos(usize, usize);

fn hash(input: &str) -> [bool; 128] {
    let lengths = input.bytes().map(|byte| byte as usize).collect_vec();

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

    let mut hash = [false; 128];

    for (i, chunk) in numbers.chunks_exact(16).enumerate() {
        let chunk =
            u8::try_from(chunk.iter().copied().reduce(|acc, el| acc ^ el).unwrap()).unwrap();

        for j in 0..8 {
            if chunk & 0x80 >> j == 0x80 >> j {
                hash[i * 8 + j] = true;
            }
        }
    }

    hash
}

fn main() {
    let key = include_str!(r"..\..\input\day14.txt").trim();
    let mut grid = HashSet::new();

    for i in 0..128 {
        let a = hash(&format!("{key}-{i}"))
            .into_iter()
            .enumerate()
            .filter(|(_, cell)| *cell)
            .map(|(idx, _)| Pos(i, idx));

        grid.extend(a);
    }

    let mut regions = 0;
    let mut queue = VecDeque::new();

    while let Some(&k) = grid.iter().next() {
        grid.remove(&k);

        regions += 1;
        queue.push_back(k);

        while let Some(Pos(x, y)) = queue.pop_front() {
            queue.extend(grid.take(&Pos(x - 1, y)));
            queue.extend(grid.take(&Pos(x, y - 1)));
            queue.extend(grid.take(&Pos(x + 1, y)));
            queue.extend(grid.take(&Pos(x, y + 1)));
        }
    }

    dbg!(regions);
}
