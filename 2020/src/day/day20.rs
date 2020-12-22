use itertools::{izip, Itertools};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

/// Provided an array of images. Flip, rotate and arrange the images
/// so that all adjacent border line up (they should match exactly).
/// Outermost edges won't align with any other tiles.
///
/// Multiply the ids of the four corner tiles together.
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day20::*;
///
/// let input = r"Tile 2311:
///..**.*..*.
///**..*.....
///*...**..*.
///****.*...*
///**.**.***.
///**...*.***
///.*.*.*..**
///..*....*..
///***...*.*.
///..***..***
///
///Tile 1951:
///*.**...**.
///*.****...*
///.....*..**
///*...******
///.**.*....*
///.***.*****
///***.**.**.
///.***....*.
///..*.*..*.*
///*...**.*..
///
///Tile 1171:
///****...**.
///*..**.*..*
///**.*..*.*.
///.***.****.
///..***.****
///.**....**.
///.*...****.
///*.**.****.
///****..*...
///.....**...
///
///Tile 1427:
///***.**.*..
///.*..*.**..
///.*.**.*..*
///*.*.*.**.*
///....*...**
///...**..**.
///...*.*****
///.*.****.*.
///..*..***.*
///..**.*..*.
///
///Tile 1489:
///**.*.*....
///..**...*..
///.**..**...
///..*...*...
///*****...*.
///*..*.*.*.*
///...*.*.*..
///**.*...**.
///..**.**.**
///***.**.*..
///
///Tile 2473:
///*....****.
///*..*.**...
///*.**..*...
///******.*.*
///.*...*.*.*
///.*********
///.***.*..*.
///********.*
///**...**.*.
///..***.*.*.
///
///Tile 2971:
///..*.*....*
///*...***...
///*.*.***...
///**.**..*..
///.*****..**
///.*..****.*
///*..*.*..*.
///..****.***
///..*.*.***.
///...*.*.*.*
///
///Tile 2729:
///...*.*.*.*
///****.*....
///..*.*.....
///....*..*.*
///.**..**.*.
///.*.****...
///****.*.*..
///**.****...
///**..*.**..
///*.**...**.
///
///Tile 3079:
///*.*.*****.
///.*..******
///..*.......
///******....
///****.*..*.
///.*...*.**.
///*.*****.**
///..*.***...
///..*.......
///..*.***...".to_string();
/// let result = part1(&input);
/// assert_eq!(result, 20899048083289);
/// ```
pub fn part1(i: &String) -> i64 {
  let images: HashMap<i64, Image<10>> = parse(i);
  let edges = find_corners(&images);
  edges.keys().product()
}

type Pixels<const N: usize> = [[bool; N]; N];

#[derive(Clone, Copy, Debug)]
struct Image<const N: usize> {
  pixels: Pixels<N>,
}

impl<const N: usize> TryFrom<Vec<Vec<bool>>> for Image<N> {
  type Error = &'static str; // whatever

  fn try_from(value: Vec<Vec<bool>>) -> Result<Self, Self::Error> {
    let pixels = value
      .into_iter()
      .map(|row| row.try_into().map_err(|_| "bad width"))
      .collect::<Result<Vec<[bool; N]>, &str>>()?
      .try_into()
      .map_err(|_| "bad height")?;

    Ok(Self { pixels })
  }
}

impl<const N: usize> Image<N> {
  fn flip_vertical(&self) -> Self {
    let mut pixels = [[false; N]; N];
    for y in 0..N {
      pixels[y] = self.pixels[(N - 1) - y];
    }
    Self { pixels }
  }

  fn rotate(&self) -> Self {
    let mut pixels = [[false; N]; N];
    for y in 0..N {
      for x in 0..N {
        pixels[y][x] = self.pixels[(N - 1) - x][y];
      }
    }
    Self { pixels }
  }

  fn orientations(&self) -> Vec<Self> {
    let mut current = self.clone();
    let mut all = Vec::new();
    for _ in 0..4 {
      all.push(current);
      current = Self {
        ..current.clone().rotate()
      };
    }
    current = current.clone().flip_vertical();
    for _ in 0..4 {
      all.push(current);
      current = current.clone().rotate();
    }
    all.iter().cloned().map(|i| i.clone()).collect()
  }

