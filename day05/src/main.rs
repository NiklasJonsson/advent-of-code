#![feature(drain_filter)]
#![feature(bool_to_option)]
#![allow(dead_code)]

use std::path::Path;

const ARRAY_SIZE: usize = 1000;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    vents: Vec<[usize; ARRAY_SIZE]>,
}

impl Default for Map {
    fn default() -> Self {
        let mut vents = Vec::with_capacity(ARRAY_SIZE);
        for _ in 0..ARRAY_SIZE {
            vents.push([0; ARRAY_SIZE]);
        }

        Self { vents }
    }
}

impl Map {
    fn dump(&self, mut w: impl std::io::Write) {
        let contents = self
            .vents
            .iter()
            .map(|r| {
                r.iter()
                    .fold(String::with_capacity(ARRAY_SIZE), |mut acc, v| {
                        if *v == 0 {
                            acc.push('.');
                        } else {
                            acc.push_str(&format!("{}", v));
                        }
                        acc
                    })
            })
            .fold(String::new(), |mut acc, row| {
                acc.push_str(&row);
                acc.push('\n');
                acc
            });

        w.write_all(contents.as_bytes())
            .expect("Failed to write map");
    }

    fn filedump(&self, p: &Path) {
        self.dump(std::fs::File::create(p).expect("Failed to create file"));
    }
}

fn min_max(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn dist(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn is_diagonal([x0, y0]: [usize; 2], [x1, y1]: [usize; 2]) -> bool {
    dist(x0, x1) == dist(y0, y1)
}

fn is_straight([x0, y0]: [usize; 2], [x1, y1]: [usize; 2]) -> bool {
    x0 == x1 || y0 == y1
}

impl Map {
    fn add_vent(&mut self, x: usize, y: usize) {
        self.vents[y][x] += 1;
    }

    fn add_vents(&mut self, start: [usize; 2], end: [usize; 2]) {
        if is_diagonal(start, end) {
            let ([x0, y0], [x1, y1]) = (start, end);
            let dist = dist(x0, x1);
            if x0 < x1 && y0 < y1 {
                for i in 0..dist + 1 {
                    self.add_vent(x0 + i, y0 + i);
                }
            } else if x0 < x1 && y0 > y1 {
                for i in 0..dist + 1 {
                    self.add_vent(x0 + i, y0 - i);
                }
            } else if x0 > x1 && y0 > y1 {
                for i in 0..dist + 1 {
                    self.add_vent(x0 - i, y0 - i);
                }
            } else {
                assert!(x0 > x1 && y0 < y1);
                for i in 0..dist + 1 {
                    self.add_vent(x0 - i, y0 + i);
                }
            }
        } else if is_straight(start, end) {
            let ([x0, y0], [x1, y1]) = (start, end);
            if x0 == x1 {
                let (min_y, max_y) = min_max(y0, y1);
                for y in min_y..max_y + 1 {
                    self.add_vent(x0, y);
                }
            } else {
                assert!(y0 == y1);
                let (min_x, max_x) = min_max(x0, x1);
                for x in min_x..max_x + 1 {
                    self.add_vent(x, y0);
                }
            }
        } else {
            println!("Ignoring {:?}", (start, end));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day05 <file>");
        return Err("Error: expected file arg".into());
    }

    let mk_arr_2 = |s: &str| -> [usize; 2] {
        let mut itr = s
            .trim()
            .split_terminator(',')
            .map(|n| n.parse::<usize>().expect("Failed to parse usize"));
        [
            itr.next().expect("Missing usize"),
            itr.next().expect("Missing usize"),
        ]
    };

    let contents = std::fs::read_to_string(&args[1])?;
    let mut map = Map::default();
    for l in contents.lines() {
        let mut points = l.split_terminator("->").map(mk_arr_2);
        let start = points.next().unwrap();
        let end = points.next().unwrap();
        map.add_vents(start, end);
    }

    let count = map
        .vents
        .iter()
        .map(|v| v.iter())
        .flatten()
        .filter(|e| **e >= 2)
        .count();
    println!("count: {}", count);

    Ok(())
}
