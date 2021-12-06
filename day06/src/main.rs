fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day06 <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut ages = [0usize; 9];

    for n in contents
        .lines()
        .next()
        .expect("Expected atleast one line")
        .split_terminator(",")
        .map(|l| l.parse::<usize>())
    {
        ages[n?] += 1;
    }

    for _ in 0..256 {
        let done = ages[0];
        for i in 0..ages.len() - 1 {
            ages[i] = ages[i + 1];
        }
        ages[6] += done;
        ages[8] = done;
    }

    println!("count: {}", ages.iter().sum::<usize>());

    Ok(())
}