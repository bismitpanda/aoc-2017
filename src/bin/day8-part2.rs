use std::collections::HashMap;

use regex::Regex;

fn main() {
    let instruction_pattern =
        Regex::new(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) ([<>=!]+) (-?\d+)$").unwrap();

    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut max = 0;

    for line in include_str!(r"..\..\input\day8.txt").lines() {
        let captures = instruction_pattern.captures(line).unwrap();

        let lhs = registers.get(&captures[4]).copied().unwrap_or(0);
        let rhs = captures[6].parse::<isize>().unwrap();

        let cond = match &captures[5] {
            "==" => lhs == rhs,
            "!=" => lhs != rhs,
            "<" => lhs < rhs,
            ">" => lhs > rhs,
            "<=" => lhs <= rhs,
            ">=" => lhs >= rhs,
            _ => unreachable!(),
        };

        if cond {
            let amount = captures[3].parse::<isize>().unwrap();
            let amount = if &captures[2] == "dec" {
                -amount
            } else {
                amount
            };

            let reg = captures[1].to_string();

            registers
                .entry(reg.clone())
                .and_modify(|v| *v += amount)
                .or_insert(amount);

            if registers[&reg] > max {
                max = registers[&reg];
            }
        }
    }

    dbg!(max);
}
