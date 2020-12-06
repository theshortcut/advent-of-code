use std::fs;

use advent_of_code_2020::day::day06::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day06.txt").expect("Something went wrong reading the file");
  println!(
    "Part 1: sum of group distinct yes answers is {}",
    part1(&contents)
  );
  println!(
    "Part 2: sum of group unanimous yes answers is {}",
    part2(&contents)
  );
}
