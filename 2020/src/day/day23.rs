/// Cup game.
///
/// Round steps:
/// - pick up the three cups after the current cup
/// - destination cup is the current cup - 1.
///   - if that cup is missing continue to subtract one.
///   - if below starting value wrap to highest value.
/// - place the cups after the destination cup
/// - select the next cup as the current cup
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day23::*;
/// let input = "389125467".to_string();
/// let results = part1(&input, 10);
/// assert_eq!(results, "67384529");
/// ```
pub fn part1(i: &String, num_moves: usize) -> String {
  let mut cups = parse(i);
  for _ in 0..num_moves {
    play_round(&mut cups);
  }
  let (start_idx, _) = cups
    .iter()
    .cloned()
    .enumerate()
    .find(|(_, val)| val == &1)
    .unwrap();
  cups
    .iter()
    .cycle()
    .skip(start_idx + 1)
    .take(cups.len() - 1)
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join("")
}

fn parse(i: &String) -> Vec<usize> {
  i.chars()
    .filter_map(|c| c.to_string().parse().ok())
    .collect()
}

fn play_round(cups: &mut Vec<usize>) {
  let current: usize = cups[0];
  let cup_count = cups.len();
  let mut removed: Vec<usize> = vec![];
  for _ in 0..3 {
    removed.push(cups.remove(1));
  }
  let mut destination_val = current - 1;
  while !cups.contains(&destination_val) {
    if destination_val < 1 {
      destination_val = *cups.iter().max().unwrap();
    } else {
      destination_val -= 1;
    }
  }
  let (destination_idx, _) = cups
    .iter()
    .cloned()
    .enumerate()
    .find(|(_, v)| v == &destination_val)
    .unwrap();
  for i in 0..3 {
    cups.insert((destination_idx + i + 1) % cup_count, removed[i])
  }
  cups.rotate_left(1);
}

/// Now there are a million cups counting up after your input.
/// Find the product of the two cups after 1 after 10,000,000 moves
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day23::*;
/// let input = "389125467".to_string();
/// let results = part2(&input, 10_000_000);
/// assert_eq!(results, 149245887792);
/// ```
pub fn part2(i: &String, num_moves: usize) -> usize {
  let mut cups = parse(i);
  cups.extend(cups.iter().copied().max().unwrap() + 1..=1_000_000);
  let mut current_cup = cups[0];
  let mut cups = to_next_prev_pairs(&cups);
  for _ in 0..num_moves {
    play_round_linked(&mut cups, &mut current_cup);
  }
  cups[1].1 * cups[cups[1].1].1
}

fn to_next_prev_pairs(cups: &[usize]) -> Vec<(usize, usize)> {
  let mut cups_next_prev_pairs = vec![(0, 0); cups.len() + 1];
  for window in cups.windows(3) {
    cups_next_prev_pairs[window[1] as usize] = (window[0], window[2]);
  }
  cups_next_prev_pairs[cups[0] as usize] = (cups[cups.len() - 1], cups[1]);
  cups_next_prev_pairs[cups[cups.len() - 1] as usize] = (cups[cups.len() - 2], cups[0]);
  cups_next_prev_pairs
}

fn play_round_linked(cups: &mut Vec<(usize, usize)>, current_cup: &mut usize) {
  let mut destination = if *current_cup == 1 {
    cups.len() - 1
  } else {
    *current_cup - 1
  };
  let removed1 = cups[*current_cup].1;
  let removed2 = cups[removed1].1;
  let removed3 = cups[removed2].1;
  while vec![removed1, removed2, removed3].contains(&destination) {
    destination = if destination == 1 {
      cups.len() - 1
    } else {
      destination - 1
    };
  }
  let after_removed = cups[destination].1;
  cups[*current_cup].1 = cups[removed3].1;
  cups[destination].1 = removed1;
  cups[removed1] = (destination, removed2);
  cups[removed2] = (removed1, removed3);
  cups[removed3] = (removed2, after_removed);
  cups[after_removed].0 = removed3;
  *current_cup = cups[*current_cup].1;
}
