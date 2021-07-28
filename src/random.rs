pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> ui32 {
        let M: u32 = 2147483647;
        let A: u64 = 16807;
        let product = self.seed * M;
        self.seed = ((product >> 31) + (product & M)) as u32;
        if self.seed > M {
            self.seed -= M;
        }
        self.seed
    }

    pub fn uniform(&mut self, n: i32) -> u32 {
        self.next() % n
    }

    pub fn one_in(&mut self, n: i32) -> bool {
        (self.next() % n) == 0
    }

    pub fn skewed(&mut self, max_log: i32) -> u32 {
        self.uniform(1 << self.uniform(max_log + 1))
    }
}
