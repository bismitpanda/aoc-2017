fn main() {
    let mut input = include_str!(r"..\..\input\day9.txt").trim().chars();

    let mut score = 0;
    let mut in_group = 0;
    let mut in_garbage = false;

    while let Some(chr) = input.next() {
        match chr {
            '{' if !in_garbage => {
                in_group += 1;
            }

            '}' if !in_garbage => {
                score += in_group;
                in_group -= 1;
            }

            '<' => {
                in_garbage = true;
            }

            '>' => {
                in_garbage = false;
            }

            '!' if in_garbage => {
                input.next();
            }

            _ => {}
        }
    }

    dbg!(score);
}
