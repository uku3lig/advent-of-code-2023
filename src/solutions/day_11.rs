use crate::common::{Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn name(&self) -> &'static str {
        "Cosmic Expansion"
    }

    fn part_a(&self, input: &str) -> Answer {
        let galaxies = expanded_galaxies(input, 1);

        pairs(galaxies.len())
            .iter()
            .map(|(first, other)| distance(&galaxies, *first, *other))
            .sum::<usize>()
            .into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let galaxies = expanded_galaxies(input, 999_999);

        pairs(galaxies.len())
            .iter()
            .map(|(first, other)| distance(&galaxies, *first, *other))
            .sum::<usize>()
            .into()
    }
}

fn expanded_galaxies(input: &str, amount: usize) -> Vec<(usize, usize)> {
    let empty_lines = empty_lines(input);
    let empty_columns = empty_columns(input);

    let mut galaxies = find_galaxies(input);

    for (line_i, col_i) in galaxies.iter_mut() {
        *line_i += empty_lines.iter().filter(|i| *i < line_i).count() * amount;
        *col_i += empty_columns.iter().filter(|i| *i < col_i).count() * amount;
    }

    galaxies
}

fn empty_lines(input: &str) -> Vec<usize> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| line.chars().all(|c| c == '.').then_some(i))
        .collect()
}

fn empty_columns(input: &str) -> Vec<usize> {
    let len = input.lines().next().unwrap().len();

    (0..len)
        .filter(|i| {
            input
                .lines()
                .filter_map(|l| l.chars().nth(*i))
                .all(|c| c == '.')
        })
        .collect()
}

fn find_galaxies(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_i, line)| {
            line.char_indices()
                .filter_map(move |(col_i, c)| (c == '#').then_some((line_i, col_i)))
        })
        .collect()
}

fn pairs(n: usize) -> Vec<(usize, usize)> {
    (1..n)
        .flat_map(|i| ((i + 1)..=n).map(move |j| (i, j)))
        .map(|(x, y)| (x - 1, y - 1)) // ugly but it works
        .collect()
}

fn distance(galaxies: &[(usize, usize)], first: usize, other: usize) -> usize {
    let (first_x, first_y) = galaxies.get(first).unwrap();
    let (other_x, other_y) = galaxies.get(other).unwrap();

    let v_dist = first_x.max(other_x) - first_x.min(other_x);
    let h_dist = first_y.max(other_y) - first_y.min(other_y);

    v_dist + h_dist
}

#[cfg(test)]
mod test {
    use super::Day11;
    use crate::common::Solution;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day11.part_a(INPUT), 374.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day11.part_b(INPUT), 82000210.into());
    }
}
