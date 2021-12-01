use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
  input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(i: &[i32]) -> usize {
  i.iter()
    .enumerate()
    .filter(|(idx, depth)| match i.get(idx + 1) {
      Some(next_depth) => *depth < next_depth,
      None => false,
    })
    .count()
}

#[aoc(day1, part2)]
fn part2(i: &[i32]) -> usize {
  // create 3 element sums
  let sums: Vec<i32> = i
    .iter()
    .enumerate()
    .filter_map(|(idx, depth_a)| match (i.get(idx + 1), i.get(idx + 2)) {
      (Some(depth_b), Some(depth_c)) => Some(depth_a + depth_b + depth_c),
      _ => None,
    })
    .collect();
  sums
    .iter()
    .enumerate()
    .filter(|(idx, depth)| match sums.get(idx + 1) {
      Some(next_depth) => *depth < next_depth,
      None => false,
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(
      part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
      7
    )
  }

  #[test]
  fn part2_example() {
    assert_eq!(
      part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
      5
    )
  }
}
