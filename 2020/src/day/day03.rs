/// Count the number of trees denoted by # that would be encountered
/// moving from top left to the bottom by 3 x and y 1 each step
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day03::*;
///
/// let input = "..##.......
///#...#...#..
///.#....#..#.
///..#.#...#.#
///.#...##..#.
///..#.##.....
///.#.#.#....#
///.#........#
///#.##...#...
///#...##....#
///.#..#...#.#".to_string();
/// println!("{}", input);
/// let result = part1(&input);
/// assert_eq!(result, 7);
/// ```
pub fn part1(i: &String) -> i32 {
  let map_grid = parse(i);
  count_trees(&map_grid, 3, 1)
}

#[derive(Debug, PartialEq)]
pub enum MapGrid {
  Open,
  Tree,
}

fn parse(i: &String) -> Vec<Vec<MapGrid>> {
  i.split("\n")
    .map(|s| {
      s.chars()
        .filter_map(|c| match c {
          '.' => Some(MapGrid::Open),
          '#' => Some(MapGrid::Tree),
          _ => None,
        })
        .collect()
    })
    .collect()
}

fn count_trees(map_grid: &Vec<Vec<MapGrid>>, step_x: usize, step_y: usize) -> i32 {
  map_grid
    .iter()
    .step_by(step_y)
    .enumerate()
    .fold(0, |tree_count, (y, row)| {
      row
        .iter()
        .cycle()
        .nth(y * step_x)
        .map_or(tree_count, |m| match m {
          MapGrid::Tree => tree_count + 1,
          MapGrid::Open => tree_count,
        })
    })
}

/// Calculate the product of trees moving from the following slopes:
///  Right 1, down 1.
/// Right 3, down 1. (This is the slope you already checked.)
/// Right 5, down 1.
/// Right 7, down 1.
/// Right 1, down 2.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day03::*;
///
/// let input = "..##.......
///#...#...#..
///.#....#..#.
///..#.#...#.#
///.#...##..#.
///..#.##.....
///.#.#.#....#
///.#........#
///#.##...#...
///#...##....#
///.#..#...#.#".to_string();
/// println!("{}", input);
/// let result = part2(&input);
/// assert_eq!(result, 336);
/// ```
pub fn part2(i: &String) -> i32 {
  let map_grid = parse(i);
  [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    .iter()
    .map(|(step_x, step_y)| count_trees(&map_grid, *step_x, *step_y))
    .product()
}
