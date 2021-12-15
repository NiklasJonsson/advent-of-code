#![feature(bool_to_option)]

const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut heights = Vec::new();
    let mut x_max: isize = 0;
    let mut y_max: isize = 0;

    for line in contents.lines() {
        let line = line.trim();
        assert!(line.len() as isize == x_max || x_max == 0);
        x_max = line.len() as isize;
        for c in line.chars() {
            assert!(c.is_numeric());
            heights.push(c.to_digit(10).expect("Failed to parse int"));
            assert!(*heights.last().unwrap() < 10);
        }
        y_max += 1;
    }

    let neighbours = |x: isize, y: isize| {
        NEIGHBOURS.iter().filter_map(move |(x_offset, y_offset)| {
            let x_abs = x + x_offset;
            let y_abs = y + y_offset;
            (x_abs >= 0 && x_abs < x_max && y_abs >= 0 && y_abs < y_max).then_some((x_abs, y_abs))
        })
    };

    let sample_height = |x: isize, y: isize| -> u32 {
        assert!(x >= 0 && x < x_max);
        assert!(y >= 0 && y < y_max);
        heights[x as usize + (y * x_max) as usize]
    };

    let find_basin = |x: isize, y: isize| -> Vec<(isize, isize)> {
        let mut basin = Vec::new();
        let mut stack = vec![(x, y)];
        while let Some(pos) = stack.pop() {
            basin.push(pos);
            for n in
                neighbours(pos.0, pos.1).filter(|&(x_abs, y_abs)| sample_height(x_abs, y_abs) != 9)
            {
                if !basin.contains(&n) && !stack.contains(&n) {
                    stack.push(n);
                }
            }
        }

        basin
    };

    let mut all_basins: std::collections::HashSet<(isize, isize)> =
        std::collections::HashSet::new();

    let mut maxes = [0; 3];

    for y in 0..y_max {
        for x in 0..x_max {
            let h_mid = sample_height(x, y);
            let is_bottom = neighbours(x, y)
                .map(|(x_abs, y_abs)| sample_height(x_abs, y_abs) > h_mid)
                .all(std::convert::identity);
            if is_bottom {
                let basin = find_basin(x, y);
                let l = basin.len();
                println!("{:?}", &basin);
                let min = maxes.iter_mut().min();
                if let Some(min) = min {
                    if *min < l {
                        *min = l;
                    }
                }
                for b in &basin {
                    assert!(!all_basins.contains(b));
                }
                all_basins.extend(basin.into_iter());
            }
        }
    }

    for y in 0..y_max {
        for x in 0..x_max {
            if all_basins.contains(&(x, y)) {
                print!("{}", sample_height(x, y));
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!(
        "maxes: {:?}, product: {} ",
        maxes,
        maxes.iter().product::<usize>()
    );

    Ok(())
}
