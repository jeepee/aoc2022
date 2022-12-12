use std::collections::HashSet;
use aoc2022::{run_and_print, Input};

#[derive(Default)]
struct Rucksack(HashSet<char>,HashSet<char>);

impl Rucksack {
    fn parse(s: String) -> Self {
        let l = s.len() / 2;
        Rucksack(
            s[..l].chars().into_iter().collect(),
            s[l..].chars().into_iter().collect()
        )
    }

    fn double_item(&self) -> char {
        *self.0.intersection(&self.1).next().unwrap()
    }

    fn take_combined_compartments(&mut self) -> HashSet<char> {
        self.0.extend(std::mem::take(&mut self.1).into_iter());
        std::mem::take(&mut self.0)
    }
}

fn priority(item: char) -> u32 {
    if item as u32 >= 'a' as u32 {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

fn find_common_item(rucksacks: &mut [Rucksack]) -> char {
    *rucksacks
        .iter_mut()
        .map(Rucksack::take_combined_compartments)
        .reduce(|acc,item| acc.intersection(&item).copied().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (u32, u32) {
    let mut rucksacks: Vec<Rucksack> = input.map(Rucksack::parse).collect();
    let part1: u32 = rucksacks
        .iter()
        .map(Rucksack::double_item)
        .map(priority)
        .sum();

    let part2: u32 = rucksacks
        .chunks_mut(3)
        .map(find_common_item)
        .map(priority)
        .sum();
    
    (part1, part2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (157, 70));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (7597, 2607));
    }
}