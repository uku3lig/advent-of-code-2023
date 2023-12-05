use crate::common::{Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Scratchcards"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;

        for card in parse_cards(input) {
            let winning = card
                .winning
                .iter()
                .filter(|e| card.numbers.contains(e))
                .count() as u32;

            if winning > 0 {
                sum += 2u32.pow(winning - 1);
            }
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let cards = parse_cards(input);
        let mut amounts = vec![1; cards.len()];

        for (i, card) in cards.iter().enumerate() {
            let winning = card
                .winning
                .iter()
                .filter(|e| card.numbers.contains(e))
                .count();

            let current_amount = amounts[i];

            amounts
                .iter_mut()
                .skip(i + 1)
                .take(winning)
                .for_each(|a| *a += current_amount);
        }

        amounts.iter().sum::<u32>().into()
    }
}

struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(parse_card).collect()
}

fn parse_card(line: &str) -> Card {
    let (winning, numbers) = line.split_once(':').unwrap().1.split_once('|').unwrap();

    let winning = winning
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    let numbers = numbers
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<_>>();

    Card { winning, numbers }
}

#[cfg(test)]
mod test {
    use super::Day04;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day04.part_a(INPUT), 13.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day04.part_b(INPUT), 30.into());
    }
}
