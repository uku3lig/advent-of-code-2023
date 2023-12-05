use std::{ops::Range, str::FromStr};

use crate::common::{Answer, Solution};

pub struct Day05;

impl Solution for Day05 {
    fn name(&self) -> &'static str {
        "If You Give A Seed A Fertilizer"
    }

    fn part_a(&self, input: &str) -> Answer {
        let almanac = input.parse::<Almanac>().unwrap();
        let min = almanac.seeds.iter().map(|s| almanac.map(*s)).min().unwrap();

        min.into()
    }

    // yes this takes 93 seconds to run.
    fn part_b(&self, input: &str) -> Answer {
        let almanac = input.parse::<Almanac>().unwrap();
        let mut min = u32::MAX;

        for range in almanac.seed_ranges() {
            for seed in range {
                let seed = almanac.map(seed);
                min = min.min(seed);
            }
        }

        min.into()
    }
}

// ===

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    fn map(&self, input: u32) -> u32 {
        let mut input = input;

        for map in &self.maps {
            input = map.map(input);
        }

        input
    }

    fn seed_ranges(&self) -> Vec<Range<u32>> {
        self.seeds
            .chunks_exact(2)
            .map(|s| s[0]..(s[0] + s[1]))
            .collect()
    }
}

impl FromStr for Almanac {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
        let seeds: Vec<u32> = seeds
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;

        let maps = lines.collect::<Vec<_>>().join("\n");
        let maps = maps
            .trim()
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .parse::<Map>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { seeds, maps })
    }
}

// ===

struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, num: u32) -> u32 {
        for range in &self.ranges {
            if range.range().contains(&num) {
                let diff = num - range.source_start;
                return range.dest_start + diff;
            }
        }

        num
    }
}

impl FromStr for Map {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .map(|l| l.parse::<MapRange>())
            .collect::<Result<_, _>>()?;

        Ok(Self { ranges })
    }
}

// ===

struct MapRange {
    dest_start: u32,
    source_start: u32,
    len: u32,
}

impl MapRange {
    fn range(&self) -> std::ops::Range<u32> {
        self.source_start..(self.source_start + self.len)
    }
}

impl FromStr for MapRange {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.split_whitespace().take(3).collect::<Vec<_>>();

        Ok(Self {
            dest_start: numbers[0].parse()?,
            source_start: numbers[1].parse()?,
            len: numbers[2].parse()?,
        })
    }
}

// ===

#[cfg(test)]
mod test {
    use super::Day05;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day05.part_a(INPUT), 35.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day05.part_b(INPUT), 46.into());
    }
}
