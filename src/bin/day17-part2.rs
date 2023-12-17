fn main() {
    let steps = include_str!(r"..\..\input\day17.txt")
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut next = 0;
    let mut value = 0;

    for i in 1..50_000_000 {
        next = ((steps + next) % i) + 1;

        if next == 1 {
            value = i;
        }
    }

    dbg!(value);
}
