use rand::Rng;
use std::{
    ptr::{null, null_mut},
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
    vec,
};
const KMaxHeight: usize = 12;
pub struct SkipList {
    head_: *mut Node,
    max_hegiht_: AtomicUsize,
}

struct Node {
    next_: Vec<AtomicPtr<Node>>,
    key_: Vec<u8>,
}

impl Node {
    fn new(key: Vec<u8>, height: usize) -> Self {
        let mut vec = Vec::new();
        for i in 0..height {
            vec.push(AtomicPtr::new(null_mut()));
        }
        Self {
            next_: vec,
            key_: key,
        }
    }
    fn Next(&self, level: usize) -> *mut Node {
        assert!(level >= 0);
        self.next_[level].load(Ordering::Acquire)
    }

    fn SetNext(&self, level: usize, next: *mut Node) {
        assert!(level >= 0);
        self.next_[level].store(next, Ordering::Release)
    }
}

impl SkipList {
    fn new() -> Self {
        Self {
            head_: Box::into_raw(Box::new(Node::new(vec![0], KMaxHeight))),
            max_hegiht_: AtomicUsize::new(1),
        }
    }

    fn GetMaxHeight(&self) -> usize {
        self.max_hegiht_.load(Ordering::Relaxed)
    }

    fn keyIsAfterNode(key: &Vec<u8>, n: *mut Node) -> bool {
        !n.is_null() && unsafe { key.to_vec() > n.as_ref().unwrap().key_ }
    }

    fn findGreaterOrEqual(&self, key: &Vec<u8>, prev: &mut Vec<*mut Node>) -> *mut Node {
        let mut n = self.head_;
        let mut level = self.GetMaxHeight() - 1;
        loop {
            unsafe {
                let next = n.as_ref().unwrap().Next(level);
                if SkipList::keyIsAfterNode(key, next) {
                    n = next;
                } else {
                    if prev.len() != 0 {
                        prev[level] = n;
                    }
                    if level == 0 {
                        return next;
                    } else {
                        level -= 1;
                    }
                }
            }
        }
    }

    fn findLessThan(&self, key: &Vec<u8>) -> *mut Node {
        let mut n = self.head_;
        let mut level = self.GetMaxHeight() - 1;
        loop {
            unsafe {
                let next = n.as_ref().unwrap().Next(level);
                if next == null_mut() || next.as_ref().unwrap().key_ > key.to_vec() {
                    if level == 0 {
                        return n;
                    } else {
                        level -= 1;
                    }
                } else {
                    n = next;
                }
            }
        }
    }

    fn GetRandomHeight() -> usize {
        let mut height = 1;
        let kBranching = 4;
        while height < KMaxHeight && (rand::random::<usize>() % kBranching) == 0 {
            height += 1;
        }
        height
    }

    fn Insert(&mut self, key: &Vec<u8>) {
        let mut prev = vec![null_mut(); KMaxHeight];
        let n = self.findGreaterOrEqual(key, &mut prev);
        let height = Self::GetRandomHeight();
        if height > self.GetMaxHeight() {
            for i in self.GetMaxHeight()..height {
                prev[i] = self.head_;
            }
            self.max_hegiht_.store(height, Ordering::Relaxed);
        }
        let x = Box::into_raw(Box::new(Node::new(key.to_vec(), height)));
        for i in 0..height {
            unsafe {
                x.as_mut()
                    .unwrap()
                    .SetNext(i, prev[i].as_mut().unwrap().Next(i));
                prev[i].as_mut().unwrap().SetNext(i, x);
            }
        }
    }

    fn contain(&self, key: &Vec<u8>) -> bool {
        let mut prev = vec![];
        let n = self.findGreaterOrEqual(key, &mut prev);
        println!("key is {:?}, node is {:?}", key, unsafe {});
        if n != null_mut() && unsafe { key.to_vec() == n.as_ref().unwrap().key_ } {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let mut list = SkipList::new();
        list.Insert(&"hello".as_bytes().to_vec());
        assert_eq!(list.contain(&"hello".as_bytes().to_vec()), true);
    }

    #[test]
    fn test_insert_and_lookup() {
        let N: i32 = 2000;
        let R: i32 = 5000;
        rand
    }
}
