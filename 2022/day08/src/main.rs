fn part1() -> Result<u32, Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;

    let itr = contents.lines().map(str::trim).filter(|l| !l.is_empty());
    let mut heightfield = Vec::new();

    for (_y, line) in itr.enumerate() {
        heightfield.push(Vec::new());
        for (_x, ch) in line.chars().enumerate() {
            let h = ch.to_digit(10).unwrap();
            heightfield.last_mut().unwrap().push(h);
        }
    }

    let mut sum = 0;
    for (y, row) in heightfield.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            let mut blocked = row[..x].iter().any(|e| e >= h);
            blocked &= row[x + 1..].iter().any(|e| e >= h);
            blocked &= (0..y).any(|yi| heightfield[yi][x] >= *h);
            blocked &= (y + 1..heightfield.len()).any(|yi| heightfield[yi][x] >= *h);

            if x == 0 || y == 0 || !blocked {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

fn scenic_score(height: u32, trees: impl Iterator<Item = u32>) -> u32 {
    let mut sum = 0;
    for tree in trees {
        sum += 1;
        if tree >= height {
            break;
        }
    }

    sum
}

fn part2() -> Result<u32, Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;

    let itr = contents.lines().map(str::trim).filter(|l| !l.is_empty());
    let mut heightfield = Vec::new();

    for (_y, line) in itr.enumerate() {
        heightfield.push(Vec::new());
        for (_x, ch) in line.chars().enumerate() {
            let h = ch.to_digit(10).unwrap();
            heightfield.last_mut().unwrap().push(h);
        }
    }

    let mut max = 0;
    for (y, row) in heightfield.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            let mut score = [0; 4];
            score[0] = scenic_score(h, row[..x].iter().rev().cloned());
            score[1] = scenic_score(h, row[x + 1..].iter().cloned());
            score[2] = scenic_score(h, (0..y).map(|yi| heightfield[yi][x]).rev());
            score[3] = scenic_score(h, (y + 1..heightfield.len()).map(|yi| heightfield[yi][x]));
            max = std::cmp::max(max, score.iter().product());
        }
    }

    Ok(max)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = part1()?;
    println!("Part 1: {part1}");

    let part2 = part2()?;
    println!("Part 2: {part2}");

    Ok(())
}
