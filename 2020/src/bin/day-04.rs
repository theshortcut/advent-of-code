use std::fs;

use advent_of_code_2020::day::day04::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day04.txt").expect("Something went wrong reading the file");
  println!("Part 1: {} valid batches", part1(&contents));
  println!("Part 2: {} valid batches", part2(&contents));
}
