use std::mem::size_of;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

const BLOCK_SIZE: usize = 4096;
// a Vec has pointer and two usize field(cap and len)
const POINTER_LENGTH: usize = size_of::<*mut u8>();
const EXTRA_VEC_LEN: usize = POINTER_LENGTH + 2 * size_of::<usize>();

pub struct Arena {
    alloc_ptr_: *mut u8,
    alloc_bytes_remaining_: usize,
    memory_usage_: AtomicUsize,
    blocks_: Vec<Vec<u8>>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            alloc_ptr_: ptr::null_mut(),
            alloc_bytes_remaining_: 0,
            memory_usage_: AtomicUsize::new(0),
            blocks_: Vec::new(),
        }
    }

    #[inline]
    pub fn allocate(&mut self, bytes: usize) -> *mut u8 {
        assert!(bytes > 0);
        if bytes <= self.alloc_bytes_remaining_ {
            let result = self.alloc_ptr_;
            unsafe {
                self.alloc_ptr_ = self.alloc_ptr_.add(bytes);
            }
            self.alloc_bytes_remaining_ -= bytes;
            result
        } else {
            self.allocate_fallback(bytes)
        }
    }

    pub fn allocate_aligned(&mut self, bytes: usize) -> *mut u8 {
        let align = if POINTER_LENGTH > 8 {
            POINTER_LENGTH
        } else {
            8
        };

        let current_mod = self.alloc_ptr_ as usize & (align - 1);
        let slop = if current_mod == 0 {
            0
        } else {
            align - current_mod
        };

        let needed = bytes + slop;
        if needed <= self.alloc_bytes_remaining_ {
            let result = unsafe { self.alloc_ptr_.add(slop) };
            self.alloc_ptr_ = unsafe { self.alloc_ptr_.add(needed) };
            self.alloc_bytes_remaining_ -= needed;
            result
        } else {
            self.allocate_fallback(bytes)
        }
    }

    pub fn memory_usage(&self) -> usize {
        self.memory_usage_.load(Ordering::SeqCst)
    }

    fn allocate_fallback(&mut self, n: usize) -> *mut u8 {
        if n > BLOCK_SIZE / 4 {
            self.allocate_new_block(n)
        } else {
            self.alloc_ptr_ = self.allocate_new_block(BLOCK_SIZE);
            self.alloc_bytes_remaining_ = BLOCK_SIZE;
            let result = self.alloc_ptr_;
            unsafe {
                self.alloc_ptr_ = self.alloc_ptr_.add(n);
            }
            self.alloc_bytes_remaining_ -= n;
            result
        }
    }

    fn allocate_new_block(&mut self, block_bytes: usize) -> *mut u8 {
        let mut v: Vec<u8> = Vec::with_capacity(block_bytes);
        unsafe {
            v.set_len(block_bytes);
        }
        let r = v.as_mut_ptr();
        self.blocks_.push(v);
        self.memory_usage_
            .fetch_add(block_bytes + EXTRA_VEC_LEN, Ordering::SeqCst);
        r
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use crate::random::Random;

    use super::*;
    #[test]
    fn test_empty() {
        let arena = Arena::new();
    }

    #[test]
    fn test_simple() {
        let mut allocated: Vec<(usize, *mut u8)> = Vec::new();
        let mut arena = Arena::new();
        let N = 100000;
        let mut bytes = 0;
        let rnd = Random::new(301);
        for i in 0..N {
            let mut s;
            if i % (N / 10) == 0 {
                s = i;
            } else {
                s = if rnd.one_in(4000) {
                    rnd.uniform(6000)
                } else {
                    if rnd.one_in(10) {
                        rnd.uniform(100)
                    } else {
                        rnd.uniform(20)
                    }
                };
            }
            if s == 0 {
                // Our arena disallows size 0 allocation
                s = 1;
            }
            let r;
            if rnd.one_in(10) {
                r = arena.allocate_aligned(s as usize);
            } else {
                r = arena.allocate(s as usize);
            }
            for b in 0..s {
                // Fill the ''i'' th allocation with a known bit pattern
                unsafe {
                    std::ptr::write::<u8>(r.add(b as usize), (i % 256) as u8);
                }
            }
            bytes += s;
            allocated.push((s as usize, r));
            println!("{} {} {}", s, bytes, i);
            println!(
                "memory_usage: {} bytes: {}",
                arena.memory_usage(),
                bytes as f64 * 1.10
            );
            assert!(arena.memory_usage() >= bytes as usize);
            if i > N / 10 {
                println!(
                    "memory_usage: {} bytes: {}",
                    arena.memory_usage(),
                    bytes as f64 * 1.10
                );
                assert!(arena.memory_usage() as f64 <= (f64::from(bytes) * 1.10));
            }
        }
        let mut i = 0;
        for (num_bytes, p) in allocated {
            for b in 0..num_bytes {
                unsafe {
                    assert_eq!(std::ptr::read::<u8>(p.add(b)) & 0xff, (i % 256) as u8);
                }
            }
            i += 1;
        }
    }
}
