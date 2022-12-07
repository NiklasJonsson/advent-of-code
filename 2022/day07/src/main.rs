use std::collections::HashMap;

#[derive(Debug)]
enum Entry {
    Dir(String),
    File { _name: String, size: usize },
}

#[derive(Default, Debug)]
struct FileSystem {
    dir_contents: HashMap<String, Vec<Entry>>,
}

impl FileSystem {
    fn add_file(&mut self, dir: String, filename: &str, size: usize) {
        self.dir_contents
            .entry(dir)
            .or_insert_with(Vec::new)
            .push(Entry::File {
                _name: filename.to_string(),
                size,
            });
    }

    fn add_dir(&mut self, parent: String, child: &str) {
        let child = format!("{parent}/{child}");
        self.dir_contents
            .entry(parent)
            .or_insert_with(Vec::new)
            .push(Entry::Dir(child));
    }
}

fn parse_filesystem_log(log: &str) -> FileSystem {
    let mut dir_stack: Vec<String> = Vec::new();

    let itr = log.lines().map(str::trim).filter(|s| !s.is_empty());

    let mut fs = FileSystem::default();

    for line in itr {
        let mut split = line.split_whitespace();
        let first = split.next().expect("No empty lines expected");
        if first == "$" {
            let cmd = split.next().unwrap();
            if cmd == "cd" {
                match split.next() {
                    Some("..") => {
                        dir_stack.pop();
                    }
                    Some(dir) => dir_stack.push(dir.to_string()),
                    None => panic!("Expected arg for cd command"),
                };
            } else {
                assert_eq!(cmd, "ls", "Unexpected command");
            }
        } else if first == "dir" {
            let dirname = split.next().unwrap();
            fs.add_dir(dir_stack.join("/"), dirname);
        } else {
            let size = first.parse::<usize>().expect("Failed to parse file size");
            let filename = split.next().unwrap();
            fs.add_file(dir_stack.join("/"), filename, size);
        }
    }

    fs
}

fn compute_dirsizes(fs: &FileSystem) -> HashMap<&str, usize> {
    let mut dir_size: HashMap<&str, usize> = HashMap::default();
    let mut queue: Vec<&str> = fs.dir_contents.keys().map(String::as_str).collect();
    while let Some(cur) = queue.pop() {
        let mut done = true;
        let mut sum = 0;
        for e in fs.dir_contents[cur].iter() {
            if let Some(size) = match e {
                Entry::File { size, .. } => Some(size),
                Entry::Dir(dirname) => dir_size.get(dirname.as_str()),
            } {
                sum += size;
            } else {
                done = false;
            }
        }

        if !done {
            queue.insert(0, cur);
        } else {
            let prev = dir_size.insert(cur, sum);
            assert!(prev.is_none());
        }
    }

    dir_size
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fname = shared::parse_arg1()?;
    let contents = std::fs::read_to_string(&fname)?;

    let fs = parse_filesystem_log(&contents);
    let dir_sizes = compute_dirsizes(&fs);

    let mut sum = 0;
    for (k, &v) in dir_sizes.iter() {
        if v < 100000 {
            sum += v;
        }
    }
    println!("Part 1 Answer: {sum}");

    let max = 70000000;
    let required_free = 30000000;
    let used = dir_sizes.get("/").unwrap();
    println!("{used}");

    let mut found = usize::MAX;
    for (_, &v) in dir_sizes.iter() {
        if (max - used + v) > required_free && v < found {
            found = v;
        }
    }
    println!("Part 2 Answer: {found}");

    Ok(())
}
