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
    let images = parse(i);
    let edges = find_corners(&images);
    edges.iter().map(|image| image.id).product()
}

struct Neighbor {
    id: i64,
    is_reversed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Image {
    id: i64,
    pixels: Vec<Vec<bool>>,
}

impl Image {
    fn flip_horizontal(&self) -> Self {
        Self {
            pixels: self
                .pixels
                .iter()
                .map(|row| row.iter().cloned().rev().collect())
                .collect(),
            ..*self
        }
    }

    fn flip_vertical(&self) -> Self {
        Self {
            pixels: self.pixels.iter().cloned().rev().collect(),
            ..*self
        }
    }

    fn rotate_90(&self) -> Self {
        Self {
            pixels: (0..10)
                .map(|y| (0..10).map(|x| self.pixels[x][y]).collect())
                .collect(),
            ..*self
        }
    }

    fn row(&self, row: usize) -> Vec<bool> {
        self.pixels[row].clone()
    }

    fn column(&self, column: usize) -> Vec<bool> {
        (0..10).map(|y| self.pixels[y][column]).collect()
    }

    fn edges(&self) -> Vec<Vec<bool>> {
        vec![
            self.row(0),
            self.row(self.pixels.len() - 1),
            self.column(0),
            self.column(self.pixels[0].len() - 1),
        ]
    }
}

fn parse(i: &String) -> Vec<Image> {
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
            let pixels = parts
                .iter()
                .skip(1)
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            _ => true,
                        })
                        .collect()
                })
                .collect();
            Image { id, pixels }
        })
        .collect()
}

fn find_corners(images: &Vec<Image>) -> Vec<Image> {
    images
        .iter()
        .cloned()
        .filter(|image| {
            image
                .edges()
                .iter()
                .filter(|edge| {
                    let reverse: Vec<bool> = edge.iter().cloned().rev().collect();
                    images
                        .iter()
                        .filter(|Image { id, pixels: _ }| image.id != *id)
                        .all(|other_image| {
                            let other_edges = other_image.edges();
                            !(other_edges.contains(edge) || other_edges.contains(&reverse))
                        })
                })
                .count()
                == 2
        })
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
    0
}

// this is gonna suck
fn assemble(images: &Vec<Image>) {
    images.iter()
}
