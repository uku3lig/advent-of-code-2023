use crate::common::{Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn name(&self) -> &'static str {
        "Wait For It"
    }

    fn part_a(&self, input: &str) -> Answer {
        let races = parse(input).unwrap();

        races
            .iter()
            .map(|r| r.winning_times())
            .product::<u64>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let race = parse_conc(input).unwrap();

        race.winning_times().into()
    }
}

fn parse(input: &str) -> Option<Vec<Race>> {
    let mut lines = input.lines();

    let times = parse_list(lines.next()?)?;
    let distances = parse_list(lines.next()?)?;

    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, dist)| Race {
            time,
            best_distance: dist,
        })
        .collect();

    Some(races)
}

fn parse_list(line: &str) -> Option<Vec<u64>> {
    line.split_once(':')?
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<_, _>>()
        .ok()
}

fn parse_conc(input: &str) -> Option<Race> {
    let mut lines = input.lines();

    let time = parse_spaced(lines.next()?)?;
    let best_distance = parse_spaced(lines.next()?)?;

    Some(Race {
        time,
        best_distance,
    })
}

fn parse_spaced(line: &str) -> Option<u64> {
    line.split_once(':')?
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .ok()
}

struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    fn winning_times(&self) -> u64 {
        // avoids iterating over ALL the times
        // you could technically make this even faster using math but im lazy :3

        let first_time = 't: {
            for t in 0..=self.time {
                let distance = t * (self.time - t);
                if distance > self.best_distance {
                    break 't t;
                }
            }

            u64::MAX
        };

        let last_time = 't: {
            for t in (0..=self.time).rev() {
                let distance = t * (self.time - t);
                if distance > self.best_distance {
                    break 't t;
                }
            }

            u64::MAX
        };

        last_time - first_time + 1
    }
}

#[cfg(test)]
mod test {
    use super::Day06;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day06.part_a(INPUT), 288.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day06.part_b(INPUT), 71503.into());
    }
}
