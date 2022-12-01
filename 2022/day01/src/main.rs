fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut calorie_counts = Vec::new();
    let mut cur = 0;

    for line in contents.lines().map(str::trim) {
        if line.is_empty() {
            calorie_counts.push(cur);
            cur = 0;
            continue;
        }

        let calories: u32 = line.parse().expect("Failed to parse calories");
        cur += calories;
    }

    calorie_counts.sort_by_key(|e| u32::MAX - e);

    let top: u32 = calorie_counts.iter().take(3).sum();

    println!("Max calories: {top:?}");

    Ok(())
}
