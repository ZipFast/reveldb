use std::ops::Index;
use std::cell::Cell;
use std::cmp::{Ordering, min};


/// Slice is a simple structure containing a reference into some external
/// storage and a size. The user of a Slice mut ensure that the slice
/// is not used after the corresponding external storage has been
/// deallocated.
///
/// multiple threads can invoke read methods on a Slice without external
/// synchronization, but if any of the threads may call a update method,
/// all threads accessing the same Slice mut use external synchronization
#[derive(Eq)]
pub struct Slice {
	data: Cell<&'static str>,
	size: Cell<usize>,
}

impl ToString for Slice {
	fn to_string(&self) -> String {
		String::from(self.data())
	}
}

impl PartialOrd for Slice {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Slice {
	fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
	}
}

impl Ord for Slice {
	fn cmp(&self, other: &Self) -> Ordering {
		let min_size = min(self.size(), other.size());
		match self.data()[..min_size].cmp(&other.data()[..min_size]) {
			Ordering::Equal => {
				if self.size() < other.size() {
					Ordering::Less
				} else if self.size() > other.size() {
					Ordering::Greater
				} else {
					Ordering::Equal
				}
			},
			default => default,
		}

	}
}

impl Slice {
	pub fn new() -> Self {
		Self {
			data: Cell::new(""),
			size: Cell::new(0),
		}
	}

	pub fn from_str(s: &'static str) -> Self {
		Self {
			data: Cell::new(s),
			size: Cell::new(s.len()),
		}
	}

	pub fn from_slice(slice: &Slice) -> Self {
		Self {
			data: Cell::new(slice.data.get()),
			size: Cell::new(slice.size.get()),
		}
	}

	pub fn data(&self) -> &'static str {
		self.data.get()
	}

	pub fn size(&self) -> usize {
		self.size.get()
	}

	pub fn empty(&self) -> bool {
		self.size.get()== 0
	}

	pub fn clear(&self) {
		self.data.set("");
		self.size.set(0);
	}

	pub fn remove_prefix(&self, n: usize) {
		assert!(n <= self.size());
		self.data.set(&self.data()[n..]);
		self.size.set(self.size() - n);
	}

	pub fn start_with(&self, other: &Self) -> bool {
		return (self.size() >= other.size()) && (self.data()[..other.size()].eq(other.data()));

	}
}

impl Index<usize> for Slice {
	type Output = u8;
	fn index(&self, index: usize) -> &Self::Output {
        self.data().as_bytes().index(index)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn start_with_test() {
		let slice1 = Slice::from_str("hello world");
		let slice2 = Slice::from_str("hello");
		assert_eq!(slice1.start_with(&slice2), true);
	}

	#[test]
	fn remove_prefix_test() {
		let slice = Slice::from_str("hello world");
		let after = Slice::from_str("world");
		slice.remove_prefix(6);
		assert!(slice.eq(&after));
	}

	#[test]
	fn clear_test() {
		let slice = Slice::from_str("hello");
		slice.clear();
		assert!(Slice::new().eq(&slice));
	}

	#[test]
	fn cmp_test() {
		let slice1 = Slice::from_str("hello");
		let slice2 = Slice::from_str("hello, world");
		assert!(slice1 < slice2);
	}

	#[test]
	fn from_slice_test() {
		let slice1 = Slice::from_str("hello");
		let slice2 = Slice::from_slice(&slice1);
		assert!(slice1.eq(&slice2));
	}

	#[test]
	fn empty_test() {
		let slice1 = Slice::from_str("hello");
		slice1.clear();
		assert!(slice1.empty());
	}
}