use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
struct ParsedNumber {
    start: usize,
    value: u32,
}

struct Data {
    input: Vec<char>,
    numbers: HashMap<usize, ParsedNumber>,
    width: usize,
}

fn preprocess(contents: &str) -> Data {
    let width = shared::input_lines(contents).next().unwrap().len();

    let mut numbers: HashMap<usize, ParsedNumber> = HashMap::new();
    let mut data: Vec<char> = Vec::new();

    let mut buf = String::new();

    let mut parse_num_buffer = |buf: &mut String, end: usize| {
        assert!(end >= buf.len());
        let value = buf.parse::<u32>().expect("Failed to parse part number");
        let start = end - buf.len();
        for i in 0..buf.len() {
            let pos = start + i;
            let n = ParsedNumber { start, value };
            println!("Inserting {pos} -> {n:?} (lineno: {})", (start / width) + 1);
            numbers.insert(pos, n);
        }

        buf.clear();
    };

    for (y, line) in shared::input_lines(contents).enumerate() {
        for (x, c) in line.char_indices() {
            if c.is_ascii_digit() {
                buf.push(c);
            } else if !buf.is_empty() {
                let end = x + y * width;
                parse_num_buffer(&mut buf, end);
            }
            data.push(c);
        }
        if !buf.is_empty() {
            parse_num_buffer(&mut buf, y * width + width);
        }
    }

    Data {
        input: data,
        numbers,
        width,
    }
}

fn part1(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    let Data {
        input,
        numbers,
        width,
    } = preprocess(contents);

    let mut unique_numbers = HashSet::new();
    for idx in 0..input.len() {
        let c = input[idx];
        if c == '.' || c.is_ascii_digit() {
            continue;
        }

        let x = idx % width;
        let y = idx / width;

        println!("Symbol {c} at ({x}, {y}, lineno: {})", y + 1);
        for y_i in [-1, 0, 1] {
            for x_i in [-1, 0, 1] {
                let cand_x: isize = x_i + x as isize;
                let cand_y: isize = y_i + y as isize;
                let cand_i = cand_x + cand_y * width as isize;
                if cand_i < 0 || cand_i as usize >= input.len() {
                    continue;
                }

                if let Some(n) = numbers.get(&(cand_i as usize)) {
                    println!("Found {n:?} ({cand_x}, {cand_y}) (lineno: {})", cand_y + 1);
                    unique_numbers.insert(n);
                }
            }
        }
    }

    for n in unique_numbers {
        sum += n.value;
    }
    Ok(sum)
}

const GEAR: char = '*';

fn part2(contents: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    let Data {
        input,
        numbers,
        width,
    } = preprocess(contents);

    let mut unique_numbers = HashSet::new();
    for idx in 0..input.len() {
        let c = input[idx];
        if c != GEAR {
            continue;
        }

        let x = idx % width;
        let y = idx / width;

        println!("Gear candidate at ({x}, {y}, lineno: {})", y + 1);
        unique_numbers.clear();
        for x_i in [-1, 0, 1] {
            for y_i in [-1, 0, 1] {
                let cand_x: isize = x_i + x as isize;
                let cand_y: isize = y_i + y as isize;
                let cand_i = cand_x + cand_y * width as isize;
                if cand_i < 0 || cand_i as usize >= input.len() {
                    continue;
                }

                if let Some(n) = numbers.get(&(cand_i as usize)) {
                    unique_numbers.insert(n);
                }
            }
        }

        if unique_numbers.len() == 2 {
            let ratio = unique_numbers.iter().map(|n| n.value).product::<u32>();
            println!("Found {unique_numbers:?} ratio: {ratio}");
            sum += ratio;
        }
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
