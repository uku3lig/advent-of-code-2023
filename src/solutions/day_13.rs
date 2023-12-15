use crate::common::{Answer, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn name(&self) -> &'static str {
        "Point of Incidence"
    }

    fn part_a(&self, input: &str) -> Answer {
        mirrors(input, 0).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        mirrors(input, 1).into()
    }
}

type Grid = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn mirrors(input: &str, smudge: usize) -> usize {
    parse(input)
        .iter()
        .filter_map(|p| {
            horizontal_mirror(p, smudge)
                .map(|n| n * 100)
                .or_else(|| vertical_mirror(p, smudge))
        })
        .sum::<usize>()
}

fn horizontal_mirror(input: &Grid, smudge: usize) -> Option<usize> {
    let col_num = input[0].len();

    for middle in 1..input.len() {
        let len = middle.min(input.len() - middle);
        let start = middle - len;

        let mut wrong = 0;
        for left in start..middle {
            let right = middle * 2 - left - 1;

            wrong += (0..col_num)
                .filter(|&x| input[left][x] != input[right][x])
                .count();

            if wrong > smudge {
                break;
            }
        }

        if wrong == smudge {
            return Some(middle);
        }
    }

    None
}

fn vertical_mirror(input: &Grid, smudge: usize) -> Option<usize> {
    let col_num = input[0].len();

    for middle in 1..col_num {
        let len = middle.min(col_num - middle);
        let start = middle - len;

        let mut wrong = 0;
        for left in start..middle {
            let right = middle * 2 - left - 1;

            wrong += (0..input.len())
                .filter(|&x| input[x][left] != input[x][right])
                .count();

            if wrong > smudge {
                break;
            }
        }

        if wrong == smudge {
            return Some(middle);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::Day13;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day13.part_a(INPUT), 405.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day13.part_b(INPUT), 400.into());
    }
}
