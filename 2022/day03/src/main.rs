fn prio(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        return c as u32 - 'a' as u32 + 1;
    }


    if c >= 'A' && c <= 'Z' {
        return c as u32 - 'A' as u32 + 27;
    }

    unimplemented!("{}", c);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut sum = 0;

    for line in contents.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }

        let len = line.len();
        assert!(len % 2 == 0);
        let first: &str = &line[..len/2];
        let second: &str = &line[len/2..];
        let first: std::collections::HashSet<char> = std::collections::HashSet::from_iter(first.chars());
        for f in first {
            if second.contains(f) {
                sum += prio(f);
            }
        }

    }

    println!("Answer: {sum}");

    Ok(())
}
