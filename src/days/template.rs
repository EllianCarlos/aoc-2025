use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day{{day}})]
pub fn input_generator(input: &str) -> Result<Vec<u32>> {
    Ok(input.lines().map(|l| l.parse().unwrap()).collect())
}

#[aoc(day{{day}}, part1)]
pub fn part1(input: &[u32]) -> u32 {
    0
}

#[aoc(day{{day}}, part2)]
pub fn part2(input: &[u32]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        let result = part1(&input_generator(input).unwrap());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let result = part2(&input_generator(input).unwrap());
        assert_eq!(result, 0);
    }
}