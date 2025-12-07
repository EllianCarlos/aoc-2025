use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

pub struct Problem {
    nums: Vec<u64>,
    op: char,
}

fn transpose(input: Vec<Vec<String>>) -> Vec<Vec<String>> {
    if input.is_empty() {
        return vec![];
    }

    let m = input[0].len();

    (0..m)
        .map(|j| {
            input
                .iter()
                .map(|row| row[j].clone())
                .collect()
        })
        .collect()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Problem> {
    let transposed = transpose(input.lines().map(|line| line.split_whitespace().map(|s| s.to_string()).collect()).collect());

    transposed.into_iter().map(|mut col| {
        let op_str = col.pop().unwrap();
        let op = op_str.chars().next().unwrap();

        let nums = col.into_iter()
            .map(|s| s.parse().unwrap())
            .collect();

        Problem { nums, op }
    }).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Problem]) -> u64 {
    input.iter().map(|p| {
            match p.op {
                '+' => p.nums.iter().sum::<u64>(),
                '*' => p.nums.iter().product::<u64>(),
                _ => panic!("Unknown Operator: {:?}", p.op),
            }
        }).sum()
}

#[aoc(day6, part2)]
pub fn part2(_input: &[Problem]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day6.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day6.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
