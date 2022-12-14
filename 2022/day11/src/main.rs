struct Monkey {
    items: Vec<usize>,
    op: Box<dyn FnMut(usize) -> usize>,
    test: Box<dyn FnMut(usize) -> usize>,
}

#[derive(Clone, Copy, Debug)]
enum Factor {
    Lit(usize),
    Old,
}

impl Factor {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Self::Old,
            n => Self::Lit(n.parse::<usize>().expect("Failed to parse literal")),
        }
    }
}

fn parse(contents: &str) -> Vec<Monkey> {
    let mut lines = contents.lines().map(str::trim).filter(|l| !l.is_empty());

    let mut out = Vec::new();

    while let Some(line) = lines.next() {
        let [monkey, _idx] = shared::split_whitespace_n(line).unwrap();
        assert_eq!(monkey, "Monkey");

        let items_line = lines.next().expect("Missing starting items line");
        let ([_starting, _items], items) = shared::take_n_words(items_line).unwrap();
        let items: Vec<usize> = items
            .split(',')
            .map(|e| e.trim().parse::<usize>().unwrap())
            .collect();

        let op_line = lines.next().expect("Missing op line");
        let [_operation, _new, _eq, f0, op, f1] =
            shared::split_whitespace_n(op_line).expect("Failed to parse opline");
        let (f0, f1) = (Factor::parse(f0), Factor::parse(f1));
        let bin_op = match op {
            "+" => std::ops::Add::add as fn(usize, usize) -> usize,
            "*" => std::ops::Mul::mul as fn(usize, usize) -> usize,
            _ => unreachable!("Unsupported op!"),
        };

        let op = move |old: usize| -> usize {
            match (f0, f1) {
                (Factor::Old, Factor::Old) => bin_op(old, old),
                (Factor::Lit(f), Factor::Old) => bin_op(f, old),
                (Factor::Lit(fa), Factor::Lit(fb)) => bin_op(fa, fb),
                (Factor::Old, Factor::Lit(f)) => bin_op(old, f),
            }
        };

        let test_line = lines.next().expect("Missing test line");
        let [_test, _divisible, _by, num] = shared::split_whitespace_n(test_line).unwrap();
        let num = num.parse::<usize>().expect("Failed to parse div number");

        let true_line = lines.next().expect("Missing true line");
        let [_if, _true, _throw, _to, _monkey, idx] =
            shared::split_whitespace_n(true_line).unwrap();
        let true_idx = idx.parse::<usize>().expect("Failed to parse true monkey");

        let false_line = lines.next().expect("Missing test line");
        let [_if, _true, _throw, _to, _monkey, idx] =
            shared::split_whitespace_n(false_line).unwrap();
        let false_idx = idx.parse::<usize>().expect("Failed to parse false monkey");
        let test = move |worry: usize| -> usize {
            if worry % num == 0 {
                true_idx
            } else {
                false_idx
            }
        };

        out.push(Monkey {
            items,
            op: Box::new(op),
            test: Box::new(test),
        });
    }

    out
}


fn part1(contents: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut monkeys = parse(contents);

    let mut records = vec![0; monkeys.len()];

    for _i in 0..20 {
        for (mi, record) in (0..monkeys.len()).zip(records.iter_mut()) {
            let items = std::mem::take(&mut monkeys[mi].items);
            for item in items {
                *record += 1;
                let mut lvl = item;
                lvl = (monkeys[mi].op)(lvl);
                lvl /= 3;
                let dst = (monkeys[mi].test)(lvl);
                monkeys[dst].items.push(lvl);
            }
        }
    }

    records.sort();
    let mut last_itr = records.iter().rev();
    let last = last_itr.next().unwrap();
    let last2 = last_itr.next().unwrap();

    Ok(last * last2)
}

fn part2(contents: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut monkeys = parse(contents);

    let mut records = vec![0; monkeys.len()];

    for _i in 0..20 {
        for (mi, record) in (0..monkeys.len()).zip(records.iter_mut()) {
            let items = std::mem::take(&mut monkeys[mi].items);
            for item in items {
                *record += 1;
                let mut lvl = item;
                lvl = (monkeys[mi].op)(lvl);
                let dst = (monkeys[mi].test)(lvl);
                monkeys[dst].items.push(lvl);
            }
        }
    }

    records.sort();
    let mut last_itr = records.iter().rev();
    let last = last_itr.next().unwrap();
    let last2 = last_itr.next().unwrap();

    Ok(last * last2)
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
