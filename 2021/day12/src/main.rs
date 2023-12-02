use std::collections::HashMap;
use std::path::Path;

fn is_uppercase(s: &str) -> bool {
    s.chars().all(char::is_uppercase)
}

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = HashMap::new();

    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        let mut parts = line.split_terminator('-');
        let src = parts.next().unwrap().trim();
        let dst = parts.next().unwrap().trim();
        graph.entry(src).or_insert_with(Vec::new).push(dst);
        graph.entry(dst).or_insert_with(Vec::new).push(src);
    }

    let mut stack = vec![(vec![], "start")];
    let mut paths = Vec::new();
    let mut cur_path = Vec::new();
    while let Some((p, n)) = stack.pop() {
        while cur_path != p {
            cur_path.pop();
        }
        cur_path.push(n);
        if n == "end" {
            paths.push(cur_path.clone());
            continue;
        }
        for &e in graph[n].iter() {
            if is_uppercase(e) || !cur_path.contains(&e) {
                let mut new_p = p.clone();
                new_p.push(n);
                stack.push((new_p, e));
            }
        }
    }

    println!("N: {}", paths.len());

    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = HashMap::new();

    let contents = std::fs::read_to_string(path)?;
    for line in contents.lines() {
        let mut parts = line.split_terminator('-');
        let src = parts.next().unwrap().trim();
        let dst = parts.next().unwrap().trim();
        graph.entry(src).or_insert_with(Vec::new).push(dst);
        graph.entry(dst).or_insert_with(Vec::new).push(src);
    }

    let mut stack = vec![vec!["start"]];
    let mut paths = Vec::new();

    let allow_visit = |cur_path: &[&str], cave: &str| {
        if is_uppercase(cave) {
            return true;
        }

        if cave == "start" {
            return false;
        }

        let occ = cur_path.iter().filter(|x| **x == cave).count();
        if occ == 0 {
            return true;
        }

        if occ == 2 {
            return false;
        }
        assert!(occ == 1);

        for (i, node_i) in cur_path.iter().enumerate() {
            if i == (cur_path.len() - 1) {
                continue;
            }

            for node_j in cur_path[(i + 1)..cur_path.len()].iter() {
                if node_i == node_j && !is_uppercase(node_i) {
                    // Already a double visit
                    return false;
                }
            }
        }

        true
    };

    while let Some(p) = stack.pop() {
        let n = *p.last().expect("There cannot be empty paths");
        if n == "end" {
            paths.push(p);
            continue;
        }
        for &e in graph[n].iter() {
            if allow_visit(&p, e) {
                let mut new_p = p.clone();
                new_p.push(e);
                stack.push(new_p);
            }
        }
    }

    println!("N: {}", paths.len());

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
