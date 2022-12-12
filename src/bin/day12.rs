use aoc2022::{Input, run_and_print, grid::{Grid, Cell, Dir}};
use pathfinding::{directed::dijkstra, prelude::build_path};

struct Map {
    grid: Grid<char>,
    start: Cell,
    end: Cell,
}

impl Map {
    fn parse(input: Input) -> Self {
        let mut grid = Grid::from_lines(input, |c| c);

        let start = grid.find(&'S').unwrap();
        let end = grid.find(&'E').unwrap();
        *grid.get_mut(start.row, start.col) = 'a';
        *grid.get_mut(end.row, end.col) = 'z';

        Self { grid, start, end }
    }
    
    // searching reverse path (from end to start) allows to find all paths leading to end
    // using the dijkstra algorithm
    fn reachable_from(&self, row: usize, col: usize) -> Vec<(Cell, usize)> {
        let min = *self.grid.get(row, col) as usize - 1;
        self.grid
            .neighbors(row, col, false)
            .filter(|neighbor| *self.grid.get(neighbor.row, neighbor.col) as usize >= min)
            .map(|n| (n.clone(), 1))
            .collect()
    }

    #[allow(dead_code)]
    fn display_path(&self, path: &[Cell]) {
        let mut grid = Grid::new(self.grid.rows, self.grid.cols, ' ');
        let dirs: Vec<Dir> = path
            .windows(2)
            .map(|w| Dir::between(&w[0], &w[1]).unwrap())
            .collect();
        
        for (d,loc) in dirs.windows(2).zip(path.iter().skip(1)) {
            use Dir::*;
            let c = match (&d[0], &d[1]) {
                (Right, Right) | (Left, Left) => '═',
                (Down , Down)  | (Up  , Up  ) => '║',
                (Up   , Right) | (Left, Down) => '╔',
                (Right, Down ) | (Up  , Left) => '╗',
                (Down , Right) | (Left, Up  ) => '╚',
                (Right, Up   ) | (Down, Left) => '╝',
                _                                => panic!("invalid path"),
            };
            
            *grid.get_mut(loc.row, loc.col) = c;
        }

        // path contains reverse path, so switch E and S labels
        let end = path.last().unwrap();
        *grid.get_mut(path[0].row, path[0].col) = 'E';
        *grid.get_mut(end.row, end.col) = 'S';

        println!("{}", grid);
    }
}

fn main() {
    run_and_print(run);
}

fn run(input: Input) -> (usize, usize) {
    let map   = Map::parse(input);
    let paths = dijkstra::dijkstra_all(&map.end, |p| map.reachable_from(p.row, p.col));
    let part1 = paths[&map.start].1;
    let (start,&(_,part2)) = paths
        .iter()
        .filter(|(cell,_)| *map.grid.get(cell.row, cell.col) == 'a')
        .min_by_key(|(_,(_,count))| count)
        .unwrap();
    
    map.display_path(&build_path(&map.start, &paths));
    map.display_path(&build_path(start, &paths));

    (part1, part2)
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle};

    #[test]
    fn example() {
        test_example(crate::run, (31,29));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (370,363));
    }
}