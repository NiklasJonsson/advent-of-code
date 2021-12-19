#![feature(bool_to_option)]
#![feature(hash_drain_filter)]

use std::collections::HashSet;
use std::path::Path;

type Point = [u32; 2];

fn parse_fold_cmd(line: &str) -> Result<(char, u32), Box<dyn std::error::Error>> {
    let mut parts = line.split_whitespace();
    debug_assert_eq!(parts.next().unwrap(), "fold");
    debug_assert_eq!(parts.next().unwrap(), "along");
    let mut cmd = parts.next().unwrap().split('=');
    let axis = cmd.next().unwrap();
    let fold_val = cmd.next().unwrap().parse::<u32>()?;

    debug_assert_eq!(axis.len(), 1);
    Ok((axis.chars().nth(0).unwrap(), fold_val))
}

fn parts(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut points = HashSet::new();
    let mut lines = contents.lines();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in &mut lines {
        if line.trim() == "" {
            break;
        }

        let mut parts = line
            .split(',')
            .map(|s| s.parse::<u32>().expect("Failed to parse number"));
        let p = [parts.next().unwrap(), parts.next().unwrap()];
        max_x = std::cmp::max(p[0], max_x);
        max_y = std::cmp::max(p[1], max_y);
        points.insert(p);
    }

    let mut width = max_x + 1;
    let mut height = max_y + 1;

    for line in lines {
        let (axis, fold_val) = parse_fold_cmd(line)?;

        let i = if axis == 'y' {
            height = fold_val;
            1
        } else {
            width = fold_val;
            0
        };

        debug_assert!(
            !points.iter().any(|p| p[i] == fold_val),
            "Unexpected point on fold line"
        );

        let folded: Vec<Point> = points.drain_filter(|p| p[i] > fold_val).collect();
        for f in folded {
            assert!(fold_val * 2 >= f[i], "{} >= {}", fold_val * 2, f[i]);
            let mut new = f;
            new[i] = fold_val - (f[i] - fold_val);
            points.insert(new);
        }
    }

    let mut set = HashSet::new();
    for p in points {
        set.insert(p);
    }

    for y in 0..height {
        for x in 0..width {
            let c = if !set.contains(&[x, y]) { '.' } else { '#' };
            print!("{}", c);
        }
        println!();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    parts(&args[1])?;

    Ok(())
}