  fn row(&self, row: usize) -> [bool; N] {
    self.pixels[row]
  }

  fn column(&self, column: usize) -> [bool; N] {
    let mut pixels = [false; N];
    for y in 0..N {
      pixels[y] = self.pixels[y][column]
    }
    pixels
  }

  fn edges(&self) -> [[bool; N]; 4] {
    [
      self.row(0),
      self.row(N - 1),
      self.column(0),
      self.column(N - 1),
    ]
  }

  fn fits_above(&self, other: &Self) -> bool {
    self.pixels[N - 1] == other.pixels[0]
  }

  fn fits_below(&self, other: &Self) -> bool {
    other.fits_above(self)
  }

  fn fits_left(&self, other: &Self) -> bool {
    (0..N).all(|y| self.pixels[y][N - 1] == other.pixels[y][0])
  }

  fn fits_right(&self, other: &Self) -> bool {
    other.fits_left(self)
  }
}

fn parse<const N: usize>(i: &String) -> HashMap<i64, Image<N>> {
  i.trim()
    .split("\n\n")
    .map(|tile| {
      let parts: Vec<&str> = tile.split("\n").collect();
      let id: i64 = parts[0]
        .split(" ")
        .last()
        .unwrap()
        .replace(":", "")
        .parse()
        .unwrap();
      let mut pixels = [[false; N]; N];
      parts.iter().skip(1).enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
          '.' => pixels[y][x] = false,
          _ => pixels[y][x] = true,
        });
      });
      (id, Image { pixels })
    })
    .collect()
}

fn find_corners<const N: usize>(images: &HashMap<i64, Image<N>>) -> HashMap<i64, Image<N>> {
  images
    .iter()
    .filter(|(id, image)| {
      image
        .edges()
        .iter()
        .filter(|edge| {
          let reverse: [bool; N] = edge
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();
          images
            .iter()
            .filter(|(other_id, _)| other_id != id)
            .all(|(_, other_image)| {
              let other_edges = other_image.edges();
              !(other_edges.contains(edge) || other_edges.contains(&reverse))
            })
        })
        .count()
        == 2
    })
    .map(|(id, image)| (*id, *image))
    .collect()
}

/// Now that the images are assembled you can find sea monsters.
/// They look like this (only the non empty spaces matter):
///                   #
/// #    ##    ##    ###
///  #  #  #  #  #  #   
///
/// The borders of each tile also needs to be removed, and you may need
/// to rotate or flip the assembled image to find the sea monsters.
///
/// How many "#" spaces exist that are NOT part of a sea monster?
///
/// # Example
///
/// ```
/// use advent_of_code_2020::day::day20::*;
///
/// let input = r"Tile 2311:
///..**.*..*.
///**..*.....
///*...**..*.
///****.*...*
///**.**.***.
///**...*.***
///.*.*.*..**
///..*....*..
///***...*.*.
///..***..***
///
///Tile 1951:
///*.**...**.
///*.****...*
///.....*..**
///*...******
///.**.*....*
///.***.*****
///***.**.**.
///.***....*.
///..*.*..*.*
///*...**.*..
///
///Tile 1171:
///****...**.
///*..**.*..*
///**.*..*.*.
///.***.****.
///..***.****
///.**....**.
///.*...****.
///*.**.****.
///****..*...
///.....**...
///
///Tile 1427:
///***.**.*..
///.*..*.**..
///.*.**.*..*
///*.*.*.**.*
///....*...**
///...**..**.
///...*.*****
///.*.****.*.
///..*..***.*
///..**.*..*.
///
///Tile 1489:
///**.*.*....
///..**...*..
///.**..**...
///..*...*...
///*****...*.
///*..*.*.*.*
///...*.*.*..
///**.*...**.
///..**.**.**
///***.**.*..
///
///Tile 2473:
///*....****.
///*..*.**...
///*.**..*...
///******.*.*
///.*...*.*.*
///.*********
///.***.*..*.
///********.*
///**...**.*.
///..***.*.*.
///
///Tile 2971:
///..*.*....*
///*...***...
///*.*.***...
///**.**..*..
///.*****..**
///.*..****.*
///*..*.*..*.
///..****.***
///..*.*.***.
///...*.*.*.*
///
///Tile 2729:
///...*.*.*.*
///****.*....
///..*.*.....
///....*..*.*
///.**..**.*.
///.*.****...
///****.*.*..
///**.****...
///**..*.**..
///*.**...**.
///
///Tile 3079:
///*.*.*****.
///.*..******
///..*.......
///******....
///****.*..*.
///.*...*.**.
///*.*****.**
///..*.***...
///..*.......
///..*.***...".to_string();
/// let result = part2(&input);
/// assert_eq!(result, 273);
/// ```
pub fn part2(i: &String) -> usize {
  let images: HashMap<i64, Image<10>> = parse(i);
  let (assembled, _) = assemble(&images);
  let (min_x, max_x) = assembled
    .keys()
    .map(|(x, _)| *x)
    .minmax()
    .into_option()
    .unwrap();
  let (min_y, max_y) = assembled
    .keys()
    .map(|(_, y)| *y)
    .minmax()
    .into_option()
    .unwrap();

  let mut stitched = Vec::new();

  for tile_y in min_y..=max_y {
    let mut rows = vec![Vec::new(); 8];
    for tile_x in min_x..=max_x {
      let tile = &assembled[&(tile_x, tile_y)];
      for y in 1..9 {
        rows[y - 1].extend(tile.pixels[y][1..9].iter().copied());
      }
    }
    stitched.extend(rows);
  }

  match stitched.len() {
    24 => remove_monsters::<24>(stitched.try_into().unwrap()),
    96 => remove_monsters::<96>(stitched.try_into().unwrap()),
    _ => panic!("unsupported dimensions"),
  }
}

