#![feature(bool_to_option)]

use std::path::Path;

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {

    let contents = std::fs::read_to_string(path)?;
    let mut points = Vec::new();
    let mut lines = contents.lines();
    for line in &mut lines {
        if line.trim() == "" {
            break;
        }

        let mut parts = line.split(',').map(|s| s.parse::<u32>().expect("Failed to parse number"));
        points.push([parts.next().unwrap(), parts.next().unwrap()]);
    }

    for fold_cmd in lines {
        let mut parts = fold_cmd.split_whitespace();
        debug_assert_eq!(parts.next().unwrap(), "fold");
        debug_assert_eq!(parts.next().unwrap(), "along");
        let mut cmd = parts.next().unwrap().split('=');
        let axis = cmd.next().unwrap();
        let val = cmd.next().unwrap().parse::<u32>()?;

        println!("Fold: {}, {}", axis, val);
        break;
    }

    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {

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
