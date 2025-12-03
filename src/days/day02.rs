use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn part1(_input: &[String]) -> u32 {
    0
}

#[aoc(day2, part2)]
pub fn part2(_input: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day2.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day2.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
