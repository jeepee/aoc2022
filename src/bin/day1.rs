use aoc2022::{Input, run_and_print};

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (i32, i32) {
    let mut sums = vec![];
    let mut running = 0;
    for line in input {
        if line.len() == 0 {
            sums.push(running);
            running = 0;
        } else {
            running += line.parse::<i32>().unwrap();
        }
    }
    if running != 0 {
        sums.push(running);
    }

    let part1 = *sums.iter().max().unwrap();
    sums.sort();
    let part2: i32 = sums.iter().rev().take(3).sum();

    (part1, part2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (24000, 45000));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (74711, 209481));
    }
}