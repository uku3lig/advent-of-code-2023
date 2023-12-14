use crate::common::{Answer, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Hot Springs"
    }

    fn part_a(&self, input: &str) -> Answer {
        input.lines().map(combinations).sum::<u32>().into()
    }

    #[allow(unreachable_code, unused_variables)]
    fn part_b(&self, input: &str) -> Answer {
        unimplemented!("do not run this");

        input
            .lines()
            .map(|l| {
                let (p, n) = l.split_once(char::is_whitespace).unwrap();
                [p].repeat(5).join("?") + " " + [n].repeat(5).join(",").as_str()
            })
            .map(|l| combinations(&l))
            .sum::<u32>()
            .into()
    }
}

fn combinations(line: &str) -> u32 {
    let (pattern, nums) = line.split_once(char::is_whitespace).unwrap();

    let nums = nums
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut patterns = vec![pattern.to_string()];

    while patterns.first().unwrap().contains('?') {
        let mut new_patterns = vec![];

        for p in &patterns {
            new_patterns.push(p.replacen('?', ".", 1));
            new_patterns.push(p.replacen('?', "#", 1));
        }

        patterns = new_patterns;
    }

    patterns.iter().filter(|p| check(p, &nums)).count() as u32
}

fn check(p: &str, nums: &[u32]) -> bool {
    let lens = p
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len() as u32)
        .collect::<Vec<_>>();

    lens == nums
}

#[cfg(test)]
mod test {
    use super::Day12;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day12.part_a(INPUT), 21.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day12.part_b(INPUT), 525152.into());
    }
}
