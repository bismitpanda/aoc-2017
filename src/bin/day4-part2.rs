use itertools::Itertools;

fn main() {
    let count = include_str!(r"..\..\input\day4.txt")
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|phrase| phrase.chars().sorted().collect::<String>())
                .sorted()
                .collect_vec()
        })
        .filter(|passphrase| {
            let mut passphrase = passphrase.clone();

            let orig_len = passphrase.len();
            passphrase.dedup();
            let deduped_len = passphrase.len();

            orig_len == deduped_len
        })
        .count();

    dbg!(count);
}
