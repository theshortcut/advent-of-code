use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
  input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(i: &[i32]) -> i32 {
  i.iter()
    .copied()
    .find_map(
      |j| match i.iter().copied().find(|k| (j + k) as i32 == 2020) {
        Some(k) => Some((j * k) as i32),
        None => None,
      },
    )
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579)
  }
}
