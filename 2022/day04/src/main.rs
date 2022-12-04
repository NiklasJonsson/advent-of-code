fn parse_range(s: &str) -> [u32; 2] {
    let mut split = s.split('-');
    let first = split.next().unwrap();
    let second = split.next().unwrap();

    [
        first.parse::<u32>().unwrap(),
        second.parse::<u32>().unwrap(),
    ]
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

    let itr = contents.lines().map(str::trim).filter(|s| !s.is_empty());

    for line in itr {
        let mut split = line.split(',');
        let first = parse_range(split.next().unwrap());
        let second = parse_range(split.next().unwrap());

        if first[0] <= second[0] && first[1] >= second[1] {
            sum += 1;
        } else if second[0] <= first[0] && second[1] >= first[1] {
            sum += 1;
        }
    }

    println!("Answer: {sum}");

    Ok(())
}
