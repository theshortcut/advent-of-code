use regex::Regex;
use std::collections::HashMap;

/// Execute the initialization program. What is the sum of all values left in memory after it completes?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day14::*;
///
/// let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
///mem[8] = 11
///mem[7] = 101
///mem[8] = 0".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 165);
/// ```
pub fn part1(i: &String) -> i64 {
  let commands = parse(i);
  let results = run_program_1(&commands);
  results.values().sum()
}

#[derive(Clone, Debug, Default)]
struct BitMask(Vec<Option<bool>>);

impl BitMask {
  fn new(i: &String) -> Self {
    Self(
      i.chars()
        .map(|c| match c {
          '1' => Some(true),
          '0' => Some(false),
          _ => None,
        })
        .collect(),
    )
  }
}

#[derive(Clone, Debug)]
enum Command {
  SetBitMask(BitMask),
  SetMemory((i64, i64)),
}

fn parse(i: &String) -> Vec<Command> {
  let mem_cmd_re = Regex::new(r"mem\[(\d+)\]").unwrap();
  i.split("\n")
    .filter_map(|line| {
      let parts: Vec<String> = line.split(" = ").map(|s| s.to_string()).collect();
      match (&parts.get(0), &parts.get(1)) {
        (Some(cmd), Some(mask_str)) if *cmd == "mask" => {
          Some(Command::SetBitMask(BitMask::new(mask_str)))
        }
        (Some(mem_cmd), Some(val_str)) if mem_cmd.starts_with("mem") => {
          let addr: i64 = mem_cmd_re.captures(&mem_cmd).unwrap()[1].parse().unwrap();
          Some(Command::SetMemory((addr, val_str.parse().unwrap())))
        }
        _ => None,
      }
    })
    .collect()
}

fn run_program_1(commands: &Vec<Command>) -> HashMap<i64, i64> {
  let mut bit_mask: BitMask = Default::default();
  let mut memory: HashMap<i64, i64> = HashMap::new();
  commands.iter().for_each(|cmd| match cmd {
    Command::SetBitMask(bm) => bit_mask = bm.clone(),
    Command::SetMemory((addr, val)) => {
      let masked_value: String = format!("{:036b}", val)
        .chars()
        .enumerate()
        .map(|(idx, char)| match bit_mask.0[idx] {
          Some(true) => '1',
          Some(false) => '0',
          None => char,
        })
        .collect();
      memory.insert(*addr, i64::from_str_radix(&masked_value, 2).unwrap());
    }
  });
  memory
}

/// The mask is now used as a memory address decoder not a value bitmask
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day14::*;
///
/// let input = "mask = 000000000000000000000000000000X1001X
///mem[42] = 100
///mask = 00000000000000000000000000000000X0XX
///mem[26] = 1".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 208);
/// ```
pub fn part2(i: &String) -> i64 {
  let commands = parse(i);
  let results = run_program_2(&commands);
  results.values().sum()
}

fn run_program_2(commands: &Vec<Command>) -> HashMap<i64, i64> {
  let mut bit_mask: BitMask = Default::default();
  let mut memory: HashMap<i64, i64> = HashMap::new();
  commands.iter().for_each(|cmd| match cmd {
    Command::SetBitMask(bm) => bit_mask = bm.clone(),
    Command::SetMemory((addr, val)) => {
      let masked_addr: String = format!("{:036b}", addr)
        .chars()
        .enumerate()
        .map(|(idx, char)| match bit_mask.0[idx] {
          Some(true) => '1',
          Some(false) => char,
          None => 'X',
        })
        .collect();
      let mut addrs: Vec<String> = Vec::new();
      addrs.push(masked_addr);
      while addrs.iter().any(|s| s.contains('X')) {
        create_addresses_with_floats(&mut addrs);
      }
      addrs.iter().for_each(|addr| {
        memory.insert(i64::from_str_radix(&addr, 2).unwrap(), *val);
      })
    }
  });
  memory
}

fn create_addresses_with_floats(addrs: &mut Vec<String>) {
  let index = addrs.iter().position(|s| s.contains('X'));
  let to_replace = addrs.remove(index.unwrap());
  addrs.push(to_replace.replacen('X', "0", 1));
  addrs.push(to_replace.replacen('X', "1", 1));
}
