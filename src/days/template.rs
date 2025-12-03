use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day{{day}})]
pub fn input_generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[aoc(day{{day}}, part1)]
pub fn part1(input: &[&str]) -> u32 {
    0
}

#[aoc(day{{day}}, part2)]
pub fn part2(input: &[&str]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day{{day}}.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day{{day}}.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}