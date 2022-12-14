use std::{fmt::Display, cmp::max};

use num::signum;

use crate::OwnedChars;

#[derive(Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(rows: usize, cols: usize, elem: T) -> Self {
        Grid { rows, cols, data: vec![elem; rows * cols]}
    }
}

impl<T> Grid<T> {
    pub fn from_data(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Grid { rows, cols, data }
    }

    pub fn iter_cells(&self) -> impl Iterator<Item=Cell> + '_ {
        (0..self.rows).flat_map(move |row| (0..self.cols).map(move |col| Cell { row, col }))
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }
    
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
    
    pub fn data(&self) -> impl Iterator<Item=&T> {
        self.data.iter()
    }

    pub fn row(&self, row: usize) -> impl DoubleEndedIterator<Item=&T> {
        self.data[row * self.cols..(row+1)*self.cols].iter()
    }

    pub fn row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item=&mut T> {
        self.data[row * self.cols..(row+1)*self.cols].iter_mut()
    }
    
    pub fn col(&self, col: usize) -> impl DoubleEndedIterator<Item=&T> {
        self.data.iter().skip(col).step_by(self.cols)
    }
    
    pub fn col_mut(&mut self, col: usize) -> impl DoubleEndedIterator<Item=&mut T> {
        self.data.iter_mut().skip(col).step_by(self.cols)
    }
    
    pub fn neighbors(&self, row: usize, col: usize, diag: bool) -> NeighborIter {
        let (row, col)   = (row as isize, col as isize);
        let (rows, cols) = (self.rows as isize, self.cols as isize);
        let coords = if diag {
            vec!((row-1,col-1),(row,col-1),(row+1,col-1),(row-1,col),(row+1,col),(row-1,col+1),(row,col+1),(row+1,col+1))
        } else {
            vec!((row-1,col),(row+1,col),(row,col-1),(row,col+1))
        };

        NeighborIter(coords
            .into_iter()
            .filter(|(row, col)| *row >= 0 && *row < rows && *col >= 0 && *col < cols)
            .map(|(row,col)| Cell{row: row as usize, col: col as usize})
            .collect()
        )
    }

    pub fn iter_up(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        self.col(col).rev().skip(self.rows - row)
    }

    pub fn iter_down(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        self.col(col).skip(row + 1)
    }

    pub fn iter_left(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        self.row(row).rev().skip(self.cols - col)
    }

    pub fn iter_right(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        self.row(row).skip(col + 1)
    }
}

impl<T> Grid<T> where T: PartialEq {
    pub fn find(&self, item: &T) -> Option<Cell> {
        self.data
            .iter()
            .position(|e| e == item)
            .map(|idx| Cell { row: idx / self.cols, col: idx % self.cols })
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

pub struct NeighborIter(Vec<Cell>);

impl Iterator for NeighborIter {
    type Item = Cell;
    fn next(&mut self) -> Option<Cell> {
        self.0.pop()
    }
}

impl<T> Display for Grid<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for chunk in self.data.chunks(self.cols) {
            for item in chunk {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn from_lines<F>(lines: impl Iterator<Item=String>, f: F) -> Grid<T>
    where F: Fn(char) -> T
    {
        let mut lines = lines.peekable();
        let cols = lines.peek().unwrap().len();
        let data: Vec<T> = lines.flat_map(OwnedChars::from_string).map(f).collect();

        Grid::from_data(data.len() / cols, cols, data)
    }
}
pub enum Dir {
    Up, Down, Left, Right,
    UpLeft, UpRight, DownLeft, DownRight,
}

impl Dir {
    pub fn between(from: &Cell, to: &Cell) -> Option<Self> {
        use Dir::*;
        match (to.row as isize - from.row as isize, to.col as isize - from.col as isize) {
            (-1, 0) => Some(Up),
            (-1, 1) => Some(UpRight),
            ( 0, 1) => Some(Right),
            ( 1, 1) => Some(DownRight),
            ( 1, 0) => Some(Down),
            ( 1,-1) => Some(DownLeft),
            ( 0,-1) => Some(Left),
            (-1,-1) => Some(UpLeft),
            _       => None,
        }
    }
}

pub struct LineIter {
    next: Cell,
    step: (isize, isize),
    count: usize,
}

impl LineIter {
    pub fn new(from: Cell, to: Cell) -> Option<Self> {
        let step = (signum(to.row as isize - from.row as isize), signum(to.col as isize - from.col as isize));
        let count = max(to.row.abs_diff(from.row), to.col.abs_diff(from.col)) + 1;
        if step.0 != 0 && step.1 != 0 {
            return None
        }

        Some(LineIter { next: from, step, count })
    }
}

impl Iterator for LineIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let next = self.next;
        self.next.row = (self.next.row as isize + self.step.0) as usize;
        self.next.col = (self.next.col as isize + self.step.1) as usize;
        self.count -= 1;

        Some(next)
    }
}