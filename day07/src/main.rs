fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    let mut numbers: Vec<usize> = std::fs::read_to_string(&args[1])?
        .lines()
        .next()
        .expect("No line in file")
        .split_terminator(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let max = *numbers.iter().max().unwrap();
    let mut costs: Vec<usize> = vec![0; max];

    let cost = |start: usize, end: usize| -> usize {
        let diff = if start < end {
            end - start
        } else {
            start - end
        };

        (0..=diff).fold(0, |acc, n| acc + n)
    };

    for n in numbers {
        for i in 0..max {
            costs[i] += cost(n, i);
        }
    }

    let min = costs
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.cmp(b.1))
        .expect("No elements!");

    println!("pos: {}, fuel: {}", min.0, min.1);

    Ok(())
}
