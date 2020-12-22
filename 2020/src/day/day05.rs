/// Given the input find the highest seat id
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day05::*;
///
/// let input = "BFFFBBFRRR
///FFFBBBFRRR
///BBFFBBFRLL".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 820);
/// ```
pub fn part1(i: &String) -> i32 {
  parse(i)
    .iter()
    .fold(0, |max, seat| if max > seat.id() { max } else { seat.id() })
}

#[derive(Debug)]
struct Seat {
  row: i32,
  column: i32,
}

impl Seat {
  pub fn id(&self) -> i32 {
    self.row * 8 + self.column
  }
}

#[derive(Debug)]
struct Range {
  min: i32,
  max: i32,
}

#[derive(Debug)]
struct PendingSeat {
  row: Range,
  column: Range,
}

fn parse(i: &String) -> Vec<Seat> {
  i.split("\n")
    .map(|line| {
      line.chars().fold(
        PendingSeat {
          row: Range { min: 0, max: 127 },
          column: Range { min: 0, max: 7 },
        },
        |pending_seat, char| match char {
          'F' => PendingSeat {
            row: Range {
              min: pending_seat.row.min,
              max: (pending_seat.row.max
                - ((pending_seat.row.max - pending_seat.row.min) as f32 / 2.0).floor() as i32),
            },
            ..pending_seat
          },
          'B' => PendingSeat {
            row: Range {
              min: (pending_seat.row.min
                + ((pending_seat.row.max - pending_seat.row.min) as f32 / 2.0).ceil() as i32),
              max: pending_seat.row.max,
            },
            ..pending_seat
          },
          'L' => PendingSeat {
            column: Range {
              min: pending_seat.column.min,
              max: (pending_seat.column.max
                - ((pending_seat.column.max - pending_seat.column.min) as f32 / 2.0).floor()
                  as i32),
            },
            ..pending_seat
          },
          'R' => PendingSeat {
            column: Range {
              min: (pending_seat.column.min
                + ((pending_seat.column.max - pending_seat.column.min) as f32 / 2.0).ceil() as i32),
              max: pending_seat.column.max,
            },
            ..pending_seat
          },
          _ => pending_seat,
        },
      )
    })
    .map(|ps| Seat {
      row: ps.row.min,
      column: ps.column.min,
    })
    .collect()
}

/// Find your seat id, can skip front and back rows
pub fn part2(i: &String) -> i32 {
  let mut seat_ids: Vec<i32> = parse(i).iter().map(|s| s.id()).collect();
  seat_ids.sort();
  seat_ids
    .iter()
    .enumerate()
    .find_map(|(idx, seat_id)| match seat_ids[idx + 1] {
      next_id if seat_id + 2 == next_id => Some(seat_id + 1),
      _ => None,
    })
    .unwrap_or_default()
}
