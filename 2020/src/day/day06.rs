use std::collections::HashSet;

/// Each line is questions a-z that were answered YES per individual.
/// Goups are separated by a blank line.
///
/// Count the total yes per group, then sum those totals.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day06::*;
///
/// let input = "abc
///
///a
///b
///c
///
///ab
///ac
///
///a
///a
///a
///a
///
///b".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 11);
/// ```
pub fn part1(i: &String) -> usize {
  let group_sets: Vec<HashSet<char>> = i
    .split("\n\n")
    .into_iter()
    .map(|s| {
      let mut set = HashSet::new();
      s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
        set.insert(c);
      });
      set
    })
    .collect();
  group_sets.iter().map(|g| g.len()).sum()
}

/// Each line is questions a-z that were answered YES per individual.
/// Goups are separated by a blank line.
///
/// Count the total where everyone in the group answered yes.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day06::*;
///
/// let input = "abc
///
///a
///b
///c
///
///ab
///ac
///
///a
///a
///a
///a
///
///b".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 6);
/// ```
pub fn part2(i: &String) -> usize {
  let group_sets: Vec<usize> = i
    .split("\n\n")
    .map(|s| {
      let mut set = HashSet::new();
      s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
        set.insert(c);
      });
      let individuals: Vec<HashSet<char>> = s.split("\n").map(|s| s.chars().collect()).collect();
      set
        .iter()
        .filter(|c| individuals.iter().all(|chars| chars.contains(c)))
        .count()
    })
    .collect();
  group_sets.iter().sum()
}
