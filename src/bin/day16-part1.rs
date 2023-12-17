use itertools::Itertools;

fn main() {
    let mut programs = (0..16)
        .map(|idx| char::from_u32(idx + 97).unwrap())
        .collect_vec();

    let instructions = include_str!(r"..\..\input\day16.txt")
        .trim()
        .split(',')
        .collect_vec();

    for instruction in &instructions {
        let mut chars = instruction.chars();
        let cmd = chars.next().unwrap();
        let op = chars.collect::<String>();

        match cmd {
            'x' => {
                let (from, to) = op
                    .split('/')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();

                programs.swap(from, to);
            }

            's' => {
                let op = op.parse::<usize>().unwrap();
                programs.rotate_right(op);
            }

            'p' => {
                let (from, to) = op
                    .split('/')
                    .map(|num| num.parse::<char>().unwrap())
                    .collect_tuple()
                    .unwrap();

                let from = programs.iter().position(|ch| *ch == from).unwrap();
                let to = programs.iter().position(|ch| *ch == to).unwrap();

                programs.swap(from, to);
            }

            _ => unreachable!(),
        }
    }

    let programs = programs.iter().collect::<String>();

    dbg!(programs);
}
