use std::fs;

use advent_of_code_2020::day::day09::*;

fn main() {
  let contents =
    fs::read_to_string("src/day/day09.txt").expect("Something went wrong reading the file");
  println!(
    "Part 1: the first value that breaks the rule is: {}",
    part1(&contents, 25)
  );
  println!(
    "Part 2: the encryption weakness is: {}",
    part2(&contents, 25)
  );
}
