use std::ops::Range;
use std::path::Path;

type Area = [Range<i32>; 2];

fn parse(s: &str) -> Area {
    let mut parts = s.split(':');
    parts.next().unwrap();
    let mut parts = parts.next().unwrap().split(',');

    let parse_range = |s: &str| {
        let s = s.trim();
        let s = &s[2..s.len()];
        let mut parts = s.split("..");
        Range {
            start: parts.next().unwrap().parse::<i32>().unwrap(),
            end: parts.next().unwrap().parse::<i32>().unwrap(),
        }
    };

    let xrange = parse_range(parts.next().unwrap());
    let yrange = parse_range(parts.next().unwrap());

    [xrange, yrange]
}

fn simulate(a: Area, mut vel: [i32; 2]) -> Vec<[i32; 2]> {
    let mut pos = [0, 0];

    let mut path = vec![pos];
    loop {
        pos[0] += vel[0];
        pos[1] += vel[1];
        path.push(pos);
        let x_diff = match vel[0] {
            0 => 0,
            x if x > 0 => -1,
            x if x < 0 => 1,
            _ => unreachable!(),
        };

        vel[0] += x_diff;
        vel[1] -= 1;

        if pos[0] <= a[0].end && pos[0] >= a[0].start && pos[1] <= a[1].end && pos[1] >= a[1].start
        {
            break;
        } else if (pos[0] > a[0].end && vel[0] > 0) || pos[1] < a[1].start {
            break;
        }
    }

    path
}

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let target = parse(&contents);
    println!("PART 1 sum: {:?}", &target);
    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    println!("PART 2 result: {}", 1);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    part1(&args[1])?;
    part2(&args[1])?;

    Ok(())
}
