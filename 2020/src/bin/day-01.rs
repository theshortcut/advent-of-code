use std::fs;

use advent_of_code_2020::day::day01::{part1, part2};

fn main() {
    let contents =
        fs::read_to_string("src/day/day01.txt").expect("Something went wrong reading the file");
    let v: Vec<i32> = contents
        .split("\n")
        .filter_map(|w| w.parse().ok())
        .collect();
    println!("Part 1: {}", part1(&v));
    println!("Part 2: {}", part2(&v));
}
