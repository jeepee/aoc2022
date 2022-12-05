use aoc2022::{run_and_print, Input, slice_mut_twice};

#[derive(Clone)]
struct Stacks(Vec<Vec<char>>);

#[derive(Copy,Clone)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

impl Stacks {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let lines: Vec<String> = lines.collect();
        let num_stacks = (lines[0].len() + 1) / 4;
        let mut stacks = Stacks(vec![vec![];num_stacks]);
        for line in lines.iter().rev().skip(1) {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .zip(stacks.0.iter_mut())
                .filter(|(c,_)| *c != ' ')
                .for_each(|(c,stack)| stack.push(c));
        }
        stacks
    }

    fn perform_move(&mut self, m: &Move) {
        let (from, to) = slice_mut_twice(&mut self.0, m.from, m.to);
        let idx = from.len() - m.num;
        to.extend(from.drain(idx..).rev());
    }

    fn perform_multi_move(&mut self, m: &Move) {
        let (from, to) = slice_mut_twice(&mut self.0, m.from, m.to);
        let idx = from.len() - m.num;
        to.extend(from.drain(idx..));
    }

    fn collect_top_crates(&self) -> String {
        self.0.iter().map(|stack| *stack.last().unwrap()).collect()
    }
}

impl Move {
    fn parse(s: String) -> Self {
        let mut numbers = s
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|s| s.parse().unwrap());
        Move {
            num:  numbers.next().unwrap(),
            from: numbers.next().unwrap() - 1,
            to:   numbers.next().unwrap() - 1,
        }
    }
}

fn run(mut input: Input) -> (String, String) {
    // Take input-iterator by_ref() in order not to consume it by take_while().
    // The input-iterator will remain valid and contain the remaining lines.
    let mut stacks = Stacks::parse(input.by_ref().take_while(|x| x.len() > 0));
    let moves = input.map(Move::parse).collect::<Vec<_>>();

    let part1 = {
        let mut stacks = stacks.clone();
        for m in &moves {
            stacks.perform_move(m);
        }
        stacks.collect_top_crates()
    };

    let part2 = {
        for m in &moves {
            stacks.perform_multi_move(m);
        }
        stacks.collect_top_crates()
    };

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
        test_example(crate::run, ("CMZ".into(), "MCD".into()))
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, ("WCZTHTMPS".into(), "BLSGJSDTS".into()))
    }
}