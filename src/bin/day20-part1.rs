use regex::Regex;

fn main() {
    let particle_pattern =
        Regex::new(r"p=<([0-9\-]+),([0-9\-]+),([0-9\-]+)>, v=<([0-9\-]+),([0-9\-]+),([0-9\-]+)>, a=<([0-9\-]+),([0-9\-]+),([0-9\-]+)>")
            .unwrap();

    for line in include_str!(r"..\..\input\day20.txt").lines().take(2) {
        let captures = particle_pattern.captures(line).unwrap();
        dbg!(vec![
            captures[1].parse::<isize>().unwrap(),
            captures[2].parse::<isize>().unwrap(),
            captures[3].parse::<isize>().unwrap(),
            captures[4].parse::<isize>().unwrap(),
            captures[5].parse::<isize>().unwrap(),
            captures[6].parse::<isize>().unwrap(),
            captures[7].parse::<isize>().unwrap(),
            captures[8].parse::<isize>().unwrap(),
            captures[9].parse::<isize>().unwrap(),
        ]);
    }
}
