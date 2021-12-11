#![feature(drain_filter)]

use std::num::ParseIntError;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct BitMask {
    m: u64,
}

impl BitMask {
    fn set(&mut self, i: u8, v: bool) {
        assert!(i < 64);
        let v = if v { 1 } else { 0 };
        self.m |= v * (1 << i);
    }

    fn get(&self, i: u8) -> bool {
        (self.m & (1 << i)) > 0
    }

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        Ok(Self {
            m: u64::from_str_radix(s, 2)?,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day02 <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;

    let mut gamma_counts = Vec::new();

    let add = |vec: &mut Vec<i32>, i: usize, v| {
        if i < vec.len() {
            vec[i] += v;
        } else {
            vec.push(v);
        }
    };

    let mut numbers = Vec::new();

    for l in contents.lines() {
        let l = l.trim();
        for (i, c) in l.char_indices() {
            let x = if c == '1' { 1 } else { -1 };
            add(&mut gamma_counts, i, x);
        }
        numbers.push(BitMask::from_str(l)?);
    }
    let mut gamma = BitMask { m: 0 };
    let l = gamma_counts.len();
    for (i, n) in gamma_counts.into_iter().rev().enumerate() {
        assert!(n != 0);
        gamma.set(i.try_into()?, n > 0);
    }

    let epsilon = !gamma.m & ((1 << l) - 1);
    println!("g * e = {} * {} = {}", gamma.m, epsilon, gamma.m * epsilon);

    let find = |v: &Vec<BitMask>, most_common: bool| {
        let mut rem = v.clone();
        let mut index = l as isize - 1;
        while rem.len() > 1 && index >= 0 {
            let i = index as u8;
            let count = rem
                .iter()
                .fold(0, |acc, e| acc + if e.get(i) { 1 } else { -1 });
            let should_be_set = if most_common { count >= 0 } else { count < 0 };

            rem.retain(|e| e.get(i) == should_be_set);
            index -= 1;
        }
        assert!(rem.len() == 1);
        rem[0]
    };

    let oxy = find(&numbers, true);
    let co2 = find(&numbers, false);
    println!("oxy * co2 = {} * {} = {}", oxy.m, co2.m, oxy.m * co2.m);

    Ok(())
}
