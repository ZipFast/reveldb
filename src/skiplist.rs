use crossbeam::atomic::AtomicCell;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::random::Random;
const K_MAX_HEIGHT: u32 = 8;

#[derive(Eq)]
enum K<T: Ord> {
    Minimum,
    Key(T),
}

impl<T: Ord> Ord for K<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            &K::Minimum => match other {
                &K::Minimum => Ordering::Equal,
                &K::Key(ref t) => Ordering::Less,
            },
            &K::Key(ref x) => match other {
                &K::Minimum => Ordering::Greater,
                &K::Key(ref y) => x.cmp(y),
            },
        }
    }
}

impl<T: Ord> PartialOrd for K<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialEq for K<T> {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

pub struct Node<T: Ord> {
    key: T,
    next: Vec<AtomicCell<Option<NonNull<Node<T>>>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T, height: u32) -> Self {
        let mut ret = Self {
            key,
            next: Vec::with_capacity(height as usize),
        };
        for _ in 0..height {
            ret.next.push(AtomicCell::default());
        }
        ret
    }

    fn next(&self, level: u32) -> Option<NonNull<Node<T>>> {
        self.next[level as usize].load()
    }

    fn set_next(&self, level: u32, x: Option<NonNull<Node<T>>>) {
        self.next[level as usize].store(x)
    }
}

mod test {
    use crate::skiplist::K;
    use std::cmp::Ordering;
    #[test]
    fn test_key() {
        let Minimum1 = K::<i32>::Minimum;
        let Minimum2 = K::<i32>::Minimum;
        assert_eq!(Minimum1.cmp(&Minimum2), Ordering::Equal);
        let x = K::Key(12);
        let y = K::Key(1);
        assert_eq!(Minimum1.cmp(&x), Ordering::Less);
        assert_eq!(x.cmp(&Minimum1), Ordering::Greater);
        assert_eq!(x.cmp(&y), Ordering::Greater);
    }
}
