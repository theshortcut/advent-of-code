use std::fs;

use advent_of_code_2020::day::day02::{part1, part2};

fn main() {
  let contents =
    fs::read_to_string("src/day/day02.txt").expect("Something went wrong reading the file");
  let v: Vec<&str> = contents.split("\n").collect();
  println!("Part 1: {} valid passwords", part1(&v));
  println!("Part 2: {} valid passwords", part2(&v));
}
