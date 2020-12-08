use std::fs;

use advent_of_code_2020::day::day08::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day08.txt").expect("Something went wrong reading the file");
  println!("Part 1: the acc value is: {}", part1(&contents));
  println!("Part 2: the acc value is: {}", part2(&contents));
}
