use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::f64;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Clone)]
struct Edge {
    a: usize,
    b: usize,
    dist: f64,
}

impl Point {
    pub fn distance_to(&self, other: Point) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct Streets {
    lamps: Vec<Point>,
    root: Vec<usize>,
    rank: Vec<u64>,
    pub disjoin_sets: u64,
}

impl Streets {
    pub fn new(lamps: Vec<Point>) -> Self {
        let len = lamps.len();
        Streets {
            lamps,
            root: (0..len).collect(),
            rank: vec![0; len],
            disjoin_sets: len as u64
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if node != self.root[node] {
            self.root[node] = self.find(self.root[node]);
        }


        self.root[node]
    }

    pub fn union(&mut self, src: usize, dest: usize) -> () {
        let root_src = self.find(src);
        let root_dest = self.find(dest);

        if root_src != root_dest {
            match self.rank[root_src].cmp(&self.rank[root_dest]) {
                std::cmp::Ordering::Less => self.root[root_src] = root_dest,
                std::cmp::Ordering::Greater => self.root[root_dest] = root_src,
                std::cmp::Ordering::Equal => {
                    self.root[root_dest] = root_src;
                    self.rank[root_src] += 1;
                }
            }

            self.disjoin_sets -= 1;
        }
    }

    pub fn lamps(&self) -> &[Point] {
        &self.lamps
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Streets {
    let lamps: Vec<Point> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            // slice destructuring is great!
            let [x_raw, y_raw, z_raw] = parts.as_slice() else {
                panic!("Invalid line format: {}", line);
            };
            
            Point {
                x: x_raw.parse().unwrap(),
                y: y_raw.parse().unwrap(),
                z: z_raw.parse().unwrap(),
            }
        })
        .collect();

    Streets::new(lamps)
}


#[aoc(day8, part1)]
pub fn part1(input: &Streets) -> u64 {
    let lamps = input.lamps();

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..lamps.len() {
        for j in (i + 1)..lamps.len() {
            edges.push(Edge {
                a: i,
                b: j,
                dist: lamps[i].distance_to(lamps[j]),
            });
        }
    }

    edges.sort_by(|e1, e2| e1.dist.total_cmp(&e2.dist));

    let mut streets = input.clone();
    let mut processed = 0;

    for edge in edges {
        if processed == 1000 {
            break;
        }

        streets.union(edge.a, edge.b);
        processed += 1;
    }

    let mut provinces_by_root: HashMap<usize, u64> = HashMap::new();

    for i in 0..streets.lamps.len() {
        let root = streets.find(i);
        *provinces_by_root.entry(root).or_default() += 1;
    }

    let mut sizes: Vec<u64> = provinces_by_root.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes.into_iter().take(3).product()
}

#[aoc(day8, part2)]
pub fn part2(_input: &Streets) -> u64 {
    0
}
