use crate::common::{Answer, Solution};

pub struct Day10;

impl Solution for Day10 {
    fn name(&self) -> &'static str {
        "Pipe Maze"
    }

    fn part_a(&self, input: &str) -> Answer {
        let grid = parse_grid(input);
        let mut tile = find_start(&grid);
        let mut direction = Direction::all()
            .iter()
            .find(|d| {
                let coords = tile.go(**d);

                get_tile(&grid, coords)
                    .and_then(|t| t.get_direction(tile))
                    .is_some()
            })
            .copied()
            .unwrap();

        let mut distance = 0;

        loop {
            distance += 1;

            let next_tile = get_tile(&grid, tile.go(direction)).unwrap();
            if next_tile.tile_type == TileType::Start {
                break;
            }

            let next_direction = next_tile.get_direction(tile).unwrap();

            tile = next_tile;
            direction = next_direction;
        }

        (distance / 2).into()
    }

    fn part_b(&self, _input: &str) -> Answer {
        Answer::Unimplemented
    }
}

type Grid = Vec<Vec<Tile>>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, l)| {
            l.chars()
                .enumerate()
                .map(|(col_idx, c)| Tile {
                    line_idx,
                    col_idx,
                    tile_type: c.into(),
                })
                .collect()
        })
        .collect()
}

fn find_start(grid: &Grid) -> &Tile {
    grid.iter()
        .filter_map(|l| l.iter().find(|t| t.tile_type == TileType::Start))
        .next()
        .unwrap()
}

fn get_tile(grid: &Grid, coords: (usize, usize)) -> Option<&Tile> {
    grid.get(coords.0).and_then(|line| line.get(coords.1))
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    line_idx: usize,
    col_idx: usize,
    tile_type: TileType,
}

impl Tile {
    fn go(&self, direction: Direction) -> (usize, usize) {
        match direction {
            Direction::North => (self.line_idx - 1, self.col_idx),
            Direction::South => (self.line_idx + 1, self.col_idx),
            Direction::East => (self.line_idx, self.col_idx + 1),
            Direction::West => (self.line_idx, self.col_idx - 1),
        }
    }

    fn get_direction(&self, from: &Tile) -> Option<Direction> {
        if from.line_idx < self.line_idx {
            // coming from north
            match self.tile_type {
                TileType::VerticalPipe => Some(Direction::South),
                TileType::NorthEastPipe => Some(Direction::East),
                TileType::NorthWestPipe => Some(Direction::West),
                _ => None, // unreachable!("{:?} is not north-facing", self.tile_type),
            }
        } else if from.line_idx > self.line_idx {
            // coming from south
            match self.tile_type {
                TileType::VerticalPipe => Some(Direction::North),
                TileType::SouthEastPipe => Some(Direction::East),
                TileType::SouthWestPipe => Some(Direction::West),
                _ => None, // unreachable!("{:?} is not south-facing", self.tile_type),
            }
        } else if from.col_idx < self.col_idx {
            // coming from west
            match self.tile_type {
                TileType::HorizontalPipe => Some(Direction::East),
                TileType::NorthWestPipe => Some(Direction::North),
                TileType::SouthWestPipe => Some(Direction::South),
                _ => None, // unreachable!("{:?} is not west-facing", self.tile_type),
            }
        } else if from.col_idx > self.col_idx {
            // coming from east
            match self.tile_type {
                TileType::HorizontalPipe => Some(Direction::West),
                TileType::NorthEastPipe => Some(Direction::North),
                TileType::SouthEastPipe => Some(Direction::South),
                _ => None, // unreachable!("{:?} is not east-facing", self.tile_type),
            }
        } else {
            unreachable!("no change in coordinates ({:?})", from);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    Start,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastPipe,
            'J' => Self::NorthWestPipe,
            '7' => Self::SouthWestPipe,
            'F' => Self::SouthEastPipe,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!("unknown tile: {value}"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![Self::North, Self::South, Self::East, Self::West]
    }
}

#[cfg(test)]
mod test {
    use super::Day10;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day10.part_a(INPUT), 8.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day10.part_b(INPUT), crate::common::Answer::Unimplemented);
    }
}
