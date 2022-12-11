struct Machine {
    register: i32,
    record: Vec<i32>,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            register: 1,
            record: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    AddX(i32),
    Noop,
}

impl Machine {
    fn cycle(&mut self) {
        self.record.push(self.register);
    }
    pub fn exec(&mut self, op: Op) {
        match op {
            Op::Noop => {
                self.cycle();
            }
            Op::AddX(v) => {
                self.cycle();
                self.cycle();
                self.register += v;
            }
        }
    }

    fn value_at(&self, cycle: usize) -> i32 {
        assert!(cycle > 0);
        self.record[cycle - 1]
    }
}

fn part1(contents: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = contents.lines().map(str::trim).filter(|l| !l.is_empty());

    let mut machine = Machine::default();

    for line in lines {
        let mut split = line.split_whitespace();
        let cmd = split.next().unwrap();
        match cmd {
            "noop" => machine.exec(Op::Noop),
            "addx" => {
                let v = split
                    .next()
                    .expect("Missing arg for addx")
                    .parse::<i32>()
                    .expect("Failed to parse addx arg");
                machine.exec(Op::AddX(v));
            }
            _ => unreachable!(),
        }
    }

    let cycles = [20, 60, 100, 140, 180, 220];

    let sum = cycles
        .iter()
        .map(|c| machine.value_at(*c) * *c as i32)
        .sum();
    Ok(sum)
}

fn part2(contents: &str) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(0)
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
