use std::{collections::HashSet, str::FromStr};

use aoc2022::parse_input;

#[derive(Default)]
struct Rucksack(HashSet<char>,HashSet<char>);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.len() / 2;
        Ok(Rucksack(
            s[..l].chars().into_iter().collect(),
            s[l..].chars().into_iter().collect()
        ))
    }
}

impl Rucksack {
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
        .into_iter()
        .map(Rucksack::take_combined_compartments)
        .reduce(|acc,item| acc.intersection(&item).map(|x|*x).collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn main() {
    let mut rucksacks: Vec<Rucksack> = parse_input().collect();
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
    
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}