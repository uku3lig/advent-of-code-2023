use itertools::Itertools;

use crate::common::{Answer, Solution};

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part_a(&self, input: &str) -> Answer {
        compute_winnings_sum(input, false).into()
    }

    fn part_b(&self, input: &str) -> Answer {
        compute_winnings_sum(input, true).into()
    }
}

fn compute_winnings_sum(input: &str, with_joker: bool) -> u32 {
    let mut hands = Vec::new();

    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();

        let hand = Hand::from_str(hand, with_joker).unwrap();
        let bid = bid.parse::<u32>().unwrap();

        hands.push((hand, bid));
    }

    hands.sort_by_key(|(h, _)| *h);
    let mut winnings = 0;

    for (i, (_, bid)) in hands.iter().enumerate() {
        winnings += bid * (i as u32 + 1);
    }

    winnings
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn from_str(s: &str, with_joker: bool) -> Result<Self, Vec<Card>> {
        let cards: [Card; 5] = s
            .chars()
            .take(5)
            .map(|c| Card::from_char(c, with_joker))
            .collect::<Vec<_>>()
            .try_into()?;

        Ok(Self {
            hand_type: HandType::from_cards(&cards),
            cards,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(value: char, joker: bool) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => {
                if joker {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => unreachable!("card {value} doesnt exist"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,     // 1
    OnePair,      // 2
    TwoPairs,     // 2
    ThreeOfAKind, // 3
    FullHouse,    // 3
    FourOfAKind,  // 4
    FiveOfAKind,  // 5
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut counts = cards.iter().copied().counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);

        let mut counts = counts.values().copied().sorted().rev(); // reversed for descending order

        let highest = counts.next().unwrap_or(0) + jokers;
        let next = counts.next();

        match (highest, next) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, Some(2)) => Self::FullHouse,
            (3, Some(1)) => Self::ThreeOfAKind,
            (2, Some(2)) => Self::TwoPairs,
            (2, Some(1)) => Self::OnePair,
            (1, _) => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Day07;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day07.part_a(INPUT), 6440.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day07.part_b(INPUT), 5905.into());
    }

    #[test]
    fn hand_order() {
        use super::{Card, Hand, HandType};

        let hand_1 = Hand {
            hand_type: HandType::FourOfAKind,
            cards: [
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Two,
            ],
        };

        let hand_2 = Hand {
            hand_type: HandType::FourOfAKind,
            cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        };

        assert!(hand_1 > hand_2);
    }
}
