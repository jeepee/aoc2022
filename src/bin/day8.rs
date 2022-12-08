use std::{cmp::max, fmt::Display};

use aoc2022::{run_and_print, Input};

#[derive(Clone)]
struct Forest {
    trees: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Forest {
    fn parse(input: Input) -> Self {
        let mut trees = Vec::new();
        let mut rows = 0;
        let mut cols = 0;

        for line in input {
            cols += 1;
            rows = line.len();
            trees.extend(line.chars().map(|c| c.to_digit(10).unwrap() + 1))
        }

        Forest { trees, rows, cols }
    }

    fn iter_locations(&self) -> impl Iterator<Item=(usize,usize)> + '_ {
        (0..self.rows).flat_map(|row| (0..self.cols).map(move |col| (row, col)))
    }

    fn tree(&self, row: usize, col: usize) -> u32 {
        self.trees[row * self.cols + col]
    }
    
    fn trees(&self) -> impl Iterator<Item=&u32> {
        self.trees.iter()
    }

    fn row(&self, row: usize) -> impl DoubleEndedIterator<Item=&u32> {
        self.trees.iter().skip(row * self.cols).take(self.cols)
    }

    fn row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item=&mut u32> {
        self.trees.iter_mut().skip(row * self.cols).take(self.cols)
    }
    
    fn col(&self, col: usize) -> impl DoubleEndedIterator<Item=&u32> {
        self.trees.iter().skip(col).step_by(self.cols)
    }
    
    fn col_mut(&mut self, col: usize) -> impl DoubleEndedIterator<Item=&mut u32> {
        self.trees.iter_mut().skip(col).step_by(self.cols)
    }

    fn trees_up(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> {
        self.col(col).rev().skip(self.rows - row)
    }

    fn trees_down(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> {
        self.col(col).skip(row + 1)
    }

    fn trees_left(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> {
        self.row(row).rev().skip(self.cols - col)
    }

    fn trees_right(&self, row: usize, col: usize) -> impl Iterator<Item = &u32> {
        self.row(row).skip(col + 1)
    }
    
    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let tree = self.tree(row, col);
        count_visible(tree, self.trees_up(row, col))
        * count_visible(tree, self.trees_down(row, col))
        * count_visible(tree, self.trees_left(row, col))
        * count_visible(tree, self.trees_right(row, col))
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let tree = self.tree(row, col);
        self.trees_up(row, col).all(|other| tree > *other)
        || self.trees_down(row, col).all(|other| tree > *other)
        || self.trees_left(row, col).all(|other| tree > *other)
        || self.trees_right(row, col).all(|other| tree > *other)
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for tree in self.row(row) {
                write!(f, "{:x}", tree)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
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

    for row in 0..forest.rows {
        propagate_max(left_right.row_mut(row));
        propagate_max(right_left.row_mut(row).rev());
    }

    for col in 0..forest.cols {
        propagate_max(top_down.col_mut(col));
        propagate_max(bottom_up.col_mut(col).rev());
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

    let num_visible = forest.iter_locations().filter(|(row, col)| forest.is_visible(*row, *col)).count();
    let max_scenic_score = forest.iter_locations().map(|(row, col)| forest.scenic_score(row, col)).max().unwrap();

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