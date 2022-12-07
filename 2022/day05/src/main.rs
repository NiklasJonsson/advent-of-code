fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        if line.trim().is_empty() {
            // Stacks are done, reverse each stack
            assert!(!stacks.is_empty());
            for v in &mut stacks {
                v.reverse();
            }
        } else if line.starts_with("move") {
            let [_move, count, _from, start, _to, end] = shared::split_whitespace_n(line).unwrap();
            let count = count.parse::<usize>().unwrap();
            let start = start.parse::<usize>().unwrap() - 1;
            let end = end.parse::<usize>().unwrap() - 1;
            assert!(start < stacks.len() && end < stacks.len());

            let new_len = stacks[start].len() - count;
            for i in new_len..stacks[start].len() {
                let c = stacks[start][i];
                stacks[end].push(c);
            }
            stacks[start].truncate(new_len);
        } else {
            let itr = line.chars().skip(1);
            for (i, c) in itr.enumerate() {
                if i % 4 == 0 && ('A'..='Z').contains(&c) {
                    let idx = i / 4;
                    if idx >= stacks.len() {
                        stacks.extend(std::iter::repeat(Vec::new()).take(idx - stacks.len() + 1));
                    }
                    stacks[idx].push(c);
                }
            }
        }
    }

    println!(
        "Answer: {}",
        stacks
            .into_iter()
            .map(|v| v.last().cloned().unwrap_or(' '))
            .collect::<String>()
    );

    Ok(())
}
