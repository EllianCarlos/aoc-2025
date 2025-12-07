use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

pub struct Ingredients {
    pub rules: Vec<(u64, u64)>,
    pub queries: Vec<u64>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Ingredients {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules = parts[0]
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
        })
        .collect();

    let queries = parts[1]
        .lines()
        .map(|line| {
            line.parse::<u64>().unwrap()
        })
        .collect();

    Ingredients { rules, queries }
}

#[aoc(day5, part1)]
pub fn part1(input: &Ingredients) -> u64 {
    // Maybe we should use a BST an Interval Tree os some sort of sorted array with ranges
    // to improve the complexity time of this 
    // but for AOC day5, this is good.
    input.queries.iter().fold(0u64, |total_valid, n| { 
        for (s, e) in input.rules.iter() {
            if s <= n && n <= e {
                return total_valid + 1;
            }
        }

        total_valid
    })
}

#[aoc(day5, part2)]
pub fn part2(_input: &Ingredients) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day5.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day5.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
