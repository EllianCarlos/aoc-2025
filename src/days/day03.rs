use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u64 {
    input.iter().fold(0u64, |acc, line| {
        let chars_value = line[..line.len()-1].chars().fold(('-', '-'), |pair, curr| {
            if curr > pair.1 {
                return ('-', curr);
            };

            if curr > pair.0 {
                return (curr, pair.1);
            };

            pair
        });
        let (mut c1, c2) = chars_value;
        let last_char = line.chars().last().unwrap();
        if c1 == '-' || last_char > c1 {
            c1 = last_char;
        }
        let num = c2.to_digit(10).unwrap() * 10 + c1.to_digit(10).unwrap();
        acc + num as u64
    })
}

#[aoc(day3, part2)]
pub fn part2(_input: &[String]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day3.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day3.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
