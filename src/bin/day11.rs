use std::fmt::Display;

use aoc2022::{Input, run_and_print};

#[derive(Clone)]
struct Inspection {
    oper: char,
    number: Option<usize>,
    divisor: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    inspection: Inspection,
}

impl Monkey {
    fn parse(lines: &[String]) -> Monkey {
        let items    = lines[1][18..].split(", ").map(|n| n.parse().unwrap()).collect();
        let oper     = lines[2].chars().nth(23).unwrap();
        let number   = lines[2][25..].parse::<usize>().ok();
        let divisor  = lines[3][21..].parse::<usize>().unwrap();
        let on_true  = lines[4][29..].parse::<usize>().unwrap();
        let on_false = lines[5][30..].parse::<usize>().unwrap();
        
        Monkey { items, inspection: Inspection { oper, number, divisor, on_true, on_false }}
    }

    fn inspect_items(&mut self, factor: usize, lcm: Option<usize>) -> Vec<(usize, usize)> {
        self.items.drain(..).map(|item| self.inspection.inspect_item(item, factor, lcm)).collect()
    }
}

impl Inspection {
    fn inspect_item(&self, item: usize, factor: usize, lcm: Option<usize>) -> (usize, usize) {
        let mut level = match self.oper {
            '+' => item + self.number.unwrap_or(item),
            '*' => item * self.number.unwrap_or(item),
            _   => panic!("invalid oper '{}'", self.oper),
        };
        level = level / factor;
        if let Some(lcm) = lcm {
            level = level % lcm;
        }
        let divisible = level % self.divisor == 0;
        (level, if divisible { self.on_true } else { self.on_false })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            write!(f, "{}, ", item)?;
        }
        Ok(())
    }
}

fn run_round(monkeys: &mut [Monkey], inspected: &mut [usize], factor: usize, lcm: Option<usize>) {
    for from in 0..monkeys.len() {
        for (level, to) in monkeys[from].inspect_items(factor, lcm) {
            monkeys[to].items.push(level);
            inspected[from] += 1;
        }
    }
}

#[allow(dead_code)]
fn debug_monkeys(monkeys: &[Monkey]) {
    for (n,monkey) in monkeys.iter().enumerate() {
        println!("monkey {}: {:?}", n, monkey.items);
    }
}

fn calc_monkey_business(monkeys: &mut [Monkey], rounds: usize, factor: usize, lcm: Option<usize>) -> usize {
    let num = monkeys.len();
    let mut inspected = vec![0;num];
    for _ in 0..rounds {
        run_round(monkeys, &mut inspected, factor, lcm);
        //debug_monkeys(&monkeys);
    }

    inspected.sort();
    inspected[num-1] * inspected[num-2]
}

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (usize,usize) {
    let lines: Vec<String> = input.collect();
    let mut monkeys: Vec<Monkey> = lines.chunks(7).map(Monkey::parse).collect();
    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.inspection.divisor)
        .fold(1, |acc,div| num::integer::lcm(acc, div));

    let part1 = calc_monkey_business(&mut monkeys.clone(), 20, 3, None);
    let part2 = calc_monkey_business(&mut monkeys, 10000, 1, Some(lcm));

    (part1, part2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (10605, 2713310158));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (113220,30599555965));
    }
}