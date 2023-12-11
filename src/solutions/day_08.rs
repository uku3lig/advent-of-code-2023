use std::collections::HashMap;

use gcd::Gcd;

use crate::common::{Answer, Solution};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Haunted Wasteland"
    }

    fn part_a(&self, input: &str) -> Answer {
        let map = Map::parse(input);

        map.compute_steps("AAA", |p| p == "ZZZ").into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let map = Map::parse(input);

        let positions = map
            .nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .collect::<Vec<_>>();

        let steps = positions
            .iter()
            .map(|p| map.compute_steps(p, |end| end.ends_with('Z')))
            .collect::<Vec<_>>();

        let steps = steps.iter().fold(1u64, |acc, next| lcm(acc, *next as u64));

        steps.into()
    }
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let directions = lines
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                'L' => Some(Direction::Left),
                'R' => Some(Direction::Right),
                _ => None,
            })
            .collect::<Vec<_>>();

        lines.next(); // empty line

        let mut nodes = HashMap::new();
        for line in lines {
            let (pos, paths) = line.split_once('=').unwrap();
            let (left, right) = paths.trim().split_once(',').unwrap();

            let left = left.trim().chars().skip(1).collect::<String>();
            let right = right
                .trim()
                .chars()
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>();

            nodes.insert(pos.trim().into(), (left, right));
        }

        Self { directions, nodes }
    }

    fn compute_steps(&self, start: impl ToString, break_cond: fn(&String) -> bool) -> u32 {
        let mut pos = start.to_string();
        let mut steps = 0u32;

        'l: loop {
            for direction in &self.directions {
                let paths = self.nodes.get(&pos).unwrap();
                pos = match direction {
                    Direction::Left => paths.0.clone(),
                    Direction::Right => paths.1.clone(),
                };

                steps += 1;

                if break_cond(&pos) {
                    break 'l;
                }
            }
        }

        steps
    }
}

// u32 is NOT big enough
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / a.gcd(b)
}

enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::Day08;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT_A: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const INPUT_B: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day08.part_a(INPUT_A), 2.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day08.part_b(INPUT_B), 6.into());
    }
}
