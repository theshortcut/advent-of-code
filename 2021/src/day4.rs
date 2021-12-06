use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

type BingoBoard = (BTreeMap<u8, usize>, u32);

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
  let (draws_str, boards_str) = input.split_once("\n\n").unwrap();
  let draws = draws_str.split(',').map(|n| n.parse().unwrap()).collect();
  let boards = boards_str
    .split("\n\n")
    .map(|b| {
      (
        b.trim()
          .split_whitespace()
          .enumerate()
          .map(|(i, n)| (n.parse().unwrap(), i))
          .collect(),
        0,
      )
    })
    .collect();
  (draws, boards)
}

#[aoc(day4, part1)]
fn part1((draws, boards): &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
  let mut boards = boards.clone();
  let (board, mark, num) = draws
    .iter()
    .find_map(|n| {
      boards.iter_mut().find_map(|(b, m)| {
        b.get(&n)
          .map(|i| *m |= 1 << *i)
          .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
          .map(|_| (b.clone(), *m, n))
      })
    })
    .unwrap();
  board
    .into_iter()
    .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * *num as u32)
    .sum::<u32>()
}

#[aoc(day4, part2)]
fn part2((draws, boards): &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
  let mut boards = boards.clone();
  let (board, mark, num) = draws
    .iter()
    .filter_map(|n| {
      boards
        .drain_filter(|(b, m)| {
          b.get(&n)
            .map(|i| *m |= 1 << *i)
            .map(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
            .unwrap_or(false)
        })
        .map(|(b, m)| (b, m, n))
        .next()
    })
    .last()
    .unwrap();
  board
    .into_iter()
    .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * *num as u32)
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

  #[test]
  fn part1_example() {
    assert_eq!(part1(&parse_input_day4(INPUT)), 4512)
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(&parse_input_day4(INPUT)), 1924)
  }
}
