use std::collections::VecDeque;

use aoc2022::{run_and_print, Input};

#[derive(Clone,Copy,Debug)]
enum Op {
    Noop,
    Addx(isize),
}

impl Op {
    fn parse(s: String) -> Self {
        if &s == "noop" {
            Op::Noop
        } else if let Some(num) = s.strip_prefix("addx ").map(|s| s.parse::<isize>().ok()).flatten() {
            Op::Addx(num)
        } else { 
            panic!("invalid op: {}", s)
        }
    }

    fn latency(&self) -> usize {
        match self {
            Op::Noop    => 1,
            Op::Addx(_) => 2,
        }
    }
}

struct CPU {
    x: isize,
    ops: VecDeque<Op>,
    cycles: usize,
}

impl CPU {
    fn new<T: Iterator<Item=Op>>(ops: T) -> Self {
        let ops = ops.collect();
        Self { x: 1, ops, cycles: 0 }
    }

    fn tick(&mut self) {
        if let Some(op) = self.ops.pop_front() {
            self.cycles += 1;
            if self.cycles == op.latency() {
                match op {
                    Op::Noop        => {},
                    Op::Addx(delta) => self.x += delta,
                }
                self.cycles = 0;
            } else {
                self.ops.push_front(op);
            }
        }
    }
}

struct CRT {
    pixels: [char;40*6],
}

impl CRT {
    fn new() -> Self {
        CRT { pixels: [' ';40*6] }
    }

    fn tick(&mut self, cycle: usize, x: isize) {
        let col = (cycle % 40) as isize;
        if col >= x - 1 && col <= x + 1 {
            self.pixels[cycle] = '█'
        }
    }

    fn display(&self) {
        self.pixels
            .chunks(40)
            .for_each(|cs| println!("{}", String::from_iter(cs.iter())));
    }
}

fn tick_system_return_system_strength(cycle: usize, cpu: &mut CPU, crt: &mut CRT) -> isize {
    let x = cpu.x;
    crt.tick(cycle, x);
    cpu.tick();

    x * (cycle as isize + 1)
}

fn run(input: Input) -> isize {
    let mut cpu = CPU::new(input.map(Op::parse));
    let mut crt = CRT::new();

    // Knowing the size of the screen, emit a cycle for each pixel,
    // ticking the whole system and collecing the signal-strenght values for
    // the requested cycles.
    let part1 = (0..40*6)
        .into_iter()
        .map(|cycle| tick_system_return_system_strength(cycle, &mut cpu, &mut crt))
        .skip(19)
        .step_by(40)
        .sum();

    crt.display();

    part1
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, 13140)
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, 12840)
    }
}