#![feature(array_windows)]

use std::collections::HashMap;
use std::path::Path;

fn parse(file_contents: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
    let mut lines = file_contents.lines();

    let state = lines.next().unwrap().chars().collect();

    lines.next();

    let mut rules = HashMap::new();
    for line in &mut lines {
        let mut parts = line.split("->").map(|s| s.trim());

        let mut input = parts.next().unwrap().chars();
        let output = parts.next().unwrap().chars().next().unwrap();
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        rules.insert([first, second], output);
    }

    (state, rules)
}

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let (mut state, rules) = parse(&contents);

    let n = 10;
    let mut next_state = Vec::new();
    for _ in 0..n {
        next_state.clear();
        next_state.push(state[0]);
        for pair in state.array_windows::<2>() {
            next_state.push(rules[pair]);
            next_state.push(pair[1]);
        }

        std::mem::swap(&mut state, &mut next_state);
    }

    let mut counts = HashMap::new();
    for c in state {
        *counts.entry(c).or_insert(0) += 1;
    }
    let (min, max) = counts
        .into_iter()
        .fold((usize::MAX, usize::MIN), |(min, max), (_, count)| {
            (std::cmp::min(min, count), std::cmp::max(max, count))
        });

    println!("PART1 max: {}, min: {}, diff: {}", max, min, max - min);

    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let (state, rules) = parse(&contents);

    let mut pairs = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    // Setup start state
    for pair in state.array_windows::<2>() {
        *pairs.entry(*pair).or_insert(0) += 1;
    }

    for c in state {
        *counts.entry(c).or_insert(0) += 1;
    }

    let n = 40;
    for _ in 0..n {
        // Recreate the map. Each pair in the map creates two new pairs
        let mut new_pairs = HashMap::new();
        for (pair, count) in pairs.into_iter() {
            let out = rules[&pair];
            *counts.entry(out).or_insert(0) += count;

            let p0 = [pair[0], out];
            let p1 = [out, pair[1]];
            *new_pairs.entry(p0).or_insert(0) += count;
            *new_pairs.entry(p1).or_insert(0) += count;
        }
        pairs = new_pairs;
    }

    let (min, max) = counts
        .values()
        .fold((usize::MAX, usize::MIN), |(min, max), &count| {
            (std::cmp::min(min, count), std::cmp::max(max, count))
        });

    println!("PART2 max: {}, min: {}, diff: {}", max, min, max - min);

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
