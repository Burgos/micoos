#![no_std]
#![feature(lang_items)]
#![feature(core)]
#![feature(core_intrinsics)] 

pub mod Register
{
	use core::intrinsics::{volatile_store, volatile_load};

	pub struct Register<T> where T: Copy {
		addr: *mut T,
		value: T
	}

	impl<T:Copy> Register<T> {
		pub fn new (addr: *mut T, initial_val: T) -> Register<T> {
			Register {
				addr: addr,
				value: initial_val
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
}
