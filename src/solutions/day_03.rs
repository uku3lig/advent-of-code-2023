use std::collections::HashMap;

use crate::common::{Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Gear Ratios"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;

        for ((line_i, c_i), num) in get_numbers(input) {
            let len = num.to_string().len();

            let lines = input
                .lines()
                .skip(line_i.saturating_sub(1)) // avoid underflow
                .take(if line_i == 0 { 2 } else { 3 });

            let mut chars = lines.flat_map(|l| l.chars().skip(c_i.saturating_sub(1)).take(len + 2));

            if chars.any(|c| !c.is_ascii_digit() && c != '.') {
                sum += num;
            }
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let gears = get_gears(input);
        let mut ratios = gears.iter().map(|c| (c, vec![])).collect::<HashMap<_, _>>();

        for ((line_i, c_i), num) in get_numbers(input) {
            let len = num.to_string().len();

            for line in line_i.saturating_sub(1)..=(line_i + 1) {
                for chr in c_i.saturating_sub(1)..=(c_i + len) {
                    if let Some(vec) = ratios.get_mut(&(line, chr)) {
                        vec.push(num);
                    }
                }
            }
        }

        ratios
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum::<u32>()
            .into()
    }
}

fn get_numbers(input: &str) -> HashMap<(usize, usize), u32> {
    let mut numbers = HashMap::new();

    for (line_i, line) in input.lines().enumerate() {
        let mut offset = 0;
        while let Some(num_i) = line[offset..].find(|c: char| c.is_ascii_digit()) {
            let num_i = num_i + offset;
            let num_str = line[num_i..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            if let Ok(num) = num_str.parse::<u32>() {
                numbers.insert((line_i, num_i), num);
            }

            offset = num_i + num_str.len();
        }
    }

    numbers
}

fn get_gears(input: &str) -> Vec<(usize, usize)> {
    let mut coords = vec![];

    for (line_i, line) in input.lines().enumerate() {
        line.char_indices()
            .filter(|(_, c)| *c == '*')
            .for_each(|(c_i, _)| coords.push((line_i, c_i)));
    }

    coords
}

#[cfg(test)]
mod test {
    use super::Day03;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day03.part_a(INPUT), 4361.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day03.part_b(INPUT), 467835.into());
    }
}
