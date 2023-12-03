#![feature(str_split_whitespace_remainder)]

pub struct WindowsItr<I, const N: usize>
where
    I: Iterator,
{
    itr: I,
    first: bool,
    v: [I::Item; N],
}

impl<I, const N: usize> Iterator for WindowsItr<I, N>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = [I::Item; N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            // TODO: MaybeUninit
            let tmp = std::array::from_fn(|_| self.itr.next());
            if tmp.iter().any(Option::is_none) {
                return None;
            }

            self.v = tmp.map(Option::unwrap);
        } else {
            for i in 0..N - 1 {
                self.v.swap(i, i + 1);
            }
            self.v[N - 1] = self.itr.next()?;
        }
        Some(self.v)
    }
}

impl<I, const N: usize> WindowsItr<I, N>
where
    I: Iterator,
    I::Item: Default,
{
    pub fn new(i: I) -> Self {
        Self {
            itr: i,
            first: true,
            v: std::array::from_fn(|_| Default::default()),
        }
    }
}

// TODO: MaybeUninit
pub fn windows<I, const N: usize>(itr: I) -> WindowsItr<I, N>
where
    I: Iterator,
    I::Item: Default,
{
    WindowsItr {
        itr,
        first: true,
        v: std::array::from_fn(|_| Default::default()),
    }
}

pub fn parse_arg1() -> Result<String, String> {
    let mut args = std::env::args();
    let argc = args.len();
    let name = args.next().expect("No caller?");
    if argc != 2 {
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }
    Ok(args.next().expect("Just checked this!"))
}

pub struct Args {
    pub fname: String,
    pub do_part1: bool,
    pub do_part2: bool,
}

pub fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let argc = args.len();
    let name = args.next().expect("No caller?");
    if argc < 2 {
        println!("usage: {name} <file>");
        return Err("Error: expected file arg".into());
    }

    let mut do_part1 = false;
    let mut do_part2 = false;
    let mut fname = String::new();

    for a in args {
        if a == "--part1" {
            do_part1 = true;
        } else if a == "--part2" {
            do_part2 = true;
        } else {
            fname = a;
        }
    }

    if !do_part1 && !do_part2 {
        do_part1 = true;
        do_part2 = true;
    }
    Ok(Args {
        fname,
        do_part1,
        do_part2,
    })
}

/// Split `s` into `N` whitespace-separated words
pub fn split_whitespace_n<const N: usize>(s: &str) -> Option<[&str; N]> {
    let mut words = s.split_whitespace();
    let tmp = std::array::from_fn(|_| words.next());
    if tmp.iter().any(Option::is_none) {
        return None;
    }

    Some(tmp.map(Option::unwrap))
}

/// Take N whitespace-separated words from `s` and return the remainder.
pub fn take_n_words<const N: usize>(s: &str) -> Option<([&str; N], &str)> {
    let mut words = s.split_whitespace();
    let tmp = std::array::from_fn(|_| words.next());
    if tmp.iter().any(Option::is_none) {
        return None;
    }

    Some((tmp.map(Option::unwrap), words.remainder().unwrap_or("")))
}

/// Split `s` into lines, trimming whitespace and removing empty lines.
pub fn input_lines(s: &str) -> impl Iterator<Item = &str> {
    s.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let array = [1, 2, 3, 4, 5];
        let mut itr = windows::<_, 3>(array.into_iter());
        assert_eq!(itr.next(), Some([1, 2, 3]));
        assert_eq!(itr.next(), Some([2, 3, 4]));
        assert_eq!(itr.next(), Some([3, 4, 5]));
        assert_eq!(itr.next(), None);
    }
}
