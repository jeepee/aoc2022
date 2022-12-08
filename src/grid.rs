use std::fmt::Display;

use crate::OwnedChars;

#[derive(Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid { rows, cols, data: Vec::new() }
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
        self.data.iter().skip(row * self.cols).take(self.cols)
    }

    pub fn row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item=&mut T> {
        self.data.iter_mut().skip(row * self.cols).take(self.cols)
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
            .iter()
            .filter(|(row, col)| *row >= 0 && *row < rows && *col >= 0 && *col < cols)
            .map(|(row,col)| (*row as usize, *col as usize))
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

pub struct Cell {
    pub row: usize,
    pub col: usize,
}

pub struct NeighborIter(Vec<(usize,usize)>);

impl Iterator for NeighborIter {
    type Item = (usize,usize);
    fn next(&mut self) -> Option<(usize,usize)> {
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

impl Grid<usize> {
    pub fn from_lines(lines: impl Iterator<Item=String>) -> Grid<usize> {
        let mut lines = lines.peekable();
        let cols = lines.peek().unwrap().len();
        let data: Vec<usize> = lines.flat_map(OwnedChars::from_string).map(|c| (c as usize) - ('0' as usize)).collect();

        Grid::from_data(data.len() / cols, cols, data)
    }
}
