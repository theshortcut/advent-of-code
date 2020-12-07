use std::fs;

use advent_of_code_2020::day::day07::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day07.txt").expect("Something went wrong reading the file");
  println!(
    "Part 1: the number of bag colors that could contain a shiny gold bag is {}",
    part1(&contents)
  );
  println!(
    "Part 2: the number of bags that a shiny gold bag must contain is {}",
    part2(&contents)
  );
}
