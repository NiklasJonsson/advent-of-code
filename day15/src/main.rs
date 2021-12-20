#![feature(bool_to_option)]

use std::path::Path;
const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

#[derive(Default, Debug, PartialEq, Eq)]
struct Node {
    pos: [usize; 2],
    cost: usize,
}

impl Node {
    fn new(pos: [usize; 2], cost: usize) -> Self {
        Self { pos, cost }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.pos.cmp(&other.pos)),
        )
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(contents: &str) -> (Vec<usize>, (usize, usize)) {
    let mut graph = Vec::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for (row, line) in contents.lines().enumerate() {
        for (col, v) in line.chars().enumerate() {
            max_y = std::cmp::max(max_y, row);
            max_x = std::cmp::max(max_x, col);
            graph.push(v.to_digit(10).unwrap() as usize);
        }
    }

    let width = max_x + 1;
    let height = max_y + 1;
    assert_eq!(width * height, graph.len());

    (graph, (width, height))
}

fn dijkstra_min_cost(graph: &[usize], width: usize, height: usize) -> Option<usize> {
    let idx = |p: [usize; 2]| p[0] + p[1] * width;

    let neighbours = |[x, y]: [usize; 2]| {
        NEIGHBOURS.iter().filter_map(move |(x_offset, y_offset)| {
            let x_abs = x as isize + x_offset;
            let y_abs = y as isize + y_offset;
            (x_abs >= 0 && x_abs < width as isize && y_abs >= 0 && y_abs < height as isize)
                .then_some([x_abs as usize, y_abs as usize])
        })
    };

    let mut visited = vec![false; graph.len()];
    let mut costs = vec![usize::MAX; graph.len()];
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Node::new([0, 0], 0));
    while let Some(Node { pos, cost }) = heap.pop() {
        if pos == [width - 1, height - 1] {
            return Some(cost);
        }

        let cur_idx = idx(pos) as usize;

        // Handle duplicates
        if visited[cur_idx] {
            continue;
        }

        visited[cur_idx] = true;

        for n in neighbours(pos).filter(|&n| !visited[idx(n)]) {
            let i = idx(n);
            let new_cost = cost + graph[i];
            if new_cost < costs[i] {
                costs[i] = new_cost;
                // This might put duplicates into the heap, this is fine
                heap.push(Node::new(n, new_cost));
            }
        }
    }

    None
}

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let (graph, (width, height)) = parse(&contents);
    let final_cost = dijkstra_min_cost(&graph, width, height).unwrap();
    println!("PART 1 c: {}", final_cost);
    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;

    let (graph, (width, height)) = parse(&contents);

    let m = 5;
    let new_width = width * m;
    let new_height = height * m;
    let mut new_graph = vec![usize::MAX; new_height * new_width];
    for y in 0..height {
        for x in 0..width {
            for ym in 0..m {
                for xm in 0..m {
                    let i = (xm * width + x) + (ym * height + y) * new_width;
                    let mut new = xm + ym + graph[x + y * width];
                    if new > 9 {
                        new -= 9;
                    };
                    new_graph[i] = new;
                }
            }
        }
    }

    debug_assert!(new_graph.iter().all(|&e| e != usize::MAX));

    let final_cost = dijkstra_min_cost(&new_graph, new_width, new_height).unwrap();
    println!("PART 2 c: {}", final_cost);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    part1(&args[1])?;
    part2(&args[1])?;

    Ok(())
}
