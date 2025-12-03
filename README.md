# Advent of Code 2025

This repository contains my solutions for the [Advent of Code 2025](https://adventofcode.com/2025) puzzles, written in Rust.

## About this Project

This project is set up to provide a fun and efficient environment for solving the Advent of Code puzzles. It leverages the `aoc-runner` crate to automatically handle input parsing, running solutions, and benchmarking. The development environment is managed by `nix` and `devenv` to ensure all dependencies and tools are available and consistent.

## Getting Started

1.  **Prerequisites**: Make sure you have `nix` and `direnv` installed on your system.
2.  **Environment Setup**: Navigate to the project directory and run `direnv allow`. This will automatically install all the necessary tools and dependencies, including the Rust toolchain and `aoc-cli`.
3.  **Session Token**: To download puzzle inputs and submit answers, you need to set your Advent of Code session token. Create a file named `.env` in the root of the project and add the following line:
    ```
    AOC_SESSION=<your_session_token>
    ```
    You can get your session token from the cookies of the Advent of Code website.

## Usage

### Running Solutions

To run the solution for a particular day, you can use the `cargo aoc` command with the `-d` (or `--day`) flag. For example, to run the code for Day 1, you would execute:

```bash
cargo aoc -d 1
```

The `aoc-runner` expects the input for each day to be in a specific file structure within an `input` directory. For example, the input for Day 1 of 2025 should be located at `input/2025/day1.txt`.

You can use the `aoc-cli` to download the input for a specific day. For example, to download the input for Day 1, run:

```bash
aoc download -d 1
```

This command will create the `input/2025/day1.txt` file with the correct puzzle input.

### Creating a New Day's Solution

To create a new solution for a given day, you can create a new file in the `src/days/` directory. For example, for Day 3, you would create `src/days/day03.rs`.

Here is a template you can use for the new file:

```rust
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<u32>> {
    Ok(input.lines().map(|l| l.parse().unwrap()).collect())
}

#[aoc(day3, part1)]
pub fn part1(input: &[u32]) -> u32 {
    0
}

#[aoc(day3, part2)]
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
```

Remember to also add the new file to `src/days/mod.rs`:

```rust
// src/days/mod.rs
pub mod day01;
pub mod day02;
pub mod day03; // Add this line
```

### Testing

You can write unit tests for your solutions within each day's module, inside the `#[cfg(test)]` block. To run all the tests, use the following command:

```bash
cargo test
```

### Submitting

Once you have a correct answer, you can use `aoc-cli` to submit it. The command is:

```bash
aoc submit -d <day> <part> <answer>
```

For example, to submit the answer for Day 1, Part 1:

```bash
aoc submit -d 1 1 12345
```
