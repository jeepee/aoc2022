use std::{iter::Peekable, fmt::Display};

use aoc2022::{Input, run_and_print};
use peeking_take_while::PeekableExt;

#[derive(Clone,Debug,Eq,PartialEq)]
enum Data {
    Number(usize),
    List(Vec<Data>),
}

impl Data {
    fn parse(input: &str) -> Self {
        Data::parse_iter(&mut input.chars().peekable())
    }

    fn parse_iter(input: &mut Peekable<impl Iterator<Item=char>>) -> Self {
        if input.next_if_eq(&'[').is_some() {
            let mut items = Vec::new();
            loop {
                if input.next_if_eq(&']').is_some() {
                    break;
                }
                items.push(Data::parse_iter(input));
                input.next_if_eq(&',');
            }
            Data::List(items)
        } else {
            let num = input
                .peeking_take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            Data::Number(num)
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Data::*;
        match (self, other) {
            (Number(x), Number(y)) => x.cmp(y),
            (List(xs), List(ys))   => xs.cmp(ys), // rust lexicographical ordering is just fine!
            (Number(x), List(_))   => List(vec![Number(*x)]).cmp(other),
            (List(_), Number(y))   => self.cmp(&List(vec![Number(*y)])),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Number(x) => write!(f, "{}", x)?,
            Data::List(xs) => {
                write!(f, "[")?;
                for (i,x) in xs.iter().enumerate() {
                    if i != 0 { write!(f, ",")?; }
                    write!(f, "{}", x)?;
                }
                write!(f, "]")?;
            },
        }
        Ok(())
    }
}

fn main() {
    run_and_print(run);
}

fn separator(num: usize) -> Data {
    Data::List(vec![Data::List(vec![Data::Number(num)])])
}

fn run(input: Input) -> (usize,usize) {
    let lines: Vec<String> = input.collect();
    let packets: Vec<(Data,Data)> = lines
        .chunks(3)
        .map(|lines| (Data::parse(&lines[0]), Data::parse(&lines[1])))
        .collect();
    
    let part1 = packets
        .iter()
        .enumerate()
        .filter(|(_,(x,y))| x <= y)
        .map(|(i,_)| i + 1)
        .sum();

    let sep1 = separator(2);
    let sep2 = separator(6);

    let mut index   = 1; // index of first separator, starts at 1
    let mut between = 1; // number of items between first and second separator (including 1 for first separator)
    for (a,b) in packets {
        if a < sep1      { index   += 1 }
        else if a < sep2 { between += 1 }
        if b < sep1      { index   += 1 }
        else if b < sep2 { between += 1 }
    }

    (part1, index*(index+between))
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (13,140));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (5529,27690));
    }
}