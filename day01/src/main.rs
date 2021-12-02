struct WindowsItr<I>
where
    I: Iterator,
{
    itr: I,
    first: bool,
    v: [I::Item; 3],
}

impl<I> Iterator for WindowsItr<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = [I::Item; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            self.v = [self.itr.next()?, self.itr.next()?, self.itr.next()?];
        } else {
            self.v.swap(0, 1);
            self.v.swap(1, 2);
            self.v[2] = self.itr.next()?;
        }
        Some(self.v)
    }
}

impl<I> WindowsItr<I>
where
    I: Iterator,
    I::Item: Default,
{
    fn new(i: I) -> Self {
        Self {
            itr: i,
            first: true,
            v: [Default::default(), Default::default(), Default::default()],
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    assert!(args.len() > 1);
    let path = std::path::PathBuf::from(args[1].clone());

    let contents = std::fs::read_to_string(path).expect("Failed to read file");

    let mut prev: Option<usize> = None;
    let mut count: usize = 0;
    let windows = WindowsItr::new(contents.lines());
    for window in windows {
        let cur = window
            .iter()
            .map(|l| l.parse::<usize>().expect("failed to parse line"))
            .sum();
        match prev {
            Some(prev) if cur > prev => {
                count += 1;
            }
            _ => (),
        }
        prev = Some(cur);
    }

    println!("{}", count);
}
