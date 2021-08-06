use std::cell::Cell;
pub struct Random {
    seed: Cell<u32>,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self {
            seed: Cell::new(seed),
        }
    }

    pub fn next(&self) -> u32 {
        let M: u32 = 2147483647;
        let A: u64 = 16807;
        let product = self.seed.get() as u64 * A;
        self.seed
            .set(((product >> 31) as u32 + (product as u32 & M)) as u32);
        let _seed = self.seed.get();
        if _seed > M {
            self.seed.set(_seed - M);
        }
        self.seed.get()
    }

    pub fn uniform(&self, n: u32) -> u32 {
        self.next() % n
    }

    pub fn one_in(&self, n: u32) -> bool {
        (self.next() % n) == 0
    }

    pub fn skewed(&self, max_log: u32) -> u32 {
        self.uniform(1 << self.uniform(max_log + 1))
    }
}
