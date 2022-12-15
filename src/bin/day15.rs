use aoc2022::point::Offset;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;

use aoc2022::{Input, run_and_print, point::Point};

// Wrapper with same functionality as RangeInclusive<isize>
// Not using this one because I want to define an absolute ordering so merging can be done more efficiently.
#[derive(Copy,Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct Coverage(isize, isize);

impl Coverage {
    fn len(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }

    fn try_merge(&mut self, other: &Self) -> bool {
        if other.1 < self.0 - 1 || other.0 > self.1 + 1 {
            false
        } else {
            self.0 = min(self.0, other.0);
            self.1 = max(self.1, other.1);
            true
        }    
    }
}

#[derive(Debug)]
struct CombinedCoverage(Vec<Coverage>);

impl FromIterator<Coverage> for CombinedCoverage {
    fn from_iter<T: IntoIterator<Item = Coverage>>(iter: T) -> Self {
        // Fetch all and then sort (less expensive than inserting sorted).
        let mut coverages = Vec::from_iter(iter);
        coverages.sort();

        // When sorted on start-point, we can try merging with previous range or add as new otherwise.
        CombinedCoverage(coverages
            .iter()
            .fold(Vec::<Coverage>::new(), |mut acc,c| {
                if !acc.last_mut().map(|last| last.try_merge(c)).unwrap_or(false) {
                    acc.push(*c);
                }
                acc
            })
        )
    }
}

struct Sensor {
    sensor: Point<2>,
    beacon: Point<2>,
}

impl Sensor {
    fn parse(s: String) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-?\d+)").unwrap());
        let nums: Vec<isize> = RE
            .captures_iter(&s)
            .map(|m| m[0].parse().unwrap())
            .collect();

        Sensor {
            sensor: Point::new(&nums[0..2]),
            beacon: Point::new(&nums[2..4]),
        }
    }

    fn range(&self) -> usize {
        self.sensor.manhattan_dist(&self.beacon)
    }

    fn covers(&self, other: &Point<2>) -> bool {
        self.sensor.manhattan_dist(other) <= self.range()
    }

    fn row_coverage(&self, row: isize) -> Option<Coverage> {
        let x = self.sensor.0[0];
        let proj = Point::<2>([x, row]);
        let dist = self.sensor.manhattan_dist(&proj);
        let size = self.range() as isize - dist as isize;

        if size >= 0 {
            Some(Coverage(x-size, x+size))
        } else {
            None
        }
    }

    fn diags(&self) -> Diags {
        let offset = Offset::<2>([0, self.range() as isize]);
        let (h1,v1) = xy_to_uv(self.sensor - offset);
        let (h2,v2) = xy_to_uv(self.sensor + offset);
        Diags { us: [h1, h2], vs: [v1, v2] }
    }
}

#[derive(Debug,Eq,PartialEq)]
struct Diags {
    us: [isize;2],
    vs: [isize;2],
}

fn xy_to_uv(p: Point<2>) -> (isize, isize) {
    (p.0[0] + p.0[1], p.0[0] - p.0[1])
}

fn uv_to_xy((u, v): (isize, isize)) -> Point<2> {
    Point::<2>([(u + v) / 2, (u - v) / 2])
}

#[derive(Debug)]
struct DiagonalSet(HashSet<isize>);

impl DiagonalSet {
    // Iterating over all pairs of diagonals and finding the ones which have a gap of one between.
    fn single_gaps(&self) -> impl Iterator<Item=isize> + '_ {
        let diags = self.0.iter();
        diags
            .clone()
            .flat_map(move |d1| diags.clone().map(|d2| (*d1, *d2)))
            .filter(|(d1,d2)| d2 - d1 == 2)
            .map(|(d1,_)| d1 + 1)
    }
}

// Fancy dumping!
fn dump(sensors: &Vec<Sensor>, row: Option<isize>, full: bool) {
    let (xrange, yrange) = if full {
        // TODO: I really need a helper to extract x-y-ranges from input-data
        let minx = sensors.iter().map(|s| min(s.sensor.0[0], s.beacon.0[0])).min().unwrap();
        let maxx = sensors.iter().map(|s| max(s.sensor.0[0], s.beacon.0[0])).max().unwrap();
        let miny = sensors.iter().map(|s| min(s.sensor.0[1], s.beacon.0[1])).min().unwrap();
        let maxy = sensors.iter().map(|s| max(s.sensor.0[1], s.beacon.0[1])).max().unwrap();
        
        let minx = min(minx, sensors.iter().map(|s| s.sensor.0[0] - s.range() as isize).min().unwrap());
        let maxx = max(maxx, sensors.iter().map(|s| s.sensor.0[0] + s.range() as isize).max().unwrap());
        let miny = min(miny, sensors.iter().map(|s| s.sensor.0[1] - s.range() as isize).min().unwrap());
        let maxy = max(maxy, sensors.iter().map(|s| s.sensor.0[1] + s.range() as isize).max().unwrap());
        
        (minx..=maxx, miny..=maxy)
    } else {
        let center = row.unwrap();

        (0..=center*2, 0..=center*2)
    };

    let width = max(yrange.start().to_string().len(), yrange.end().to_string().len());

    println!("x={:?}", xrange);

    for y in yrange {
        if row == Some(y) {
            print!("\x1b[1;31m")
        }
        print!("{:>width$} ", y, width = width);
        for x in xrange.clone() {
            let p = Point::<2>([x,y]);
            let mut c = ' ';
            for s in sensors {
                if s.sensor == p {
                    c = 'S';
                    break;
                } else if s.beacon == p {
                    c = 'B';
                    break;
                } else if s.sensor.manhattan_dist(&p) <= s.range() {
                    c = '#';
                }
            }
            print!("{}", c);
        }
        if row == Some(y) {
            print!("\x1b[0m")
        }
        println!();
    }
}

