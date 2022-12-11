use std::collections::HashSet;

type Pos = [usize; 2];
type Visited = HashSet<Pos>;

fn apply_move<const N: usize>(visited: &mut Visited, rope: &mut [Pos; N], dir: &str, count: usize) {
    let (idx, ops) = match dir {
        "R" => (0, std::ops::Add::add as fn(usize, usize) -> usize),
        "L" => (0, std::ops::Sub::sub as fn(usize, usize) -> usize),
        "U" => (1, std::ops::Add::add as fn(usize, usize) -> usize),
        "D" => (1, std::ops::Sub::sub as fn(usize, usize) -> usize),
        _ => unreachable!(),
    };

    for _ in 0..count {
        rope[0][idx] = ops(rope[0][idx], 1);

        for i in 1..rope.len() {
            let prev = rope[i - 1];
            let cur = &mut rope[i];
            let diff_y = prev[0] as isize - cur[0] as isize;
            let diff_x = prev[1] as isize - cur[1] as isize;

            if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
                // Adjacent
                continue;
            }

            cur[0] = (cur[0] as isize + diff_y.signum()) as usize;
            cur[1] = (cur[1] as isize + diff_x.signum()) as usize;
        }

        visited.insert(*rope.last().unwrap());
    }
}

fn part1() -> Result<usize, Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;
    let itr = contents.lines().map(str::trim).filter(|l| !l.is_empty());

    let mut rope = [[0, 0]; 2];
    let mut visited: Visited = HashSet::new();
    visited.insert([0, 0]);

    for line in itr {
        let [dir, count] = shared::split_whitespace_n(line).unwrap();
        let count: usize = count.parse()?;
        apply_move(&mut visited, &mut rope, dir, count);
    }
    Ok(visited.len())
}

fn part2() -> Result<usize, Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;
    let itr = contents.lines().map(str::trim).filter(|l| !l.is_empty());

    let mut rope = [[0, 0]; 10];
    let mut visited: Visited = HashSet::new();
    visited.insert([0, 0]);

    for line in itr {
        let [dir, count] = shared::split_whitespace_n(line).unwrap();
        let count: usize = count.parse()?;
        apply_move(&mut visited, &mut rope, dir, count);
    }
    Ok(visited.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = part1()?;
    println!("Part 1: {part1}");

    let part2 = part2()?;
    println!("Part 2: {part2}");

    Ok(())
}
