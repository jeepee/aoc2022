use std::{collections::HashSet, cmp::{min, max}};

use aoc2022::{run_and_print, Input, parse_pair, point::{Point, Offset}};
use num_traits::{abs, signum};

type Direction = Offset<2>;

struct Rope {
    knots: Vec<Point<2>>,
}

impl Rope {
    fn new(size: usize) -> Self {
        assert!(size >= 2);
        Self { knots: vec![Point([0,0]);size] }
    }
     
    fn pull(&mut self, dir: Direction) -> Point<2> {
        // Move the head in the right direction, no restrictions apply.
        self.knots[0] += dir;

        for idx in 1..self.knots.len() {
            // Consider each pair of knots acting on eachother
            let head = self.knots[idx-1];
            let tail = &mut self.knots[idx];

            // Calculate the current offset between moved head and non-moved tail
            // If distance in either direction is <= 1, the tail doesn't move.
            // Else, the tail moves 1 position over in both directions.
            let mut offset = *tail - head;
            if abs(offset.0[0]) > 1 || abs(offset.0[1]) > 1 {
                offset.0[0] -= signum(offset.0[0]);
                offset.0[1] -= signum(offset.0[1]);
                *tail = head + offset;
            }
        }

        //self.display();

        // Return tail for easy collection of tail positions.
        self.knots
            .iter()
            .last()
            .cloned()
            .unwrap()
    }

    #[allow(dead_code)]
    fn display(&self) {
        let minx = min(0, self.knots.iter().map(|p| p.0[0]).min().unwrap());
        let maxx = max(5, self.knots.iter().map(|p| p.0[0]).max().unwrap());
        let miny = min(0, self.knots.iter().map(|p| p.0[1]).min().unwrap());
        let maxy = max(5, self.knots.iter().map(|p| p.0[1]).max().unwrap());
        
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                let p = Point([x,y]);
                let c = self.knots
                    .iter()
                    .position(|knot| *knot == p)
                    .map(|n| if n == 0 { "H".into() } else if n == self.knots.len() - 1 { "T".into() } else { n.to_string() })
                    .unwrap_or_else(|| if p == Point([0,0]) { "s".into() } else { ".".into() });
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn parse_line(line: String) -> impl Iterator<Item=Direction> {
    let (dir, count) = parse_pair::<String,usize>(&line, " ");
    let dir = match dir.as_str() {
        "U" => Offset([ 0, 1]),
        "D" => Offset([ 0,-1]),
        "L" => Offset([-1, 0]),
        "R" => Offset([ 1, 0]),
        _   => panic!("invalid input"),
    };
    std::iter::repeat(dir).take(count)
}

#[allow(dead_code)]
fn display_visited(visited: &HashSet<Point<2>>) {
    let minx = visited.iter().map(|p| p.0[0]).min().unwrap();
    let maxx = visited.iter().map(|p| p.0[0]).max().unwrap();
    let miny = visited.iter().map(|p| p.0[1]).min().unwrap();
    let maxy = visited.iter().map(|p| p.0[1]).max().unwrap();

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            let c = if visited.contains(&Point([x, y])) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

fn count_visited(directions: &[Direction], rope_knots: usize) -> usize {
    let mut rope = Rope::new(rope_knots);
    let visited = directions
        .iter()
        .map(|dir| rope.pull(*dir))
        .collect::<HashSet<_>>();
    //display_visited(&visited);
    
    visited.len()
}

fn run(input: Input) -> (usize,usize) {
    let directions = input
        .flat_map(parse_line)
        .collect::<Vec<_>>();

    (count_visited(&directions, 2), count_visited(&directions, 10))
}

fn main() {
    run_and_print(run);
}

#[cfg(test)]
mod test {
    use aoc2022::test::{test_example, test_puzzle, test_file};

    #[test]
    fn example() {
        test_example(crate::run, (13,1))
    }

    #[test]
    fn example2() {
        test_file(Some("test2"), crate::run, (88,36))
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (6563,2653))
    }
}