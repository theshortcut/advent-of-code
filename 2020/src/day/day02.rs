use std::convert::TryInto;

/// Find the valid passwords from the input.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day02::part1;
///
/// let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
/// let result = part1(&input);
/// assert_eq!(result, 2);
/// ```
pub fn part1(i: &Vec<&str>) -> usize {
  parse(i)
    .iter()
    .filter_map(|(rule, password)| {
      if is_valid_one((rule, password)) {
        Some(password)
      } else {
        None
      }
    })
    .count()
}

fn parse<'a>(i: &Vec<&'a str>) -> Vec<(&'a str, &'a str)> {
  i.iter()
    .map(|s| {
      let parts: Vec<&str> = s.split(":").collect();
      (parts[0], parts[1].trim())
    })
    .collect()
}

fn is_valid_one(i: (&str, &str)) -> bool {
  let (rule, password) = i;
  let parts: Vec<&str> = rule.split(" ").collect();
  let counts: Vec<i32> = parts[0].split("-").filter_map(|s| s.parse().ok()).collect();
  let min = counts[0];
  let max = counts[1];
  let char = parts[1].trim();
  let matches: i32 = password.matches(&char).count().try_into().unwrap();
  matches >= min && matches <= max
}

/// Find the valid passwords from the input.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day02::part2;
///
/// let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
/// let result = part2(&input);
/// assert_eq!(result, 1);
/// ```
pub fn part2(i: &Vec<&str>) -> usize {
  parse(i)
    .iter()
    .filter_map(|(rule, password)| {
      if is_valid_two((rule, password)) {
        Some(password)
      } else {
        None
      }
    })
    .count()
}

fn is_valid_two(i: (&str, &str)) -> bool {
  let (rule, password) = i;
  let parts: Vec<&str> = rule.split(" ").collect();
  let indices: Vec<usize> = parts[0].split("-").filter_map(|s| s.parse().ok()).collect();
  let char = parts[1].trim();
  match (
    password.get(indices[0] - 1..indices[0]),
    password.get(indices[1] - 1..indices[1]),
  ) {
    (Some(char1), Some(char2)) => char1 != char2 && (char1 == char || char2 == char),
    _ => false,
  }
}
