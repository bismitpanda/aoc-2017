fn main() {
    let steps = include_str!(r"..\..\input\day17.txt")
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut lock = vec![0];
    let mut pos = 0;

    for i in 0..2017 {
        let new = ((pos + steps) % lock.len()) + 1;

        if new >= lock.len() {
            lock.push(i + 1);
        } else {
            lock.insert(new, i + 1);
        }
        pos = new;
    }

    let next = lock[pos + 1];
    dbg!(next);
}
