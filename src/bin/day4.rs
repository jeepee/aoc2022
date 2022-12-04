use std::{ops::RangeInclusive, str::FromStr};

use aoc2022::{run_and_print, Input, parse_pair};

struct SectionRange(RangeInclusive<usize>);

impl FromStr for SectionRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a,b) = parse_pair(s, "-");
        Ok(SectionRange(a..=b))
    }
}

impl SectionRange {
    fn fully_contains(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

fn run(input: Input) -> (usize, usize) {
    let ranges: Vec<(SectionRange,SectionRange)> = input
        .map(|line| parse_pair(&line, ","))
        .collect();

    let part1: usize = ranges
        .iter()
        .filter(|(a,b)| a.fully_contains(b) || b.fully_contains(a))
        .count();

    let part2: usize = ranges
        .iter()
        .filter(|(a,b)| a.overlaps(b) || b.overlaps(a))
        .count();

    (part1, part2)
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (2, 4))
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (540, 872))
    }
}