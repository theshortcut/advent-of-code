use std::collections::HashMap;

/// Memory Game.
/// Provided a list of starting numbers, those are read in turn then play continues following these rules:
/// - If that was the first time the number has been spoken, the current player says 0.
/// - Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
///
/// Find the 2020th number spoken.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day15::*;
///
/// let input: Vec<usize> = vec![0,3,6];
/// let result = part1(&input);
/// assert_eq!(result, 436);
/// ```
pub fn part1(starting_numbers: &Vec<usize>) -> usize {
  find_nth(&starting_numbers, 2020)
}

/// Find the 30000000th number spoken.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day15::*;
///
/// let input: Vec<usize> = vec![0,3,6];
/// let result = part2(&input);
/// assert_eq!(result, 175594);
/// ```
pub fn part2(starting_numbers: &Vec<usize>) -> usize {
  find_nth(&starting_numbers, 30_000_000)
}

fn find_nth(starting_numbers: &Vec<usize>, nth: usize) -> usize {
  let mut seen_numbers: HashMap<usize, usize> = HashMap::new();

  starting_numbers
    .iter()
    .cloned()
    .enumerate()
    .for_each(|(idx, v)| {
      seen_numbers.insert(v, idx + 1);
    });

  let mut last_pushed = *starting_numbers.last().unwrap();

  for i in starting_numbers.len()..nth {
    if !seen_numbers.contains_key(&last_pushed) {
      seen_numbers.insert(last_pushed, i);
      last_pushed = 0;
    } else {
      let when_seen = seen_numbers.get(&last_pushed).unwrap().clone();
      seen_numbers.insert(last_pushed, i);
      last_pushed = i - when_seen;
    }
  }
  last_pushed
}
