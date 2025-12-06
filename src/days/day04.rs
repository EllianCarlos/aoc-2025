use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use std::cmp::max;
use std::cmp::min;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[String]) -> u64 {
    let input_as_char_matrix: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut acessible_rolls = 0u64;

    let n = input_as_char_matrix.len() as i64;

    for i in 0i64..n {
        for j in 0i64..n {
            let mut roll_near = 0u64;

            for ii in max(0, i-1)..=min(i+1, n-1) {
                for jj in max(0, j-1)..=min(j+1, n-1) {
                    let eval_roll = input_as_char_matrix[ii as usize][jj as usize];
                    if eval_roll == '@' && (ii != i || jj != j) {
                        roll_near += 1u64;
                    } 
                }
            }


            if roll_near < 4u64 && input_as_char_matrix[i as usize][j as usize] == '@' {
                acessible_rolls += 1u64;
            }
        }
    }

    acessible_rolls
}

#[aoc(day4, part2)]
pub fn part2(_input: &[String]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/2025/day4.txt").unwrap();
        let result = part1(&input_generator(&input));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/2025/day4.txt").unwrap();
        let result = part2(&input_generator(&input));
        assert_eq!(result, 0);
    }
}
