use crate::common::{Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;

        for (i, line) in input.lines().enumerate() {
            let game = parse_game(line);

            if game.iter().all(|s| s.0 <= 12 && s.1 <= 13 && s.2 <= 14) {
                sum += i + 1;
            }
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let game = parse_game(line);

            let r = game.iter().map(|(r, _, _)| r).max().unwrap();
            let g = game.iter().map(|(_, g, _)| g).max().unwrap();
            let b = game.iter().map(|(_, _, b)| b).max().unwrap();

            sum += r * g * b;
        }

        sum.into()
    }
}

fn parse_game(line: &str) -> Vec<(u32, u32, u32)> {
    let mut game = Vec::new();

    let sets = line.split_once(':').unwrap().1.split(';');
    for set in sets {
        let mut cols = (0, 0, 0);

        for color in set.split(',') {
            let color = color.trim();

            if let Some((n, color)) = color.split_once(' ') {
                let n = n.parse::<u32>().unwrap();
                match color {
                    "red" => cols.0 = n,
                    "green" => cols.1 = n,
                    "blue" => cols.2 = n,
                    _ => {}
                }
            }
        }

        game.push(cols);
    }

    game
}

#[cfg(test)]
mod test {
    use super::Day02;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day02.part_a(INPUT), 8.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day02.part_b(INPUT), 2286.into());
    }
}
