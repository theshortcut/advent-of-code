use std::collections::HashMap;

/// 3 dimensional infinite array of "cubes" which cycle and during the cycle the cubes follow these rules:
/// - If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
/// - If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
///
/// Input is a flat plane where '#' is active and '.' is inactive.
///
/// How many cubes are active after 6 cycles?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day17::*;
///
/// let input = ".#.
///..#
///###".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 117);
/// ```
pub fn part1(i: &String) -> usize {
  let pocket_state = parse_3d(i);
  (0..6)
    .fold(pocket_state, |state, _| process_cycle_3d(&state))
    .iter()
    .filter(|(_, active)| *active == &true)
    .count()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Vector3 {
  x: i64,
  y: i64,
  z: i64,
}

impl Vector3 {
  fn new(x: i64, y: i64, z: i64) -> Self {
    Self { x, y, z }
  }

  fn neighbor_coords(&self) -> Vec<Self> {
    (-1..2)
      .flat_map(|delta_x| {
        (-1..2)
          .flat_map(|delta_y| {
            (-1..2)
              .filter_map(|delta_z| match (delta_x, delta_y, delta_z) {
                (0, 0, 0) => None,
                (dx, dy, dz) => Some(Self::new(self.x + dx, self.y + dy, self.z + dz)),
              })
              .collect::<Vec<_>>()
          })
          .collect::<Vec<_>>()
      })
      .collect()
  }
}

type PocketState3D = HashMap<Vector3, bool>;

fn parse_3d(i: &String) -> PocketState3D {
  let mut pocket = HashMap::new();
  i.split("\n").enumerate().for_each(|(y, row)| {
    row.chars().enumerate().for_each(|(x, char)| {
      let active = match char {
        '#' => true,
        '.' => false,
        err => panic!("Unexpected input {}", err),
      };
      pocket.insert(Vector3::new(x as i64, y as i64, 0), active);
    })
  });
  pocket
}

fn process_cycle_3d(pocket_state: &PocketState3D) -> PocketState3D {
  let mut new_state = pocket_state.clone();
  let (min, max) = pocket_state.iter().fold(
    (Vector3::default(), Vector3::default()),
    |(min, max), (coord, _)| {
      let mut new_min = min;
      let mut new_max = max;
      if coord.x < min.x {
        new_min.x = coord.x;
      }
      if coord.x > max.x {
        new_max.x = coord.x;
      }
      if coord.y < min.y {
        new_min.y = coord.y;
      }
      if coord.y > max.y {
        new_max.y = coord.y;
      }
      if coord.z < min.z {
        new_min.z = coord.z;
      }
      if coord.z > max.z {
        new_max.z = coord.z;
      }
      (new_min, new_max)
    },
  );
  (min.x - 1..max.x + 2).for_each(|x| {
    (min.y - 1..max.y + 2).for_each(|y| {
      (min.z - 1..max.z + 2).for_each(|z| {
        let coord = Vector3::new(x, y, z);
        let coord_state = pocket_state.get(&coord).unwrap_or(&false);
        let active_neighbor_count = coord
          .neighbor_coords()
          .iter()
          .filter_map(|coord| match pocket_state.get(&coord) {
            Some(active) if active == &true => Some(true),
            _ => None,
          })
          .count();
        match (coord_state, active_neighbor_count) {
          (true, 2) => (),
          (true, 3) => (),
          (false, 3) => {
            new_state.insert(coord, true);
          }
          _ => {
            new_state.insert(coord, false);
          }
        }
      });
    });
  });
  new_state
}

/// 4d now but all other rules remain the same
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day17::*;
///
/// let input = ".#.
///..#
///###".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 848);
/// ```
pub fn part2(i: &String) -> usize {
  let pocket_state = parse_4d(i);
  (0..6)
    .fold(pocket_state, |state, _| process_cycle_4d(&state))
    .iter()
    .filter(|(_, active)| *active == &true)
    .count()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Vector4 {
  x: i64,
  y: i64,
  z: i64,
  w: i64,
}

impl Vector4 {
  fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
    Self { x, y, z, w }
  }

  fn neighbor_coords(&self) -> Vec<Self> {
    (-1..2)
      .flat_map(|delta_x| {
        (-1..2)
          .flat_map(|delta_y| {
            (-1..2)
              .flat_map(|delta_z| {
                (-1..2)
                  .filter_map(|delta_w| match (delta_x, delta_y, delta_z, delta_w) {
                    (0, 0, 0, 0) => None,
                    (dx, dy, dz, dw) => Some(Self::new(
                      self.x + dx,
                      self.y + dy,
                      self.z + dz,
                      self.w + dw,
                    )),
                  })
                  .collect::<Vec<_>>()
              })
              .collect::<Vec<_>>()
          })
          .collect::<Vec<_>>()
      })
      .collect()
  }
}

type PocketState4D = HashMap<Vector4, bool>;

fn parse_4d(i: &String) -> PocketState4D {
  let mut pocket = HashMap::new();
  i.split("\n").enumerate().for_each(|(y, row)| {
    row.chars().enumerate().for_each(|(x, char)| {
      let active = match char {
        '#' => true,
        '.' => false,
        err => panic!("Unexpected input {}", err),
      };
      pocket.insert(Vector4::new(x as i64, y as i64, 0, 0), active);
    })
  });
  pocket
}

fn process_cycle_4d(pocket_state: &PocketState4D) -> PocketState4D {
  let mut new_state = pocket_state.clone();
  let (min, max) = pocket_state.iter().fold(
    (Vector4::default(), Vector4::default()),
    |(min, max), (coord, _)| {
      let mut new_min = min;
      let mut new_max = max;
      if coord.x < min.x {
        new_min.x = coord.x;
      }
      if coord.x > max.x {
        new_max.x = coord.x;
      }
      if coord.y < min.y {
        new_min.y = coord.y;
      }
      if coord.y > max.y {
        new_max.y = coord.y;
      }
      if coord.z < min.z {
        new_min.z = coord.z;
      }
      if coord.z > max.z {
        new_max.z = coord.z;
      }
      if coord.w < min.w {
        new_min.w = coord.w;
      }
      if coord.w > max.w {
        new_max.w = coord.w;
      }
      (new_min, new_max)
    },
  );
  (min.x - 1..max.x + 2).for_each(|x| {
    (min.y - 1..max.y + 2).for_each(|y| {
      (min.z - 1..max.z + 2).for_each(|z| {
        (min.w - 1..max.w + 2).for_each(|w| {
          let coord = Vector4::new(x, y, z, w);
          let coord_state = pocket_state.get(&coord).unwrap_or(&false);
          let active_neighbor_count = coord
            .neighbor_coords()
            .iter()
            .filter_map(|coord| match pocket_state.get(&coord) {
              Some(active) if active == &true => Some(true),
              _ => None,
            })
            .count();
          match (coord_state, active_neighbor_count) {
            (true, 2) => (),
            (true, 3) => (),
            (false, 3) => {
              new_state.insert(coord, true);
            }
            _ => {
              new_state.insert(coord, false);
            }
          }
        });
      });
    });
  });
  new_state
}
