use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.to_string()) .collect()
}

fn is_repeating_twice(s: &str) -> bool {
    if s.len() % 2 == 1 { return false; }

    let (a, b) = s.split_at(s.len()/2);

    a == b
}

fn is_repeating(s: &str) -> bool {
    if s.len() < 2 { return false; }

    let doubled = s.repeat(2);

    let trimmed = &doubled[1..doubled.len()-1];

    trimmed.contains(s)
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> u64 {
    input.iter().fold(0u64, |acc, range| {
        let ranges: Vec<&str> = range.split('-').collect::<Vec<&str>>();
        if ranges.len() < 2 {
            return acc;
        }
        let id1 = ranges.first().expect("Trust me").parse::<u64>().unwrap();
        let id2 = ranges.last().expect("Trust me").parse::<u64>().unwrap();
        let seq = id1..=id2;

        acc + seq.fold(0u64, |acc_s, n| {
            if is_repeating_twice(&n.to_string()) {
                println!("Invalid {n}");
                acc_s + n
            } else {
                acc_s
            }
        })
    })
}

#[aoc(day2, part2)]
pub fn part2(_input: &[String]) -> u64 {
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
