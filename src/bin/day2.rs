use std::{str::FromStr, cmp::Ordering, fmt::Debug};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

use aoc2022::{parse_pair, run_and_print, Input};

#[derive(Copy,Clone,Debug,PartialEq,FromPrimitive)]
enum Item {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Copy,Clone,Debug)]
enum Action {
    Draw = 0,
    Win = 1,
    Lose = 2,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A"|"X" => Ok(Item::Rock),
            "B"|"Y" => Ok(Item::Paper),
            "C"|"Z" => Ok(Item::Scissors),
            _       => Err(()),
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Action::Lose),
            "Y" => Ok(Action::Draw),
            "Z" => Ok(Action::Win),
            _       => Err(()),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (3 + *self as u8 - *other as u8) % 3 {
            2 => Some(Ordering::Less),
            1 => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

fn single_score(a: Item, b: Item) -> usize {
    (b as usize + 1) + match b.partial_cmp(&a) {
        Some(Ordering::Less) => 0,
        Some(Ordering::Equal) => 3,
        Some(Ordering::Greater) => 6,
        None => unreachable!(),
    } as usize
}

fn action_to_item(a: Item, action: Action) -> Item {
    FromPrimitive::from_u8((a as u8 + action as u8) % 3).unwrap()
}

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (usize, usize) {
    let input: Vec<String> = input.collect();

    let score1: usize = input
        .iter()
        .map(|line| parse_pair::<Item,Item>(&line, " "))
        .map(|(a,b)| single_score(a, b))
        .sum();
    
    let score2: usize = input
        .iter()
        .map(|line| parse_pair::<Item,Action>(&line, " "))
        .map(|(a,b)| single_score(a, action_to_item(a, b)))
        .sum();

    (score1, score2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (15, 12));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (15422, 15442));
    }
}