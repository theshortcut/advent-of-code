/// Find the two items that sum to 2020 and return their product
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day01::part1;
///
/// let input = vec![1721, 979, 366, 299, 675, 1456];
/// let result = part1(&input);
/// assert_eq!(result, 514579);
/// ```
pub fn part1(i: &Vec<i32>) -> i32 {
  i.iter()
    .copied()
    .find_map(|j| match i.iter().copied().find(|k| j + k == 2020) {
      Some(k) => Some(j * k),
      None => None,
    })
    .unwrap_or(0)
}

/// Find the three items that sum to 2020 and return their product
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day01::part2;
///
/// let input = vec![1721, 979, 366, 299, 675, 1456];
/// let result = part2(&input);
/// assert_eq!(result, 241861950);
/// ```
pub fn part2(i: &Vec<i32>) -> i32 {
  i.iter()
    .copied()
    .find_map(|j| {
      match i
        .iter()
        .copied()
        .find_map(|k| match i.iter().copied().find(|l| j + k + l == 2020) {
          Some(l) => Some((k, l)),
          None => None,
        }) {
        Some((k, l)) => Some(j * k * l),
        None => None,
      }
    })
    .unwrap_or(0)
}
