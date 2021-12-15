#![feature(bool_to_option)]

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

use std::path::Path;

const WIDTH: usize = 10;

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut levels = [[(0i32, false); WIDTH]; WIDTH];
    for (li, line) in contents.lines().enumerate() {
        for (ci, c) in line.chars().enumerate() {
            levels[li][ci].0 = c.to_digit(10).expect("Failed to parse") as i32;
        }
    }

    let neighbours = |x: isize, y: isize| {
        NEIGHBOURS.iter().filter_map(move |(x_offset, y_offset)| {
            let x_abs = x + x_offset;
            let y_abs = y + y_offset;
            (x_abs >= 0 && x_abs < WIDTH as isize && y_abs >= 0 && y_abs < WIDTH as isize)
                .then_some((x_abs as usize, y_abs as usize))
        })
    };

    let steps = 100;
    let mut stack = Vec::new();
    let mut sum = 0;
    for _step in 1..=steps {
        stack.clear();
        for (ri, row) in levels.iter_mut().enumerate() {
            for (ci, (lvl, has_run)) in row.iter_mut().enumerate() {
                *lvl += 1;
                *has_run = false;
                if *lvl > 9 {
                    stack.push((ci, ri));
                }
            }
        }

        let mut _prop_level = 0;
        while let Some((x, y)) = stack.pop() {
            levels[y][x].1 = true;
            for (x, y) in neighbours(x as isize, y as isize) {
                if !levels[y][x].1 {
                    levels[y][x].0 += 1;
                    if levels[y][x].0 > 9 && !stack.contains(&(x, y)) {
                        stack.push((x, y));
                    }
                }
            }
            _prop_level += 1;
        }

        for row in levels.iter_mut() {
            for (lvl, has_run) in row.iter_mut() {
                if *has_run {
                    assert!(*lvl > 9);
                    *lvl = 0;
                }
            }
        }

        sum += levels
            .iter()
            .flat_map(|r| r.iter().map(|x| x.1))
            .filter(|x| *x)
            .count();
    }

    println!("sum: {}", sum);

    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut levels = [[(0i32, false); WIDTH]; WIDTH];
    for (li, line) in contents.lines().enumerate() {
        for (ci, c) in line.chars().enumerate() {
            levels[li][ci].0 = c.to_digit(10).expect("Failed to parse") as i32;
        }
    }

    let neighbours = |x: isize, y: isize| {
        NEIGHBOURS.iter().filter_map(move |(x_offset, y_offset)| {
            let x_abs = x + x_offset;
            let y_abs = y + y_offset;
            (x_abs >= 0 && x_abs < WIDTH as isize && y_abs >= 0 && y_abs < WIDTH as isize)
                .then_some((x_abs as usize, y_abs as usize))
        })
    };

    let mut step = 0;
    let mut stack = Vec::new();
    loop {
        step += 1;

        stack.clear();
        for (ri, row) in levels.iter_mut().enumerate() {
            for (ci, (lvl, has_run)) in row.iter_mut().enumerate() {
                *lvl += 1;
                *has_run = false;
                if *lvl > 9 {
                    stack.push((ci, ri));
                }
            }
        }

        let mut _prop_level = 0;
        while let Some((x, y)) = stack.pop() {
            levels[y][x].1 = true;
            for (x, y) in neighbours(x as isize, y as isize) {
                if !levels[y][x].1 {
                    levels[y][x].0 += 1;
                    if levels[y][x].0 > 9 && !stack.contains(&(x, y)) {
                        stack.push((x, y));
                    }
                }
            }
            _prop_level += 1;
        }

        for row in levels.iter_mut() {
            for (lvl, has_run) in row.iter_mut() {
                if *has_run {
                    assert!(*lvl > 9);
                    *lvl = 0;
                }
            }
        }

        let all = levels
            .iter()
            .flat_map(|r| r.iter().map(|x| x.1))
            .reduce(|acc, e| acc && e)
            .unwrap();

        if all {
            println!("step: {}", step);
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    part1(&args[1])?;
    part2(&args[1])?;

    Ok(())
}
