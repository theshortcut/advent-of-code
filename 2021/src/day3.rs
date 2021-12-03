use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct BinaryList(Vec<u32>);

impl BinaryList {
  fn new() -> Self {
    BinaryList(vec![])
  }

  fn digit_at(&self, position: usize) -> u32 {
    self.0[position]
  }

  fn push(&mut self, i: u32) {
    self.0.push(i)
  }
}

impl Into<u32> for BinaryList {
  fn into(self) -> u32 {
    u32::from_str_radix(
      self
        .0
        .iter()
        .map(|digit| digit.to_string())
        .collect::<Vec<String>>()
        .join("")
        .as_str(),
      2,
    )
    .unwrap()
  }
}

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<BinaryList> {
  input
    .lines()
    .map(|l| {
      BinaryList(
        l.trim()
          .chars()
          .map(|ch| ch.to_digit(10).unwrap())
          .collect(),
      )
    })
    .collect()
}

fn most_common_bit_at(lists: &Vec<BinaryList>, position: usize) -> u32 {
  let (c0, c1) = lists
    .iter()
    .map(|b| b.digit_at(position))
    .fold((0, 0), |(c0, c1), digit| match digit {
      0 => (c0 + 1, c1),
      _ => (c0, c1 + 1),
    });
  if c0 > c1 {
    0
  } else {
    1
  }
}

#[aoc(day3, part1)]
fn part1(i: &Vec<BinaryList>) -> u32 {
  let (gamma, epsilon) = (0..i[0].0.len()).fold(
    (BinaryList::new(), BinaryList::new()),
    |(mut gamma, mut epsilon), idx| {
      let most_common = most_common_bit_at(i, idx);
      gamma.push(most_common);
      epsilon.push(if most_common == 1 { 0 } else { 1 });
      (gamma, epsilon)
    },
  );
  let gamma_as_int: u32 = gamma.into();
  let epsilon_as_int: u32 = epsilon.into();
  gamma_as_int * epsilon_as_int
}

fn filter_by_commononality(
  lists: &Vec<BinaryList>,
  position: usize,
  most_common: bool,
) -> BinaryList {
  let most_common_digit = most_common_bit_at(lists, position);
  let digit_to_match = if most_common {
    most_common_digit
  } else {
    if most_common_digit == 0 {
      1
    } else {
      0
    }
  };
  let filtered: Vec<BinaryList> = lists
    .iter()
    .filter(|b| b.digit_at(position) == digit_to_match)
    .cloned()
    .collect();
  if filtered.len() == 1 {
    filtered[0].clone()
  } else {
    filter_by_commononality(&filtered, position + 1, most_common)
  }
}

#[aoc(day3, part2)]
fn part2(i: &Vec<BinaryList>) -> u32 {
  let o2 = filter_by_commononality(i, 0, true);
  let co2 = filter_by_commononality(i, 0, false);
  let o2_as_int: u32 = o2.into();
  let co2_as_int: u32 = co2.into();
  o2_as_int * co2_as_int
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "00100
  11110
  10110
  10111
  10101
  01111
  00111
  11100
  10000
  11001
  00010
  01010";

  #[test]
  fn part1_example() {
    assert_eq!(part1(&parse_input_day3(INPUT)), 198)
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(&parse_input_day3(INPUT)), 230)
  }
}
