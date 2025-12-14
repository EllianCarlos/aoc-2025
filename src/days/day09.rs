use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
pub struct Point {
    x: u64,
    y: u64,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Point> {
    input.lines().map(|line| { 
        let split_part: Vec<&str> = line.split(",").collect();

        if split_part.len() != 2 {
            panic!("Line should have two coordinates! {}", line);
        }

        let x_raw: &str = split_part[0];
        let y_raw: &str = split_part[1];
        let x: u64 = x_raw.parse().unwrap();
        let y: u64 = y_raw.parse().unwrap();

        Point { x, y }
    }).collect::<Vec<Point>>()
}


#[aoc(day9, part1)]
pub fn part1(input: &[Point]) -> u64 {
    input
        .iter()
        .enumerate()
        .map(|(i1, p)| {
            input
                .iter()
                .enumerate()
                .filter(|(i2, _)| *i2 != i1)
                .map(|(_, point1)| {
                    let l1 = p.x.abs_diff(point1.x) + 1;
                    let r1 = p.y.abs_diff(point1.y) + 1;

                    l1 * r1
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn part2(_input: &[Point]) -> u64 {
    0
}
