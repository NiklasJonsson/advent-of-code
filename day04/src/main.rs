#![feature(drain_filter)]
#![feature(bool_to_option)]

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct Board {
    numbers: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn rows(&self) -> impl Iterator<Item = (&[u32; 5], &[bool; 5])> {
        self.numbers.iter().zip(self.marked.iter())
    }

    fn cols(&self) -> impl Iterator<Item = ([u32; 5], [bool; 5])> + '_ {
        (0..5).map(|i| {
            (
                [
                    self.numbers[0][i],
                    self.numbers[1][i],
                    self.numbers[2][i],
                    self.numbers[3][i],
                    self.numbers[4][i],
                ],
                [
                    self.marked[0][i],
                    self.marked[1][i],
                    self.marked[2][i],
                    self.marked[3][i],
                    self.marked[4][i],
                ],
            )
        })
    }

    fn elems(&self) -> impl Iterator<Item = (&u32, &bool)> {
        self.rows()
            .flat_map(|(n_row, m_row)| n_row.iter().zip(m_row.iter()))
    }

    /// Returns if the boards has a bingo row
    fn mark(&mut self, number: u32) -> bool {
        for (n_row, m_row) in self.numbers.iter().zip(self.marked.iter_mut()) {
            for (n, m) in n_row.iter().zip(m_row.iter_mut()) {
                if *n == number {
                    *m = true;
                    break;
                }
            }
        }
        self.has_bingo()
    }

    fn score(&self) -> u32 {
        self.elems().filter_map(|(n, m)| (!m).then_some(n)).sum()
    }

    fn has_bingo(&self) -> bool {
        self.rows().any(|e| e.1.iter().all(|e| *e)) || self.cols().any(|e| e.1.iter().all(|e| *e))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day02 <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut lines = contents.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split_terminator(',')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<u32>, _>>()?;
    let mut boards = Vec::new();

    loop {
        match lines.next() {
            Some(l) => assert_eq!(l.trim(), ""),
            None => break,
        }
        let mut board = Board::default();
        for i in 0..5 {
            let l = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(str::parse::<u32>);
            for (j, n) in l.enumerate() {
                board.numbers[i][j] = n?;
            }
        }
        boards.push(board);
    }

    // part 1
    for n in &numbers {
        if let Some(board) = boards.iter_mut().find_map(|b| b.mark(*n).then_some(b)) {
            let s: u32 = board.score();
            println!("{}", s * n);
            break;
        }
    }

    // part 2
    for n in &numbers {
        if boards.len() == 1 {
            let bingo = boards[0].mark(*n);
            if bingo {
                let s: u32 = boards[0].score();
                println!("{}", s * n);
                break;
            }
            continue;
        }

        for b in &mut boards {
            b.mark(*n);
        }

        boards.retain(|b| !b.has_bingo());
    }

    Ok(())
}
