use std::fs;

use advent_of_code_2020::day::day03::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day03.txt").expect("Something went wrong reading the file");
  println!("Part 1: would hit {} trees", part1(&contents));
  println!(
    "Part 2: product of all slopes is {} trees",
    part2(&contents)
  );
}
