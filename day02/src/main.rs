// Top-left is origin
struct State {
    y: usize,
    x: usize,
    aim: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day02 <file>");
        return Err("Error: expected file arg".into());
    }

    let path = std::path::PathBuf::from(args[1].clone());

    let contents = std::fs::read_to_string(path)?;

    let mut state = State { y: 0, x: 0, aim: 0 };

    for l in contents.lines() {
        let mut itr = l.split_whitespace();
        let dir = itr.next().expect("Missing direction");
        let count: usize = itr.next().expect("Missing count").parse()?;
        match dir {
            "forward" => {
                state.x += count;
                state.y += state.aim * count;
            }
            "down" => state.aim += count,
            "up" => state.aim -= count,
            x => unreachable!("Invalid command {}", x),
        };
    }

    println!(
        "down * forward = {} * {} = {}",
        state.y,
        state.x,
        state.y * state.x
    );

    Ok(())
}
