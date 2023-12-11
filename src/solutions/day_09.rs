use crate::common::{Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Mirage Maintenance"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let line = parse_line(line);
            sum += get_next(line);
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let mut line = parse_line(line);
            line.reverse();

            sum += get_next(line);
        }

        sum.into()
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// who would've thought that learning recursive functions in scheme would help for me AoC
fn get_next(nums: Vec<i32>) -> i32 {
    let differences = nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let next = if differences.iter().all(|n| *n == 0) {
        0
    } else {
        get_next(differences.clone())
    };

    next + nums.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::Day09;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day09.part_a(INPUT), 114.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day09.part_b(INPUT), 2.into());
    }
}
