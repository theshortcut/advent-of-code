/// What is the ID of the earliest bus you can take to the airport
/// multiplied by the number of minutes you'll need to wait for that bus?
///
/// Bus IDS indicate how long their trip from the seaport to the airport takes.
/// All buses depart at timestamp 0.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day13::*;
///
/// let input = "939
///7,13,x,x,59,x,31,19".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 295);
/// ```
pub fn part1(i: &String) -> i32 {
  let (arrival_timestamp, bus_ids) = parse(&i);
  let (departure_time, bus_id) = find_earliest_departing_bus_id(&arrival_timestamp, &bus_ids);
  (departure_time - arrival_timestamp) * bus_id
}

fn parse(i: &String) -> (i32, Vec<i32>) {
  let mut parts = i.split("\n");
  let arrival_timestamp: i32 = parts.next().unwrap().parse().ok().unwrap();
  let bus_ids: Vec<i32> = parts
    .next()
    .unwrap()
    .split(",")
    .filter_map(|s| s.parse().ok())
    .collect();
  (arrival_timestamp, bus_ids)
}

fn find_earliest_departing_bus_id(&start: &i32, bus_ids: &Vec<i32>) -> (i32, i32) {
  let mut bus_id: Option<i32> = None;
  let mut time = start;
  while bus_id.is_none() {
    bus_id = bus_ids.iter().cloned().find(|id| time % id == 0);
    if bus_id.is_none() {
      time = time + 1;
    }
  }
  (time, bus_id.unwrap())
}

/// What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day13::*;
///
/// let input = "939
///7,13,x,x,59,x,31,19".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 1068781);
/// ```
pub fn part2(i: &String) -> i64 {
  let enumerated_bus_ids: Vec<(i64, i64)> = i
    .split("\n")
    .nth(1)
    .unwrap()
    .split(",")
    .enumerate()
    .filter_map(|(idx, s)| match s.parse::<i64>() {
      Ok(i) => Some((idx as i64, i)),
      _ => None,
    })
    .collect();
  let mods = enumerated_bus_ids
    .iter()
    .map(|&(_, b)| b)
    .collect::<Vec<_>>();
  let res = enumerated_bus_ids
    .iter()
    .map(|&(i, b)| (b - i))
    .collect::<Vec<_>>();
  chinese_remainder(&res, &mods).unwrap()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
  if a == 0 {
    (b, 0, 1)
  } else {
    let (g, x, y) = egcd(b % a, a);
    (g, y - (b / a) * x, x)
  }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
  let (g, x, _) = egcd(x, n);
  if g == 1 {
    Some((x % n + n) % n)
  } else {
    None
  }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
  let prod = modulii.iter().product::<i64>();
  let mut sum = 0;
  for (&residue, &modulus) in residues.iter().zip(modulii) {
    let p = prod / modulus;
    sum += residue * mod_inv(p, modulus)? * p
  }
  Some(sum % prod)
}
