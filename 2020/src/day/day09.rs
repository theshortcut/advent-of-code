/// Each line after the preamble (25 lines in input data, 5 in the example below)
/// each line must be a sum of 2 of the preceeding preamble length numbers.
/// Find the first number that breaks that rule.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day09::*;
///
/// let input = "35
///20
///15
///25
///47
///40
///62
///55
///65
///95
///102
///117
///150
///182
///127
///219
///299
///277
///309
///576".to_string();
/// let result = part1(&input, 5);
/// assert_eq!(result, 127);
/// ```
pub fn part1(i: &String, preamble_length: usize) -> usize {
  let numbers = parse(i);
  find_first_exception(preamble_length, &numbers)
}

fn parse(i: &String) -> Vec<usize> {
  i.split("\n")
    .filter_map(|s| s.trim().parse().ok())
    .collect()
}

fn find_first_exception(preamble_length: usize, numbers: &Vec<usize>) -> usize {
  let enumerated: Vec<(usize, usize)> = numbers.iter().cloned().enumerate().collect();
  enumerated[preamble_length..]
    .iter()
    .cloned()
    .find(|(idx, num_to_check)| {
      enumerated[idx - preamble_length..*idx]
        .iter()
        .cloned()
        .all(|(_, preceeding_1)| {
          enumerated[idx - preamble_length..*idx]
            .iter()
            .cloned()
            .all(|(_, preceeding_2)| preceeding_1 + preceeding_2 != *num_to_check)
        })
    })
    .expect("No exceptions")
    .1
}

/// Find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.
/// Then add the smallest and largest numbers together to find the encryption weakness
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day09::*;
///
/// let input = "35
///20
///15
///25
///47
///40
///62
///55
///65
///95
///102
///117
///150
///182
///127
///219
///299
///277
///309
///576".to_string();
/// let result = part2(&input, 5);
/// assert_eq!(result, 62);
/// ```
pub fn part2(i: &String, preamble_length: usize) -> usize {
  let numbers = parse(i);
  let exception = find_first_exception(preamble_length, &numbers);
  find_contiguous_sum_set(exception, &numbers)
}

fn find_contiguous_sum_set(sum: usize, numbers: &Vec<usize>) -> usize {
  let enumerated: Vec<(usize, usize)> = numbers.iter().cloned().enumerate().collect();
  let (start_idx, end_idx) = enumerated
    .iter()
    .find_map(|(start_idx, _)| {
      enumerated[start_idx + 1..]
        .iter()
        .take_while(|(current_idx, _)| {
          numbers[*start_idx..*current_idx].iter().sum::<usize>() <= sum
        })
        .find(|(current_idx, _)| numbers[*start_idx..*current_idx].iter().sum::<usize>() == sum)
        .map(|(end_idx, _)| (start_idx, end_idx))
    })
    .expect("no contiguous set found");
  numbers[*start_idx..*end_idx].iter().min().unwrap()
    + numbers[*start_idx..*end_idx].iter().max().unwrap()
}
