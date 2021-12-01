use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
  input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(i: &[i32]) -> usize {
  i.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[aoc(day1, part2)]
fn part2(i: &[i32]) -> usize {
  let sums: Vec<i32> = i.windows(3).map(|triple| triple.iter().sum()).collect();
  sums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[cfg(test)]
mod tests {
  use super::*;
  const TEST_INPUT: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn part1_example() {
    assert_eq!(part1(TEST_INPUT), 7)
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(TEST_INPUT), 5)
  }
}
