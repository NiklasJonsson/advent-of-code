use shared;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let itr = contents.lines().map(str::trim).filter(|s| !s.is_empty());

    const N_CHARS_MARKER: usize = 14;

    let mut index = None;
    for line in itr {
        for (idx, window) in shared::windows::<_, N_CHARS_MARKER>(line.chars()).enumerate() {
            let unique = (0..window.len() - 1).all(|i| !window[i + 1..].contains(&window[i]));
            if unique {
                index = Some(idx + window.len());
                break;
            }
        }
        if index.is_some() {
            println!("Answer: {index:?}");
        }
    }

    Ok(())
}
