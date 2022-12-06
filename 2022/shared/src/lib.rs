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
