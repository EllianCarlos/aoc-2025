use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    let dials = input.iter().fold(Vec::from([50i32]), |acc, line| {
        let type_operation = line.chars().next().unwrap();
        let rest_of_the_string = &line[type_operation.len_utf8()..];
        let number: i32 = rest_of_the_string.parse().unwrap();
        let next_val = match type_operation {
            'L' => acc.last().unwrap() - number,
            'R' => acc.last().unwrap() + number,
            _ => todo!(),
        };
        let abs_next_val = if next_val < 0 {
            next_val + 100
        } else {
            next_val
        };

        [acc, [abs_next_val % 100].to_vec()].concat()
    }).iter().fold(0, |acc, num| {
        if *num == 0 {
            acc + 1
        } else {
            acc
        }
    });
    println!("{dials}");
    dials
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
