// Top-left is origin
struct State {
    x: usize,
    y: usize,
    aim: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: day02 <file>");
        return Err("Error: expected file arg".into());
    }

    let contents = std::fs::read_to_string(&args[1])?;

    let mut state = State { x: 0, y: 0, aim: 0 };

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
