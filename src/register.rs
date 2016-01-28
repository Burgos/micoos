use core::intrinsics::{volatile_store, volatile_load};

pub struct Register<T> where T: Copy {
	addr: *mut T,
}

impl<T:Copy> Register<T> {
	pub fn new (addr: *mut T) -> Register<T> {
		Register {
			addr: addr,
		}
	}

	pub fn get(&self) -> T {
		unsafe {
			volatile_load(self.addr)
		}
	}

	pub fn set(&self, value: T) {
		unsafe {
			volatile_store(self.addr, value)
		}
	}
}
