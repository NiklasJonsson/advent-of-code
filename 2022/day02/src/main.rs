#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn parse(sym: &str) -> Self {
        match sym {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid symbol {sym}"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl Shape {
    fn parse(sym: &str) -> Self {
        match sym {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Invalid symbol {sym}"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn parse_score(line: &str) -> u32 {
    let mut split = line.split_ascii_whitespace();
    let theirs = Shape::parse(split.next().expect("Missing first action"));
    let result = GameResult::parse(split.next().expect("Missing first action"));
    let mine = match (result, theirs) {
        (GameResult::Draw, _) => theirs,
        (GameResult::Win, Shape::Paper) => Shape::Scissors,
        (GameResult::Win, Shape::Rock) => Shape::Paper,
        (GameResult::Win, Shape::Scissors) => Shape::Rock,
        (GameResult::Loss, Shape::Paper) => Shape::Rock,
        (GameResult::Loss, Shape::Rock) => Shape::Scissors,
        (GameResult::Loss, Shape::Scissors) => Shape::Paper,
    };

    result.score() + mine.score()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let name = &args[0];
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;
    let mut total_score = 0;

    for line in contents.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }

        total_score += parse_score(line);
    }

    println!("Score: {total_score}");

    Ok(())
}
