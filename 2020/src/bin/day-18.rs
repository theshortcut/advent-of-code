use std::fs;

use advent_of_code_2020::day::day18::*;

fn main() {
    let contents =
        fs::read_to_string("src/day/day18.txt").expect("Something went wrong reading the file");
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}