// Collect all pairs of diagonals into two lists of both directions.
// Using hash-set for deduplication.
fn collect_diags(iter: impl Iterator<Item=Diags>) -> (DiagonalSet, DiagonalSet) {
    let mut us = HashSet::new();
    let mut vs = HashSet::new();

    for diag in iter {
        us.extend(diag.us);
        vs.extend(diag.vs);
    }

    (DiagonalSet(us), DiagonalSet(vs))
}

fn main() {
    run_and_print(run);
}

fn run(mut input: Input) -> (usize,usize) {
    // NOTE: in order to have different target rows for example vs puzzle without modifying the code,
    //       I have added the target-row to the input.
    //       Now we can parse the input-file and both runs and tests will pick the correct value.
    let row: isize = input.next().and_then(|s| s.parse().ok()).unwrap();
    let bounds = 0..=row*2;

    // Overrides from arguments for testing
    // - if the arg-list contains "debug", the grid will be printed.
    // - if the arg-list contains "full", the whole range of all sensors will be printed.
    // - if the arg-list contains a number, that row will be used for calculations (and will be highlighted)
    let row = env::args().find_map(|s| s.parse().ok()).unwrap_or(row);
    let debug = env::args().any(|s| s == "debug"); 
    let full = env::args().any(|s| s == "full");

    // Parse all sensors.
    let sensors: Vec<_> = input.map(Sensor::parse).collect();
    if debug {
        dump(&sensors, Some(row), full);
    }

    // Collect the coverage of each sensor of the requested row (if any)
    // and combine them all so that no cell is double-counted.
    let coverages: CombinedCoverage = sensors
        .iter()
        .filter_map(|sensor| sensor.row_coverage(row))
        .collect();
    let num_coverage: usize = coverages.0
        .iter()
        .map(Coverage::len)
        .sum();

    // find all beacons in the requested row, as these have to be subtracted
    let num_beacons: usize = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|beacon| beacon.0[1] == row)
        .collect::<HashSet<_>>()
        .len();

    let part1 = num_coverage - num_beacons;

    // Each sensor has a range in the shape of a rhombus.
    // If any gap exists it will lie between the diagonals of the rhombi.
    // We convert xy-space to uv-space (which is 45° rotated) in which the diagonals are
    // straight lines and can be expressed by a number.
    // Collecting all these numbers we only have look between pairs of diagonals that are
    // exactly 2 units apart, as there is only one hole and as such should be stradled by
    // diamonds in both directions.
    let (us,vs) = collect_diags(sensors.iter().map(|s| s.diags()));
    let us = us.single_gaps().collect::<HashSet<_>>();
    let vs = vs.single_gaps().collect::<HashSet<_>>();

    // Pairing up the gaps in both directions, converting back to xy and finding the one that is:
    // - out of reach of all sensors
    // - withing the target range
    // should result in a single point.
    let p = us
        .iter()
        .flat_map(|u| vs.iter().map(|v| (*u,*v)))
        .map(uv_to_xy)
        .filter(|point| bounds.contains(&point.0[0]) && bounds.contains(&point.0[1]))
        .find(|point| !sensors.iter().any(|s| s.covers(point)))
        .unwrap();

    let part2 = (p.0[0] * 4000000 + p.0[1]) as usize;

    (part1,part2)
}

#[cfg(test)]
mod test {
    use aoc2022::{test::{test_example, test_puzzle}, point::Point};

    use crate::{Coverage, xy_to_uv, uv_to_xy, Sensor, Diags};

    #[test]
    fn example() {
        test_example(crate::run, (26,56000011));
    }

    #[test]
    fn puzzle() {
        test_puzzle(crate::run, (4737567,13267474686239));
    }

    #[test]
    fn try_merge() {
        let mut a = Coverage(1, 5);
        let b = Coverage(6, 8);
        let ab = Coverage(1, 8);
        let c = Coverage(-5, 2);
        let abc = Coverage(-5, 8);
        let x = Coverage(10, 10);

        assert!(a.try_merge(&b));
        assert_eq!(a, ab);
        assert!(a.try_merge(&c));
        assert_eq!(a, abc);
        assert!(!a.try_merge(&x));
    }

    #[test]
    fn test_xy_to_hv() {
        assert_eq!((0, 0), xy_to_uv(Point::<2>([0, 0])));
        assert_eq!((1, 1), xy_to_uv(Point::<2>([1, 0])));
        assert_eq!((1, -1), xy_to_uv(Point::<2>([0, 1])));

        for x in -1..=1 {
            for y in -1..=1 {
                let p = Point::<2>([x, y]);
                let q = uv_to_xy(xy_to_uv(p));
                assert_eq!(p, q);
            }
        }
    }

    #[test]
    fn test_diag() {
        let s = Sensor { sensor: Point::<2>([1, 1]), beacon: Point::<2>([0, 0]) };
        let diags = s.diags();

        assert_eq!(diags, Diags {us: [0, 4], vs: [2, -2]});
    }
}