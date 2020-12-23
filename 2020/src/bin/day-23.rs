use advent_of_code_2020::day::day23::*;

fn main() {
  let contents = "137826495".to_string();
  println!("Part 1: {}", part1(&contents, 100));
  println!("Part 2: {}", part2(&contents, 10_000_000));
}
