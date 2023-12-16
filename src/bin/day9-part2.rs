fn main() {
    let mut input = include_str!(r"..\..\input\day9.txt").trim().chars();

    let mut in_garbage = false;
    let mut garbage = 0;

    while let Some(chr) = input.next() {
        if in_garbage {
            garbage += 1;
        }

        match chr {
            '<' => {
                in_garbage = true;
            }

            '>' => {
                in_garbage = false;
                garbage -= 1;
            }

            '!' if in_garbage => {
                input.next();
                garbage -= 1;
            }

            _ => {}
        }
    }

    dbg!(garbage);
}
