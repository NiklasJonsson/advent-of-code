fn part1(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    for line in shared::input_lines(contents) {
        let ([_game, id], rem) = shared::take_n_words::<2>(line).unwrap();
        let id = id
            .strip_suffix(':')
            .expect("No : suffic for game id")
            .parse::<u32>()
            .expect("Failed to parse game id");

        let mut valid = true;
        for round in rem.split(';') {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;
            for balls in round.split(',') {
                let [count, color] = shared::split_whitespace_n::<2>(balls).unwrap();
                let v = match color {
                    "blue" => &mut b,
                    "red" => &mut r,
                    "green" => &mut g,
                    _ => unreachable!(),
                };
                *v += count.parse::<u32>().unwrap();
            }

            if r > max_r || g > max_g || b > max_b {
                println!("Game {id} is not valid due to round {round}");
                valid = false;
            }
        }
        if valid {
            dbg!("Valid: ", id);
            sum += id;
        }
    }

    Ok(sum)
}

fn part2(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    for line in shared::input_lines(contents) {
        let ([_game, _id], rem) = shared::take_n_words::<2>(line).unwrap();
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        for round in rem.split(';') {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;
            for balls in round.split(',') {
                let [count, color] = shared::split_whitespace_n::<2>(balls).unwrap();
                let v = match color {
                    "blue" => &mut b,
                    "red" => &mut r,
                    "green" => &mut g,
                    _ => unreachable!(),
                };
                *v += count.parse::<u32>().expect("Failed to parse ball count");
            }

            max_r = std::cmp::max(r, max_r);
            max_g = std::cmp::max(g, max_g);
            max_b = std::cmp::max(b, max_b);
        }

        let power = max_r * max_g * max_b;
        sum += power;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared::Args {
        fname,
        do_part1,
        do_part2,
    } = shared::parse_args()?;
    let contents = std::fs::read_to_string(fname)?;

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
