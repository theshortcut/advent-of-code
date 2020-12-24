use std::collections::HashSet;

/// Hex grid with directions starting from the center of the grid.
/// End tile is flipped from white to black.
/// How many tiles are black at the end of all instructions?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day24::*;
/// let input = "sesenwnenenewseeswwswswwnenewsewsw
///neeenesenwnwwswnenewnwwsewnenwseswesw
///seswneswswsenwwnwse
///nwnwneseeswswnenewneswwnewseswneseene
///swweswneswnenwsewnwneneseenw
///eesenwseswswnenwswnwnwsewwnwsene
///sewnenenenesenwsewnenwwwse
///wenwwweseeeweswwwnwwe
///wsweesenenewnwwnwsenewsenwwsesesenwne
///neeswseenwwswnwswswnw
///nenwswwsewswnenenewsenwsenwnesesenew
///enewnwewneswsewnwswenweswnenwsenwsw
///sweneswneswneneenwnewenewwneswswnese
///swwesenesewenwneswnwwneseswwne
///enesenwswwswneneswsenwnewswseenwsese
///wnwnesenesenenwwnenwsewesewsesesew
///nenewswnwewswnenesenwnesewesw
///eneswnwswnwsenenwnwnwwseeswneewsenese
///neswnwewnwnwseenwseesewsenwsweewe
///wseweeenwnesenwwwswnew".to_string();
/// let results = part1(&input);
/// assert_eq!(results, 10);
/// ```
pub fn part1(i: &String) -> usize {
  apply_directions(i).len()
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
  East,
  Southeast,
  Southwest,
  West,
  Northwest,
  Northeast,
}

impl Direction {
  const ALL: [Direction; 6] = [
    Direction::East,
    Direction::Southeast,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
    Direction::Northeast,
  ];
}

peg::parser! {
  grammar directions_parser() for str {
    rule direction() -> Direction
      = "e" { Direction::East }
      / "se" { Direction::Southeast }
      / "sw" { Direction::Southwest }
      / "w" { Direction::West }
      / "nw" { Direction::Northwest }
      / "ne" { Direction::Northeast }

    pub rule directions_set() -> Vec<Direction>
      = direction()*

    pub rule all_directions() -> Vec<Vec<Direction>>
      = directions_set() ** "\n"
  }
}

fn directions_to_coordinate(directions: &Vec<Direction>) -> (i64, i64) {
  directions.iter().fold((0, 0), |(x, y), dir| match dir {
    Direction::East => (x + 1, y),
    Direction::Southeast => (x, y + 1),
    Direction::Southwest => (x - 1, y + 1),
    Direction::West => (x - 1, y),
    Direction::Northwest => (x, y - 1),
    Direction::Northeast => (x + 1, y - 1),
  })
}

fn apply_directions(i: &String) -> HashSet<(i64, i64)> {
  let parsed = directions_parser::all_directions(i).unwrap();
  let coordinates = parsed
    .iter()
    .map(|d| directions_to_coordinate(d))
    .collect::<Vec<_>>();
  let mut black_tiles: HashSet<(i64, i64)> = HashSet::new();
  coordinates.iter().cloned().for_each(|coord| {
    if black_tiles.contains(&coord) {
      black_tiles.remove(&coord);
    } else {
      black_tiles.insert(coord);
    }
  });
  black_tiles
}

/// Now tiles flip each day according to these rules:
/// - Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
/// - Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day24::*;
/// let input = "sesenwnenenewseeswwswswwnenewsewsw
///neeenesenwnwwswnenewnwwsewnenwseswesw
///seswneswswsenwwnwse
///nwnwneseeswswnenewneswwnewseswneseene
///swweswneswnenwsewnwneneseenw
///eesenwseswswnenwswnwnwsewwnwsene
///sewnenenenesenwsewnenwwwse
///wenwwweseeeweswwwnwwe
///wsweesenenewnwwnwsenewsenwwsesesenwne
///neeswseenwwswnwswswnw
///nenwswwsewswnenenewsenwsenwnesesenew
///enewnwewneswsewnwswenweswnenwsenwsw
///sweneswneswneneenwnewenewwneswswnese
///swwesenesewenwneswnwwneseswwne
///enesenwswwswneneswsenwnewswseenwsese
///wnwnesenesenenwwnenwsewesewsesesew
///nenewswnwewswnenesenwnesewesw
///eneswnwswnwsenenwnwnwwseeswneewsenese
///neswnwewnwnwseenwseesewsenwsweewe
///wseweeenwnesenwwwswnew".to_string();
/// let results = part2(&input);
/// assert_eq!(results, 2208);
/// ```
pub fn part2(i: &String) -> usize {
  let mut black_tiles = apply_directions(i);
  for _ in 0..100 {
    apply_day_rules(&mut black_tiles);
  }
  black_tiles.len()
}

fn apply_day_rules(black_tiles: &mut HashSet<(i64, i64)>) {
  let b = black_tiles.clone();
  let min_x = b.iter().map(|(x, _)| x).min().unwrap();
  let max_x = b.iter().map(|(x, _)| x).max().unwrap();
  let min_y = b.iter().map(|(_, y)| y).min().unwrap();
  let max_y = b.iter().map(|(_, y)| y).max().unwrap();
  let mut coords_to_flip: HashSet<(i64, i64)> = HashSet::new();
  for x in (min_x - 1)..(max_x + 2) {
    for y in (min_y - 1)..(max_y + 2) {
      match (
        b.get(&(x, y)),
        get_neighbor_count(x, y, black_tiles.clone()),
      ) {
        (Some(_), n) if n == 0 => {
          coords_to_flip.insert((x, y));
        }
        (Some(_), n) if n > 2 => {
          coords_to_flip.insert((x, y));
        }
        (None, n) if n == 2 => {
          coords_to_flip.insert((x, y));
        }
        _ => (),
      }
    }
  }
  coords_to_flip.iter().for_each(|c| {
    if black_tiles.contains(c) {
      black_tiles.remove(c);
    } else {
      black_tiles.insert(*c);
    }
  })
}

fn get_neighbor_count(x: i64, y: i64, black_tiles: HashSet<(i64, i64)>) -> usize {
  Direction::ALL
    .iter()
    .filter_map(|dir| match dir {
      Direction::East => black_tiles.get(&(x + 1, y)),
      Direction::Southeast => black_tiles.get(&(x, y + 1)),
      Direction::Southwest => black_tiles.get(&(x - 1, y + 1)),
      Direction::West => black_tiles.get(&(x - 1, y)),
      Direction::Northwest => black_tiles.get(&(x, y - 1)),
      Direction::Northeast => black_tiles.get(&(x + 1, y - 1)),
    })
    .count()
}
