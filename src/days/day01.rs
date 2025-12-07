use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    let initial_pos = 50i32;

    let positions = input.iter().scan(initial_pos, |current_pos, line| {
        let type_operation = line.chars().next().unwrap();
        let rest_of_the_string = &line[type_operation.len_utf8()..];
        let number: i32 = rest_of_the_string.parse().unwrap();

        let next_val = match type_operation {
            'L' => *current_pos - number,
            'R' => *current_pos + number,
            _ => todo!(),
        };

        let next_pos = next_val.rem_euclid(100);

        *current_pos = next_pos;
        Some(next_pos)
    });

    let zero_count = std::iter::once(initial_pos)
        .chain(positions)
        .filter(|&pos| pos == 0)
        .count() as u32;

    zero_count
}

#[aoc(day1, part2)]
pub fn part2(_input: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/manual.txt").unwrap();
        // let input = fs::read_to_string("input/2025/day1.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        // let input = fs::read_to_string("input/2025/day1.txt").unwrap();
        let input = fs::read_to_string("input/2025/manual.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
