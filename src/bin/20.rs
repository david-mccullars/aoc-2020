extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

const DRAGON: &str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #
";

const TILE_WIDTH: isize = 10;

pub fn part_one(input: &str) -> Option<usize> {
    Some(TiledMap::new(input).corners().product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let tiled_map = TiledMap::new(input);
    let width = tiled_map.tiles_per_side * (TILE_WIDTH - 2);
    let image = Image {
        pixels: tiled_map.assembled_image(),
        bounds: (width, width),
    };

    let mut dragon = parser!(grid_of(" #")).parse(&DRAGON[1..]).unwrap();
    let mut dragon_image = Image {
        pixels: dragon.take_all('#'),
        bounds: dragon.bounds,
    };
    let dragon_pixels = dragon_image.pixel_count();

    image
        .search_any_orientation(dragon_image)
        .map(|found| image.pixel_count() - found * dragon_pixels)
}

#[derive(Default)]
struct TiledMap {
    tiles: HashMap<usize, Tile>,
    tiles_per_side: isize,
    arrangement: HashMap<Pos, usize>,
}

impl TiledMap {
    fn new(input: &str) -> Self {
        let mut tm = TiledMap::default();
        tm.tiles = parser!(hash_map(sections(
            line("Tile " usize ":")
            grid_of(".#")
        )))
        .parse(input)
        .expect("Failed to parse")
        .into_iter()
        .map(|(id, mut grid)| (id, Tile::from_pixels(id, grid.take_all('#'))))
        .collect();

        tm.tiles_per_side = (tm.tiles.len() as f32).sqrt() as isize;
        tm.find_neighbors();
        tm.arrange();
        tm
    }

    fn tile(&self, id: usize) -> &Tile {
        self.tiles.get(&id).unwrap()
    }

    fn tile_at(&self, pos: &Pos) -> &Tile {
        self.tile(*self.arrangement.get(pos).unwrap())
    }

    fn tile_mut(&mut self, id: usize) -> &mut Tile {
        self.tiles.get_mut(&id).unwrap()
    }

    fn corners(&self) -> impl Iterator<Item = usize> {
        self.tiles
            .values()
            .filter_map(|tile| tile.is_corner().then_some(tile.id))
    }

    fn find_neighbors(&mut self) {
        let combos: Vec<Vec<usize>> = self.tiles.keys().copied().combinations(2).collect();
        for ids in combos {
            if self.tile(ids[0]).shares_edge(self.tile(ids[1])) {
                self.tile_mut(ids[0]).neighbors.insert(ids[1]);
                self.tile_mut(ids[1]).neighbors.insert(ids[0]);
            }
        }
    }

    fn arrange(&mut self) {
        let random_corner = self.corners().next().unwrap();
        self.arrangement.insert((0, 0), random_corner);

        for i in 0..(2 * self.tiles_per_side) {
            for x in 0..=i {
                let y = i - x;
                if x + 1 < self.tiles_per_side && y < self.tiles_per_side {
                    let pos = (x, y);
                    self.arrange_from(Direction::East, &pos);
                    if x == 0 && y + 1 < self.tiles_per_side {
                        self.arrange_from(Direction::South, &pos);
                    }
                }
            }
        }
    }

    fn arrange_from(&mut self, dir: Direction, pos: &Pos) {
        let mut unarranged = self.unarranged_neighbors(pos);
        if pos.1 > 0 && dir == Direction::East {
            let ne_pos = Direction::North + &(Direction::East + &pos);
            let ne_unarranged = self.unarranged_neighbors(&ne_pos);
            unarranged = unarranged.intersection(&ne_unarranged).copied().collect();
        }
        let unarranged = unarranged.into_iter().next();
        self.arrangement.insert(dir + &pos, unarranged.unwrap());
    }

    fn unarranged_neighbors(&self, pos: &Pos) -> HashSet<usize> {
        let arranged = DIRECTIONS
            .into_iter()
            .filter_map(|dir| self.arrangement.get(&(dir + &pos)))
            .copied()
            .collect();
        self.tile_at(pos)
            .neighbors
            .difference(&arranged)
            .copied()
            .collect()
    }

    fn border_orientation(&self, p1: &Pos, dir: Direction) -> Direction {
        let t1 = self.tile_at(p1);
        let t2 = self.tile_at(&(dir + p1));
        t1.shared_border_orientation(t2)
    }

    fn assembled_image(&self) -> HashSet<Pos> {
        let w = TILE_WIDTH - 2;
        let mut all: Box<dyn Iterator<Item = Pos>> = Box::new(std::iter::empty());
        for pos in pos_in_square_grid(self.tiles_per_side) {
            let alignment_orientation = match pos {
                (0, 0) => (Direction::East, Direction::South),
                (0, _) => (Direction::East, Direction::North),
                (_, 0) => (Direction::West, Direction::South),
                _ => (Direction::West, Direction::North),
            };
            let mut pixels: Box<dyn Iterator<Item = Pos>> =
                Box::new(self.tile_at(&pos).pixels.iter().copied());

            let mut actual_orientation = (
                self.border_orientation(&pos, alignment_orientation.0),
                self.border_orientation(&pos, alignment_orientation.1),
            );

            while actual_orientation.0 != alignment_orientation.0 {
                pixels = Box::new(rotate(pixels, TILE_WIDTH - 1));
                actual_orientation = (
                    actual_orientation.0.turn_right(),
                    actual_orientation.1.turn_right(),
                );
            }
            if actual_orientation.1 != alignment_orientation.1 {
                pixels = Box::new(flip(pixels, TILE_WIDTH - 1));
            }

            all = Box::new(all.chain(shift(remove_border(pixels), (w * pos.0, w * pos.1))));
        }
        all.into_iter().collect()
    }
}

#[derive(Default)]
struct Tile {
    id: usize,
    pixels: HashSet<Pos>,
    border: Vec<u16>,
    edges: HashSet<u16>,
    neighbors: HashSet<usize>,
}

impl Tile {
    fn from_pixels(id: usize, pixels: HashSet<Pos>) -> Self {
        let mut tile = Tile::default();
        tile.id = id;
        tile.pixels = pixels;
        tile.border = tile.find_border();
        tile.edges = tile
            .border
            .iter()
            .map(|a| (a.reverse_bits() >> 6).min(*a))
            .collect();
        tile
    }

    fn find_border(&self) -> Vec<u16> {
        DIRECTIONS
            .into_iter()
            .map(|dir| {
                (0..10).fold(0, |n, i| {
                    let pos = match dir {
                        Direction::North => (i, 0),
                        Direction::South => (i, TILE_WIDTH - 1),
                        Direction::West => (0, i),
                        Direction::East => (TILE_WIDTH - 1, i),
                    };
                    (n << 1) + if self.pixels.contains(&pos) { 1 } else { 0 }
                })
            })
            .collect()
    }

    fn shares_edge(&self, other: &Tile) -> bool {
        self.edges.intersection(&other.edges).next().is_some()
    }

    fn is_corner(&self) -> bool {
        self.neighbors.len() == 2
    }

    fn shared_border_orientation(&self, other: &Tile) -> Direction {
        let e1 = *self.edges.intersection(&other.edges).next().unwrap();
        let e2 = e1.reverse_bits() >> 6;
        let idx = self.border.iter().position(|&e| e == e1 || e == e2);
        DIRECTIONS[idx.unwrap()]
    }
}

struct Image {
    pixels: HashSet<Pos>,
    bounds: Pos,
}

impl Image {
    fn pixel_count(&self) -> usize {
        self.pixels.len()
    }

    fn search_any_orientation(&self, mut img: Image) -> Option<usize> {
        for _ in 0..2 {
            for _ in 0..4 {
                let found = self._search(&img);
                if found > 0 {
                    return Some(found);
                }
                img.rotate();
            }
            img.flip();
        }
        None
    }

    fn _search(&self, img: &Image) -> usize {
        pos_in_grid(self.bounds.0 - img.bounds.0, self.bounds.1 - img.bounds.1)
            .filter(|offset| {
                img.pixels
                    .iter()
                    .all(|pos| self.pixels.contains(&pos_add(pos, offset)))
            })
            .count()
    }

    fn rotate(&mut self) {
        self.pixels = rotate(self.pixels.iter().copied(), self.bounds.1).collect();
        self.bounds = (self.bounds.1, self.bounds.0);
    }

    fn flip(&mut self) {
        self.pixels = flip(self.pixels.iter().copied(), self.bounds.1).collect();
    }
}

fn flip(data: impl Iterator<Item = Pos>, max_y: isize) -> impl Iterator<Item = Pos> {
    data.map(move |(x, y)| (x, max_y - y))
}

fn rotate(data: impl Iterator<Item = Pos>, max_y: isize) -> impl Iterator<Item = Pos> {
    data.map(move |(x, y)| (max_y - y, x))
}

fn remove_border(data: impl Iterator<Item = Pos>) -> impl Iterator<Item = Pos> {
    let rng = 1..(TILE_WIDTH - 1);
    data.filter_map(move |(x, y)| (rng.contains(&x) && rng.contains(&y)).then_some((x - 1, y - 1)))
}

fn shift(data: impl Iterator<Item = Pos>, offset: Pos) -> impl Iterator<Item = Pos> {
    data.map(move |p| pos_add(&p, &offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20899048083289));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(273));
    }
}
