use std::collections::HashSet;

/// Each line is a command. `acc` increments the global accumulator,
/// `jmp` moves relative to the current command, and `nop` is no operation.
/// what value is in the accumulator right before it enters the infinite loop
/// or executes a command that has already run once?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day08::*;
///
/// let input = "nop +0
///acc +1
///jmp +4
///acc +3
///jmp -3
///acc -99
///acc +1
///jmp -4
///acc +6".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 5);
/// ```
pub fn part1(i: &String) -> i32 {
  let commands = parse(i);
  let mut acc = 0;
  let mut index = 0;
  let mut call_stack: HashSet<i32> = HashSet::new();
  run_until_loop(&mut index, &mut acc, &mut call_stack, &commands);
  acc
}

#[derive(Debug, Clone)]
enum Command {
  Noop(i32),
  Jump(i32),
  Add(i32),
}

fn parse(i: &String) -> Vec<Command> {
  i.split("\n")
    .map(|line| {
      let parts1: Vec<&str> = line.split(" ").collect();
      let raw_command = parts1[0];
      let arg: i32 = (match parts1[1].strip_prefix("+") {
        Some(raw_arg) => raw_arg,
        None => parts1[1],
      })
      .parse()
      .ok()
      .unwrap();
      (match raw_command {
        "nop" => Some(Command::Noop(arg)),
        "jmp" => Some(Command::Jump(arg)),
        "acc" => Some(Command::Add(arg)),
        _ => None,
      })
      .expect("unmatched command")
    })
    .collect()
}

fn process_command(index: i32, acc: i32, commands: &Vec<Command>) -> (i32, i32) {
  println!("executing command {:?}", commands.get(index as usize));
  (match commands.get(index as usize) {
    Some(Command::Noop(_)) => Some((index + 1, acc)),
    Some(Command::Jump(delta)) => Some((index + delta, acc)),
    Some(Command::Add(delta)) => Some((index + 1, acc + delta)),
    _ => None,
  })
  .expect("unable to process command")
}

fn run_until_loop(
  index: &mut i32,
  acc: &mut i32,
  call_stack: &mut HashSet<i32>,
  commands: &Vec<Command>,
) {
  call_stack.insert(*index);
  let (new_index, new_acc) = process_command(*index, *acc, commands);
  *index = new_index;
  *acc = new_acc;
  if !call_stack.contains(&new_index) {
    run_until_loop(index, acc, call_stack, commands);
  }
}

/// Change one jmp to nop or nop to jmp and get the program to complete.
/// what value is in the accumulator if the program completes?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day08::*;
///
/// let input = "nop +0
///acc +1
///jmp +4
///acc +3
///jmp -3
///acc -99
///acc +1
///jmp -4
///acc +6".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 8);
/// ```
pub fn part2(i: &String) -> i32 {
  let commands = parse(i);
  commands
    .iter()
    .enumerate()
    .filter_map(|(idx, cmd)| match cmd {
      Command::Jump(_) => Some(idx),
      Command::Noop(_) => Some(idx),
      _ => None,
    })
    .find_map(|index| attempt_swap(index, &commands))
    .expect("swapped them all and nothing completed")
}

fn attempt_swap(swap_index: usize, commands: &Vec<Command>) -> Option<i32> {
  let new_command = match commands.get(swap_index) {
    Some(Command::Jump(arg)) => Command::Noop(*arg),
    Some(Command::Noop(arg)) => Command::Jump(*arg),
    _ => Command::Noop(0),
  };
  let mut modified_commands = commands.clone();
  modified_commands.splice(
    swap_index..swap_index + 1,
    vec![new_command].iter().cloned(),
  );
  let mut acc = 0;
  let mut index = 0;
  let mut call_stack: HashSet<i32> = HashSet::new();
  run_until_complete(&mut index, &mut acc, &mut call_stack, &modified_commands)
}

fn run_until_complete(
  index: &mut i32,
  acc: &mut i32,
  call_stack: &mut HashSet<i32>,
  commands: &Vec<Command>,
) -> Option<i32> {
  call_stack.insert(*index);
  let (new_index, new_acc) = process_command(*index, *acc, commands);
  *index = new_index;
  *acc = new_acc;
  if new_index == commands.len() as i32 {
    return Some(new_acc);
  }
  if call_stack.contains(&new_index) {
    return None;
  }
  run_until_complete(index, acc, call_stack, commands)
}
