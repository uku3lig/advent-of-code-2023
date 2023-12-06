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
        // +1 because you're not beating the distance if you tie it
        let delta = self.time.pow(2) - 4 * (self.best_distance + 1);
        let sq_delta = (delta as f64).sqrt();

        let t1 = (self.time as f64 - sq_delta) / 2.0;
        let t1 = t1.ceil() as u64;

        let t2 = (self.time as f64 + sq_delta) / 2.0;
        let t2 = t2.floor() as u64;

        t2 - t1 + 1
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