fn assemble(
  input: &HashMap<i64, Image<10>>,
) -> (HashMap<(i64, i64), Image<10>>, HashMap<(i64, i64), i64>) {
  let mut images = input.clone();
  let mut img = HashMap::new();
  let mut ids = HashMap::new();

  let first_id = images.keys().copied().next().unwrap();
  ids.insert((0, 0), first_id);

  let first_tile = images.remove(&first_id).unwrap();
  img.insert((0, 0), first_tile);

  while images.len() > 0 {
    let mut failed = HashMap::new();

    'next_tile: for (id, image) in &images {
      for piece in image.orientations() {
        'next_pos: for (pos, o) in &img {
          let (x, y) = *pos;
          let (above, below, left, right) = ((x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y));

          let placement = if !img.contains_key(&above) && piece.fits_above(o) {
            above
          } else if !img.contains_key(&below) && piece.fits_below(o) {
            below
          } else if !img.contains_key(&left) && piece.fits_left(o) {
            left
          } else if !img.contains_key(&right) && piece.fits_right(o) {
            right
          } else {
            continue 'next_pos;
          };
          ids.insert(placement, *id);
          img.insert(placement, piece);
          continue 'next_tile;
        }
      }
      failed.insert(*id, *image);
    }

    images = failed;
  }

  (img, ids)
}

fn row_match(pixels: &[bool], monster: &[u8]) -> bool {
  pixels
    .iter()
    .zip(monster.iter())
    .all(|(v, c)| *c == b' ' || *v)
}

fn remove_monsters<const N: usize>(img: Image<N>) -> usize {
  let num_hash = img
    .pixels
    .iter()
    .flat_map(|row| row.iter())
    .filter(|x| **x)
    .count();

  let monster = [
    b"                  # ",
    b"#    ##    ##    ###",
    b" #  #  #  #  #  #   ",
  ];

  for img in img.orientations() {
    let mut monster_count = 0;
    for rows in img.pixels.windows(3) {
      let (t, m, b) = match rows {
        [t, m, b] => (t, m, b),
        _ => unreachable!(),
      };

      let w = monster[0].len();
      for (t, m, b) in izip!(t.windows(w), m.windows(w), b.windows(w)) {
        if izip!(&[t, m, b], &monster).all(|(p, m)| row_match(p, *m)) {
          monster_count += 1;
        }
      }
    }

    if monster_count != 0 {
      return num_hash - 15 * monster_count;
    }
  }

  println!("no monsters found");
  num_hash
}
