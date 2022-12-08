use std::cmp::max;

use aoc2022::{run_and_print, Input, grid::Grid};

#[derive(Clone)]
struct Forest(Grid<u32>);

impl Forest {
    fn parse(input: Input) -> Self {
        let mut data = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for line in input {
            cols += 1;
            rows = line.len();
            data.extend(line.chars().map(|c| c.to_digit(10).unwrap() + 1))
        }

        Forest(Grid::from_data(rows, cols, data))
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let tree = *self.0.get(row, col);
        count_visible(tree, self.0.iter_up(row, col))
        * count_visible(tree, self.0.iter_down(row, col))
        * count_visible(tree, self.0.iter_left(row, col))
        * count_visible(tree, self.0.iter_right(row, col))
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let tree = *self.0.get(row, col);
        self.0.iter_up(row, col).all(|other| tree > *other)
        || self.0.iter_down(row, col).all(|other| tree > *other)
        || self.0.iter_left(row, col).all(|other| tree > *other)
        || self.0.iter_right(row, col).all(|other| tree > *other)
    }

    fn trees(&self) -> impl Iterator<Item=&u32> {
        self.0.data.iter()
    }
}

fn count_visible<'a>(tree: u32, trees: impl Iterator<Item=&'a u32>) -> usize {
    let mut count = 0;
    for other in trees {
        count += 1;
        if *other >= tree { break; }
    }
    count
}

fn propagate_max<'a>(trees: impl Iterator<Item=&'a mut u32>) {
    trees.fold(0, |acc,tree| {
        max(acc, std::mem::replace(tree, acc))
    });
}

fn num_visible_performant(forest: &Forest) -> usize {
    let mut top_down = forest.clone();
    let mut bottom_up = forest.clone();
    let mut left_right = forest.clone();
    let mut right_left = forest.clone();

    for row in 0..forest.0.rows {
        propagate_max(left_right.0.row_mut(row));
        propagate_max(right_left.0.row_mut(row).rev());
    }

    for col in 0..forest.0.cols {
        propagate_max(top_down.0.col_mut(col));
        propagate_max(bottom_up.0.col_mut(col).rev());
    }

    forest
        .trees()
        .zip(left_right.trees())
        .zip(right_left.trees())
        .zip(top_down.trees())
        .zip(bottom_up.trees())
        .filter(|((((t, a), b), c), d)| t > a || t > b || t > c || t > d)
        .count()
}
    
fn run(input: Input) -> (usize, usize) {
    let forest = Forest::parse(input);
 
    let _ = num_visible_performant(&forest);

    let num_visible = forest.0.iter_cells().filter(|cell| forest.is_visible(cell.row, cell.col)).count();
    let max_scenic_score = forest.0.iter_cells().map(|cell| forest.scenic_score(cell.row, cell.col)).max().unwrap();

    (num_visible, max_scenic_score)
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (21, 8))
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (1711, 301392))
    }
}