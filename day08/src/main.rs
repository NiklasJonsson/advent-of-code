fn to_unique(s: &str) -> Option<usize> {
    match s.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn str_to_bitmask(s: &str) -> u8 {
    debug_assert!(s.len() < 8);

    let mut ret = 0;
    for c in s.chars() {
        let i = "abcdefg".chars().position(|x| x == c).unwrap();
        ret |= 1 << i;
    }

    ret
}

fn count_overlap(a: u8, b: u8) -> u32 {
    (a & b).count_ones()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let start = std::time::Instant::now();
    let mut sum = 0;
    for line in contents.lines() {
        let mut parts = line.split_terminator('|');
        let mut signals = parts.next().unwrap().split_ascii_whitespace();
        let signals = [
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
            signals.next().unwrap(),
        ];

        // Number to bitmask repr for this line
        let mut mapping = [0; 10];
        let mut rev_mapping = [0; 256];

        // Find mappings for uniques
        for sig in &signals {
            if let Some(unique) = to_unique(*sig) {
                let mask = str_to_bitmask(sig);
                mapping[unique] = mask;
                rev_mapping[mask as usize] = unique;
            }
        }

        // Compute unknowns with rules for which of the other masks it must overlap
        for sig in &signals {
            let mask = str_to_bitmask(sig);
            let len = sig.len();
            if len == 5 {
                // 2, 3, 5
                if count_overlap(mask, mapping[1]) == 2 {
                    debug_assert_eq!(count_overlap(mask, mapping[4]), 3);
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 3);
                    mapping[3] = mask;
                    rev_mapping[mask as usize] = 3;
                } else if count_overlap(mask, mapping[4]) == 3 {
                    debug_assert_eq!(count_overlap(mask, mapping[1]), 1);
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 2);
                    mapping[5] = mask;
                    rev_mapping[mask as usize] = 5;
                } else {
                    debug_assert_eq!(count_overlap(mask, mapping[1]), 1);
                    debug_assert_eq!(count_overlap(mask, mapping[4]), 2);
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 2);
                    mapping[2] = mask;
                    rev_mapping[mask as usize] = 2;
                }
            } else if len == 6 {
                // 0, 6, 9
                if count_overlap(mask, mapping[1]) == 2 && count_overlap(mask, mapping[4]) == 3 {
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 3);
                    mapping[0] = mask;
                    rev_mapping[mask as usize] = 0;
                } else if count_overlap(mask, mapping[1]) == 1
                    && count_overlap(mask, mapping[4]) == 3
                {
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 2);
                    mapping[6] = mask;
                    rev_mapping[mask as usize] = 6;
                } else {
                    debug_assert_eq!(count_overlap(mask, mapping[1]), 2);
                    debug_assert_eq!(count_overlap(mask, mapping[4]), 4);
                    debug_assert_eq!(count_overlap(mask, mapping[7]), 3);
                    mapping[9] = mask;
                    rev_mapping[mask as usize] = 9;
                }
            } else {
                debug_assert!(to_unique(sig).is_some());
            }
        }

        debug_assert!(mapping.iter().all(|x| *x > 0), "Missing mapping!");

        sum += parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|number_str| {
                let mask = str_to_bitmask(number_str);
                rev_mapping[mask as usize]
            })
            .fold(0, |acc, n| acc * 10 + n);
    }
    let end = std::time::Instant::now();
    println!("micro seconds: {}", (end - start).as_micros());
    // 1068933
    println!("sum: {}", sum);

    Ok(())
}
