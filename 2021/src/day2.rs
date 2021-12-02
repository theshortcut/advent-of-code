use aoc_runner_derive::{aoc, aoc_generator};

enum Direction {
  Forward(i32),
  Up(i32),
  Down(i32),
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Direction> {
  input
    .lines()
    .filter_map(|l| {
      let mut parts = l.trim().split(" ");
      match (parts.next(), parts.next()) {
        (Some("up"), Some(distance)) => Some(Direction::Up(distance.parse().unwrap())),
        (Some("down"), Some(distance)) => Some(Direction::Down(distance.parse().unwrap())),
        (Some("forward"), Some(distance)) => Some(Direction::Forward(distance.parse().unwrap())),
        _ => None,
      }
    })
    .collect()
}

#[aoc(day2, part1)]
fn part1(i: &[Direction]) -> i32 {
  let (x, y) = i.iter().fold((0, 0), |(x, y), dir| match dir {
    Direction::Up(dist) => (x, y - dist),
    Direction::Down(dist) => (x, y + dist),
    Direction::Forward(dist) => (x + dist, y),
  });
  x * y
}

#[aoc(day2, part2)]
fn part2(i: &[Direction]) -> i32 {
  let (x, y, _) = i.iter().fold((0, 0, 0), |(x, y, aim), dir| match dir {
    Direction::Up(dist) => (x, y, aim - dist),
    Direction::Down(dist) => (x, y, aim + dist),
    Direction::Forward(dist) => (x + dist, y + (aim * dist), aim),
  });
  x * y
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "forward 5
  down 5
  forward 8
  up 3
  down 8
  forward 2
  ";

  #[test]
  fn part1_example() {
    let input: &[Direction] = &parse_input_day2(INPUT);
    assert_eq!(part1(input), 150)
  }

  #[test]
  fn part2_example() {
    let input: &[Direction] = &parse_input_day2(INPUT);
    assert_eq!(part2(input), 900)
  }
}
