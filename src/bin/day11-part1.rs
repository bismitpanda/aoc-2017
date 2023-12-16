#[derive(Clone, Copy)]
struct Steps {
    ne: usize,
    n: usize,
    nw: usize,
    sw: usize,
    s: usize,
    se: usize,
}

impl Steps {
    const fn new() -> Self {
        Self {
            ne: 0,
            n: 0,
            nw: 0,
            sw: 0,
            s: 0,
            se: 0,
        }
    }

    const fn get_total(self) -> usize {
        self.ne + self.n + self.nw + self.sw + self.s + self.se
    }
}

fn main() {
    let mut steps = Steps::new();

    for step in include_str!(r"..\..\input\day11.txt").trim().split(',') {
        match step {
            "ne" => {
                if steps.sw > 0 {
                    steps.sw -= 1;
                } else if steps.nw > 0 {
                    steps.nw -= 1;
                    steps.n += 1;
                } else if steps.s > 0 {
                    steps.s -= 1;
                    steps.se += 1;
                } else {
                    steps.ne += 1;
                }
            }
            "n" => {
                if steps.s > 0 {
                    steps.s -= 1;
                } else {
                    steps.n += 1;
                }
            }
            "nw" => {
                if steps.se > 0 {
                    steps.se -= 1;
                } else if steps.ne > 0 {
                    steps.ne -= 1;
                    steps.n += 1;
                } else if steps.s > 0 {
                    steps.s -= 1;
                    steps.sw += 1;
                } else {
                    steps.nw += 1;
                }
            }
            "sw" => {
                if steps.ne > 0 {
                    steps.ne -= 1;
                } else if steps.se > 0 {
                    steps.se -= 1;
                    steps.s += 1;
                } else if steps.n > 0 {
                    steps.n -= 1;
                    steps.nw += 1;
                } else {
                    steps.sw += 1;
                }
            }
            "s" => {
                if steps.n > 0 {
                    steps.n -= 1;
                } else {
                    steps.s += 1;
                }
            }
            "se" => {
                if steps.nw > 0 {
                    steps.nw -= 1;
                } else if steps.sw > 0 {
                    steps.sw -= 1;
                    steps.s += 1;
                } else if steps.n > 0 {
                    steps.n -= 1;
                    steps.ne += 1;
                } else {
                    steps.se += 1;
                }
            }
            _ => {}
        }
    }

    let steps = steps.get_total();

    dbg!(steps);
}
