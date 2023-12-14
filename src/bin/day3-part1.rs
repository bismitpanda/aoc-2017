use itertools::Itertools;

fn main() {
    let input = include_str!(r"..\..\input\day3.txt")
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut sq = 0;

    for i in (1..).step_by(2) {
        if i * i > input {
            sq = i;
            break;
        }
    }

    let layer_dist = sq / 2;

    let cells = (((sq - 2) * (sq - 2) + 1)..=(sq * sq)).collect_vec();
    let cell_len = cells.len();
    let block = cells
        .chunks_exact(cell_len / 4)
        .filter(|chunk| chunk.contains(&input))
        .flatten()
        .copied()
        .collect_vec();

    let pos = block.iter().position(|&cell| cell == input).unwrap();
    let block_mid = cell_len / 8;

    let layer_offset = if pos < block_mid {
        block_mid - pos - 1
    } else {
        pos - block_mid + 1
    };

    let steps = layer_offset + layer_dist;

    dbg!(steps);
}
