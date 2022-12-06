use aoc2022::{run_and_print, Input};

fn run(input: Input) -> Vec<(usize,usize)> {
    input.map(|line| (
        find_first_different(4, &line),
        find_first_different(14, &line)
    )).collect()
}

fn find_first_different(num: usize, line: &str) -> usize {
    let (pos,_) = line
        .as_bytes()
        .windows(num)
        .enumerate()
        .find(|(_, w)| all_different(w))
        .unwrap();
    pos + num
}

fn all_different<T: PartialEq>(s: &[T]) -> bool {
    s.iter()
     .map(|a| s.iter().filter(|b| a == *b).count())
     .all(|c| c == 1)
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, vec![(7, 19), (5, 23), (6, 23), (10, 29), (11, 26)])
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, vec![(1238, 3037)])
    }
}