#![feature(iter_next_chunk)]

fn prio(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        return c as u32 - 'a' as u32 + 1;
    }


    if ('A'..='Z').contains(&c) {
        return c as u32 - 'A' as u32 + 27;
    }

    panic!("Invalid: {}", c);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    assert!(contents.lines().count() % 3 == 0);
    let mut sum = 0;

    let mut itr = contents.lines().map(str::trim).filter(|s| !s.is_empty());
    while let Ok(chunk) = itr.next_chunk::<3>() {
        let first: std::collections::HashSet<char> = std::collections::HashSet::from_iter(chunk[0].chars());
        let p: Option<u32> = first
        .iter()
        .find_map(|&c| (chunk[1].contains(c) && chunk[2].contains(c)).then(|| prio(c)));
        sum += p.expect("No common item in group");
    }


    println!("Answer: {sum}");

    Ok(())
}
