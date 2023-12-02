use std::path::Path;

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let costs = std::collections::HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let contents = std::fs::read_to_string(path)?;
    let mut stack = Vec::new();
    let mut sum = 0;
    for line in contents.lines() {
        stack.clear();
        for c in line.chars() {
            if "{([<".contains(c) {
                stack.push(c);
                continue;
            }

            match (stack.last(), c) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                    stack.pop();
                }
                (last, cur) => {
                    let cost = costs[&cur];
                    println!("Error: {}, can't close {:?} (cost: {})", cur, last, cost);
                    sum += cost;
                    break;
                }
            }
        }
    }

    println!("error score: {}", sum);

    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    // Map opening brace to simplify logic in the loop
    let points = std::collections::HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let contents = std::fs::read_to_string(path)?;
    let mut stack = Vec::new();
    let mut scores = Vec::new();
    for line in contents.lines() {
        stack.clear();
        let mut invalid = false;
        for c in line.chars() {
            if "{([<".contains(c) {
                stack.push(c);
                continue;
            }

            match (stack.last(), c) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                    stack.pop();
                }
                _ => {
                    invalid = true;
                    break;
                }
            }
        }
        if !invalid && !stack.is_empty() {
            let mut result: usize = 0;
            while let Some(x) = stack.pop() {
                result = result * 5 + points[&x];
            }
            scores.push(result);
        }
    }

    assert_eq!(scores.len() % 2, 1);
    scores.sort_unstable();
    dbg!(&scores);

    println!("autocomplete score: {}", scores[scores.len() / 2]);

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
