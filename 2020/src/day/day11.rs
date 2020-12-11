/// Rules:
/// - If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
/// - If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
/// - Otherwise, the seat's state does not change.
///
/// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
/// How many seats end up occupied?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day11::*;
///
/// let input = "L.LL.LL.LL
///LLLLLLL.LL
///L.L.L..L..
///LLLL.LL.LL
///L.LL.LL.LL
///L.LLLLL.LL
///..L.L.....
///LLLLLLLLLL
///L.LLLLLL.L
///L.LLLLL.LL".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 37);
/// ```
pub fn part1(i: &String) -> usize {
  let mut grid = Grid::new(i);
  let mut is_settled = false;
  while !is_settled {
    let next_grid = process_round(&grid);
    if grid == next_grid {
      is_settled = true;
    } else {
      grid = next_grid;
    }
  }
  grid
    .spaces
    .iter()
    .flat_map(|row| {
      row
        .iter()
        .cloned()
        .filter(|space| *space == Space::OccupiedSeat)
    })
    .count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
  EmptySeat,
  Floor,
  OccupiedSeat,
}

#[derive(Debug, PartialEq)]
struct Grid {
  spaces: Vec<Vec<Space>>,
}

impl Grid {
  fn new(i: &String) -> Self {
    let spaces: Vec<Vec<Space>> = i
      .split("\n")
      .map(|line| {
        line
          .chars()
          .filter_map(|c| match c {
            'L' => Some(Space::EmptySeat),
            '#' => Some(Space::OccupiedSeat),
            '.' => Some(Space::Floor),
            _ => None,
          })
          .collect()
      })
      .collect();
    Self { spaces }
  }

  fn get_adjacent_occupied_count(&self, x: usize, y: usize) -> usize {
    let is_top = y == 0;
    let is_left = x == 0;
    let is_right = x == self.spaces[0].len() - 1;
    let is_bottom = y == self.spaces.len() - 1;
    let mut adjacent_spaces: Vec<Space> = Vec::new();
    let start_x = if is_left { x } else { x - 1 };
    let end_x = if is_right { x + 1 } else { x + 2 };
    if !is_top {
      adjacent_spaces.append(&mut self.spaces[y - 1][start_x..end_x].to_vec());
    }
    if !is_left {
      adjacent_spaces.push(self.spaces[y][x - 1]);
    }
    if !is_right {
      adjacent_spaces.push(self.spaces[y][x + 1]);
    }
    if !is_bottom {
      adjacent_spaces.append(&mut self.spaces[y + 1][start_x..end_x].to_vec());
    }
    adjacent_spaces
      .iter()
      .cloned()
      .filter(|space| *space == Space::OccupiedSeat)
      .count()
  }

  fn get_visible_occupied_count(&self, x: usize, y: usize) -> usize {
    let directions: Vec<(isize, isize)> = vec![
      (-1, -1),
      (-1, 0),
      (-1, 1),
      (0, -1),
      (0, 1),
      (1, -1),
      (1, 0),
      (1, 1),
    ];
    directions
      .iter()
      .cloned()
      .flat_map(|(dy, dx)| self.get_first_visible_seat(x, y, dx, dy))
      .filter(|space| *space == Space::OccupiedSeat)
      .count()
  }

  fn get_first_visible_seat(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<Space> {
    let mut cx = x as isize + dx;
    let mut cy = y as isize + dy;
    let mut space: Option<Space> = None;
    while cx >= 0
      && cy >= 0
      && cx < self.spaces[0].len() as isize
      && cy < self.spaces.len() as isize
      && space.is_none()
    {
      space = match self.spaces[cy as usize][cx as usize] {
        Space::OccupiedSeat => Some(Space::OccupiedSeat),
        Space::EmptySeat => Some(Space::EmptySeat),
        _ => None,
      };
      cx = cx + dx;
      cy = cy + dy;
    }
    space
  }
}

fn process_round(grid: &Grid) -> Grid {
  let spaces: Vec<Vec<Space>> = grid
    .spaces
    .iter()
    .enumerate()
    .map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .map(|(x, space)| match space {
          Space::EmptySeat if grid.get_adjacent_occupied_count(x, y) == 0 => Space::OccupiedSeat,
          Space::OccupiedSeat if grid.get_adjacent_occupied_count(x, y) > 3 => Space::EmptySeat,
          _ => *space,
        })
        .collect()
    })
    .collect();
  Grid { spaces }
}

/// People now care about the first seat they see in each of the eight directions.
/// Additionally they are more tolerant and allow up to 5 adjacent occupied seats.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day11::*;
///
/// let input = "L.LL.LL.LL
///LLLLLLL.LL
///L.L.L..L..
///LLLL.LL.LL
///L.LL.LL.LL
///L.LLLLL.LL
///..L.L.....
///LLLLLLLLLL
///L.LLLLLL.L
///L.LLLLL.LL".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 26);
/// ```
pub fn part2(i: &String) -> usize {
  let mut grid = Grid::new(i);
  let mut is_settled = false;
  while !is_settled {
    let next_grid = process_round2(&grid);
    if grid == next_grid {
      is_settled = true;
    } else {
      grid = next_grid;
    }
  }
  grid
    .spaces
    .iter()
    .flat_map(|row| {
      row
        .iter()
        .cloned()
        .filter(|space| *space == Space::OccupiedSeat)
    })
    .count()
}

fn process_round2(grid: &Grid) -> Grid {
  let spaces: Vec<Vec<Space>> = grid
    .spaces
    .iter()
    .enumerate()
    .map(|(y, row)| {
      row
        .iter()
        .enumerate()
        .map(|(x, space)| match space {
          Space::EmptySeat if grid.get_visible_occupied_count(x, y) == 0 => Space::OccupiedSeat,
          Space::OccupiedSeat if grid.get_visible_occupied_count(x, y) > 4 => Space::EmptySeat,
          _ => *space,
        })
        .collect()
    })
    .collect();
  Grid { spaces }
}
