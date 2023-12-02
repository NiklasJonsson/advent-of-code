fn part1(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    for line in contents.split('\n') {
        if line == "" {
            continue;
        }

        let mut first = None;
        let mut last = None;

        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                if first.is_none() {
                    first = Some(d);
                }
                last = Some(d);
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }

    Ok(sum)
}

const NUMBERS: [(&'static str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eigth", 8),
    ("nine", 9),
];

fn part2(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    let mut buf = String::new();

    for line in contents.split('\n') {
        if line == "" {
            continue;
        }

        let mut first = None;
        let mut last = None;

        for c in line.chars() {
            let mut d = None;

            if let Some(v) = c.to_digit(10) {
                d = Some(v);
            } else {
                buf.push(c);
                for n in NUMBERS {
                    if buf.ends_with(n.0) {
                        buf.clear();
                        d = Some(n.1);
                    }
                }
            };

            let Some(d) = d else {
                continue;
            };

            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
        }
        let i = first.unwrap() * 10 + last.unwrap();
        dbg!(line, first, last, i);

        sum += i;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;

    let part1 = part1(&contents)?;
    println!("Part 1: {part1}");

    let part2 = part2(&contents)?;
    println!("Part 2: {part2}");

    Ok(())
}
