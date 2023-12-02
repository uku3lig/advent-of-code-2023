use crate::common::{Answer, Solution};

pub struct Day01;

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Trebuchet?!"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let mut numbers = line.chars().filter(|c| c.is_numeric());

            let Some(first) = numbers.next() else {
                continue;
            };

            let last = match numbers.next_back() {
                Some(n) => n,
                None => first,
            };

            if let Ok(n) = format!("{first}{last}").parse::<u32>() {
                sum += n;
            }
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut sum = 0;

        for line in input.lines() {
            let mut numbers = line
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .collect::<Vec<_>>();

            numbers.extend(find_words(line));

            numbers.sort_by_key(|(i, _)| *i);

            let Some((_, first)) = numbers.first() else {
                continue;
            };

            let last = match numbers.last() {
                Some((_, n)) => n,
                None => first,
            };

            if let Ok(n) = format!("{first}{last}").parse::<u32>() {
                sum += n;
            }
        }

        sum.into()
    }
}

fn find_words(input: &str) -> Vec<(usize, char)> {
    let mut words = Vec::new();
    let mut offset = 0;

    while offset < input.len() {
        let offset_input = &input[offset..];

        let next = NUMBERS
            .iter()
            .enumerate()
            .filter_map(|(n, w)| offset_input.find(w).map(|i| (i + offset, n + 1)))
            .min_by_key(|(i, _)| *i);

        if let Some((index, number)) = next {
            let char = format!("{number}").chars().next().unwrap();

            words.push((index, char));
            offset = index + 1;
        } else {
            break;
        }
    }

    words
}
