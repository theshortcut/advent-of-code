/// Each line is a navigational command. NSEW mean move in that direction that many units.
/// F means move in the facing direction
/// L and R rotate left & right  by degrees.
/// Find the Manhattan distance from the starting point
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day12::*;
///
/// let input = "F10
///N3
///F7
///R90
///F11".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 25);
/// ```
pub fn part1(i: &String) -> i32 {
  let commands = parse1(i);
  let ship_state = process_commands1(&commands);
  println!("final coordinates: {:?}", ship_state.coordinates);
  ship_state.coordinates.x.abs() + ship_state.coordinates.y.abs()
}

#[derive(Clone, Copy, Debug)]
enum Command {
  North(i32),
  South(i32),
  East(i32),
  West(i32),
  RotateLeft(i32),
  RotateRight(i32),
  Forward(i32),
}

#[derive(Clone, Copy, Default, Debug)]
struct Vector2 {
  x: i32,
  y: i32,
}

impl Vector2 {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  fn to_degrees(&self) -> i32 {
    match (self.x, self.y) {
      (1, 0) => 0,
      (0, -1) => 90,
      (-1, 0) => 180,
      (0, 1) => 270,
      _ => panic!("unhandled to_degrees"),
    }
  }

  fn from_degrees(deg: i32) -> Self {
    let normalized = deg % 360;
    match normalized {
      0 => Self::new(1, 0),
      90 => Self::new(0, -1),
      180 => Self::new(-1, 0),
      270 => Self::new(0, 1),
      _ => panic!("unhandled to_degrees"),
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct ShipState1 {
  coordinates: Vector2,
  rotation: Vector2,
}

impl ShipState1 {
  fn new() -> Self {
    Self {
      coordinates: Vector2::default(),
      rotation: Vector2::new(1, 0),
    }
  }

  fn move_by(&self, dx: i32, dy: i32) -> Self {
    Self {
      coordinates: Vector2::new(self.coordinates.x + dx, self.coordinates.y + dy),
      ..*self
    }
  }

  fn rotate(&self, degrees: i32) -> Self {
    Self {
      rotation: Vector2::from_degrees(self.rotation.to_degrees() + degrees),
      ..*self
    }
  }
}

fn parse1(i: &String) -> Vec<Command> {
  i.split("\n")
    .filter_map(|line| {
      let first_letter = line.chars().next();
      let rest = line[1..].parse::<i32>().ok();
      match (first_letter, rest) {
        (Some('N'), Some(i)) => Some(Command::North(i)),
        (Some('S'), Some(i)) => Some(Command::South(i)),
        (Some('E'), Some(i)) => Some(Command::East(i)),
        (Some('W'), Some(i)) => Some(Command::West(i)),
        (Some('F'), Some(i)) => Some(Command::Forward(i)),
        (Some('R'), Some(i)) => Some(Command::RotateRight(i)),
        (Some('L'), Some(i)) => Some(Command::RotateLeft(i)),
        _ => panic!("unhandled command: {}", line),
      }
    })
    .collect()
}

fn process_commands1(commands: &Vec<Command>) -> ShipState1 {
  commands
    .iter()
    .fold(ShipState1::new(), |ship_state, command| match command {
      Command::North(i) => ship_state.move_by(0, *i),
      Command::South(i) => ship_state.move_by(0, -1 * i),
      Command::East(i) => ship_state.move_by(*i, 0),
      Command::West(i) => ship_state.move_by(-1 * i, 0),
      Command::RotateRight(deg) => ship_state.rotate(*deg),
      Command::RotateLeft(deg) => ship_state.rotate(360 - deg),
      Command::Forward(i) => {
        ship_state.move_by(i * ship_state.rotation.x, i * ship_state.rotation.y)
      }
    })
}

/// Turns out all the commands were for moving a waypoint relative to the ship.
/// The waypoint begins 10 east and 1 north of the ship
/// F moves to the waypoint x times, the other commands move or rotate the waypoint
/// Find the Manhattan distance from the starting point
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day12::*;
///
/// let input = "F10
///N3
///F7
///R90
///F11".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 286);
/// ```
pub fn part2(i: &String) -> i32 {
  let commands = parse(i);
  let ship_state = process_commands(&commands);
  println!("final coordinates: {:?}", ship_state.coordinates);
  ship_state.coordinates.x.abs() + ship_state.coordinates.y.abs()
}

#[derive(Clone, Copy, Debug)]
struct ShipState {
  waypoint: Vector2,
  coordinates: Vector2,
}

impl ShipState {
  fn new() -> Self {
    Self {
      waypoint: Vector2::new(10, 1),
      coordinates: Vector2::default(),
    }
  }

  fn move_waypoint(&self, dx: i32, dy: i32) -> Self {
    Self {
      waypoint: Vector2::new(self.waypoint.x + dx, self.waypoint.y + dy),
      ..*self
    }
  }

  fn rotate_waypoint(&self, degrees: i32) -> Self {
    match degrees % 360 {
      90 => Self {
        waypoint: Vector2::new(self.waypoint.y, -self.waypoint.x),
        ..*self
      },
      180 => Self {
        waypoint: Vector2::new(-self.waypoint.x, -self.waypoint.y),
        ..*self
      },
      270 => Self {
        waypoint: Vector2::new(-self.waypoint.y, self.waypoint.x),
        ..*self
      },
      _ => panic!("unhandled rotation {}", degrees),
    }
  }

  fn move_to_waypoint(&self, times: i32) -> Self {
    Self {
      coordinates: Vector2::new(
        self.coordinates.x + (times * self.waypoint.x),
        self.coordinates.y + (times * self.waypoint.y),
      ),
      ..*self
    }
  }
}

fn parse(i: &String) -> Vec<Command> {
  i.split("\n")
    .filter_map(|line| {
      let first_letter = line.chars().next();
      let rest = line[1..].parse::<i32>().ok();
      match (first_letter, rest) {
        (Some('N'), Some(i)) => Some(Command::North(i)),
        (Some('S'), Some(i)) => Some(Command::South(i)),
        (Some('E'), Some(i)) => Some(Command::East(i)),
        (Some('W'), Some(i)) => Some(Command::West(i)),
        (Some('F'), Some(i)) => Some(Command::Forward(i)),
        (Some('R'), Some(i)) => Some(Command::RotateRight(i)),
        (Some('L'), Some(i)) => Some(Command::RotateLeft(i)),
        _ => panic!("unhandled command: {}", line),
      }
    })
    .collect()
}

fn process_commands(commands: &Vec<Command>) -> ShipState {
  commands
    .iter()
    .fold(ShipState::new(), |ship_state, command| match command {
      Command::North(i) => ship_state.move_waypoint(0, *i),
      Command::South(i) => ship_state.move_waypoint(0, -1 * i),
      Command::East(i) => ship_state.move_waypoint(*i, 0),
      Command::West(i) => ship_state.move_waypoint(-1 * i, 0),
      Command::RotateRight(deg) => ship_state.rotate_waypoint(*deg),
      Command::RotateLeft(deg) => ship_state.rotate_waypoint(360 - deg),
      Command::Forward(times) => ship_state.move_to_waypoint(*times),
    })
}
