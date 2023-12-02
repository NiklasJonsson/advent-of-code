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
    ("eight", 8),
    ("nine", 9),
];

fn part2(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    for line in contents.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut first = None;
        let mut last = None;

        if line == "threegzbn1four4hbqkmtmoneightrhg" {
            dbg!("Now!");
        }

        for (i, c) in line.char_indices() {
            let mut d = None;

            if let Some(v) = c.to_digit(10) {
                d = Some(v);
            } else {
                for n in NUMBERS {
                    if line[0..i + 1].ends_with(n.0) {
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
    let shared::Args {
        fname,
        do_part1,
        do_part2,
    } = shared::parse_args()?;
    let contents = std::fs::read_to_string(&fname)?;

    if do_part1 {
        let part1 = part1(&contents)?;
        println!("Part 1: {part1}");
    }

    if do_part2 {
        let part2 = part2(&contents)?;
        println!("Part 2: {part2}");
    }

    Ok(())
}
