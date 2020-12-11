use std::collections::HashMap;

/// Find the # of 1 diff * # of 3 diff
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day10::*;
///
/// let input = "16
///10
///15
///5
///1
///11
///7
///19
///6
///12
///4".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 35);
/// ```
pub fn part1(i: &String) -> i32 {
  let mut adapter_joltages: Vec<i32> = i.split("\n").filter_map(|s| s.parse().ok()).collect();
  adapter_joltages.sort();
  adapter_joltages.insert(0, 0);
  let (count_1, count_3) =
    adapter_joltages
      .iter()
      .enumerate()
      .fold((0, 0), |(count_1, count_3), (idx, joltage)| {
        if idx == 0 {
          return (count_1, count_3);
        }
        match adapter_joltages.get(idx - 1) {
          Some(previous_joltage) if joltage - previous_joltage == 1 => (count_1 + 1, count_3),
          Some(previous_joltage) if joltage - previous_joltage == 3 => (count_1, count_3 + 1),
          _ => (count_1, count_3),
        }
      });
  count_1 * (count_3 + 1)
}

/// Your device requires a joltage +3 the highest adapter.
/// Adapters can chain from 1 to 3 higher joltages.
/// Find all the distinct ways you can chain your adapters
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day10::*;
///
/// let input = "16
///10
///15
///5
///1
///11
///7
///19
///6
///12
///4".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 8);
/// ```
pub fn part2(i: &String) -> usize {
  let mut adapter_joltages: Vec<isize> = i.split("\n").filter_map(|s| s.parse().ok()).collect();
  adapter_joltages.sort();
  let mut jumps_per_adapter = HashMap::new();
  jumps_per_adapter.insert(0, 1);
  for &joltage in &adapter_joltages {
    let mut jumps = 0;
    for previous in 0.max(joltage - 3)..joltage {
      jumps = jumps + jumps_per_adapter.get(&previous).unwrap_or(&0);
    }
    jumps_per_adapter.insert(joltage, jumps);
  }
  jumps_per_adapter[adapter_joltages.last().unwrap()]
}
