use std::{collections::HashSet, fmt::Display, iter, env};

use aoc2022::{Input, run_and_print, parse_pair, grid::{Cell, LineIter}};


struct Cave {
    walls: HashSet<Cell>,
    sand: HashSet<Cell>,
    source: Cell,
    bottom_wall: usize,
    last_trail: Vec<Cell>,
}

impl Cave {
    pub fn from_input(input: Input) -> Self {
        let mut walls = HashSet::new();
        let obstacles: Vec<_> = input.map(parse_line).collect();
        for obstacle in &obstacles {
            for corners in obstacle[..].windows(2) {
                for cell in LineIter::new(corners[0], corners[1]).unwrap() {
                    walls.insert(cell);
                }
            }
        }

        let sand = HashSet::new();
        let last_trail = Vec::new();
        let bottom_wall = walls.iter().map(|cell| cell.row).max().unwrap();
        let source = Cell { row: 0, col: 500 };

        Cave { walls, sand, source, bottom_wall, last_trail }
    }

    /// Drops a unit of sand and returns if it has landed.
    /// This can be false in case there is no floor and the 
    fn drop_sand_unit(&mut self, has_floor: bool) -> bool {
        // Start from where we last were (this saves a lot of double work!)
        // or the source in case we don't have any trail yet.
        let mut sand = self.last_trail.pop().unwrap_or(self.source);

        loop {
            if has_floor {
                // Check for implicit floor if present,
                if sand.row == self.bottom_wall + 1 {
                    self.sand.insert(sand);
                    return true;
                }
            } else {
                // else check for falling below lowest obstacle.
                if sand.row >= self.bottom_wall + 2 {
                    return false
                }
            }

            // Try falling down or else returning we have landed
            if let Some(next) = self.fall_next(sand) {
                self.last_trail.push(sand);
                sand = next;
            } else {
                self.sand.insert(sand);
                return true;
            }
        }
    }

    #[inline]
    fn available(&self, cell: &Cell) -> bool {
        !self.walls.contains(cell) && !self.sand.contains(cell)
    }

    fn fall_next(&self, mut sand: Cell) -> Option<Cell> {
        sand.row += 1;
        if self.available(&sand) { return Some(sand) }

        sand.col -= 1;
        if self.available(&sand) { return Some(sand) }

        sand.col += 2;
        if self.available(&sand) { return Some(sand) }

        None
    }

    fn is_blocked(&self) -> bool {
        self.sand.contains(&self.source)
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.walls.iter().chain(self.sand.iter()).chain(iter::once(&self.source));
        let row_min = cells.clone().map(|cell| cell.row).min().unwrap();
        let row_max = cells.clone().map(|cell| cell.row).max().unwrap();
        let col_min = cells.clone().map(|cell| cell.col).min().unwrap();
        let col_max = cells.clone().map(|cell| cell.col).max().unwrap();

        for row in row_min..=row_max {
            for col in col_min..=col_max {
                let cell = Cell { row, col };
                let c = if self.walls.contains(&cell) {
                    '█'
                } else if self.sand.contains(&cell) {
                    '░'
                } else if cell == self.source {
                    '+'
                } else if self.last_trail.contains(&cell) {
                    '~'
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?; 
        }
        Ok(())
    }
}

fn parse_line(line: String) -> Vec<Cell> {
    line
        .split(" -> ")
        .map(|s| parse_pair(s, ","))
        .map(|(x,y)| Cell{row: y, col: x})
        .collect()
}

fn get_input() -> usize {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse::<usize>().unwrap_or(0)
}

fn get_visualizer() -> impl FnMut(usize, &Cave) {
    let debug = env::args().any(|arg| arg == "debug");
    let mut skip = 0;
    move |count, cave| {
        if debug {
            if skip == 0 {
                print!("\x1B[2J\x1B[H");
                println!("After {} sand units:", count);
                println!("{}", cave);
                skip = get_input();
            } else {
                skip -= 1;
            }
        }
    }
}

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (usize,usize) {
    let mut cave = Cave::from_input(input);

    // some visualization when "debug" is given as argument
    let mut visualize = get_visualizer();
    visualize(0, &cave);

    // part1: drop until it falls below lowest obstacle
    let mut count = 0;
    while cave.drop_sand_unit(false) {
        count += 1;
        visualize(count, &cave);
    }
    let part1 = count;

    // part2: drop until source is blocked
    while !cave.is_blocked() {
        cave.drop_sand_unit(true);
        count += 1;
        visualize(count, &cave);
    }
    let part2 = count;

    (part1,part2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (24,93));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (674,24958));
    }
}