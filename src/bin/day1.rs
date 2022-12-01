use aoc2022::read_lines;

fn main() {
    let mut sums = vec![];
    let mut running = 0;
    for line in read_lines() {
        if line.len() == 0 {
            sums.push(running);
            running = 0;
        } else {
            running += line.parse::<i32>().unwrap();
        }
    }
    if running != 0 {
        sums.push(running);
    }

    let part1 = sums.iter().max().unwrap();
    println!("part1: {}", part1);

    sums.sort();
    let part2: i32 = sums.iter().rev().take(3).sum();
    println!("part2: {}", part2);
}