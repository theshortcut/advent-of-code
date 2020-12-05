use std::fs;

use advent_of_code_2020::day::day05::*;

fn main() {
    let contents =
        fs::read_to_string("src/day/day05.txt").expect("Something went wrong reading the file");
    println!("Part 1: highest seat id is {}", part1(&contents));
    println!("Part 2: your seat id is {}", part2(&contents));
}
