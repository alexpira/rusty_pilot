
use crate::common::Fpt;

pub struct Random {
	data: Vec<u8>,
	ptr: usize
}
impl Random {
	pub fn new() -> Self {
		let mut d : Vec<u8> = vec![0u8; 1024];
		let _ = getrandom::getrandom(&mut d);
		Self {
			ptr: 0,
			data: d
		}
	}
	pub fn next(&mut self) -> u8 {
		let rv = self.data[self.ptr];
		self.ptr += 1;
		if self.ptr >= self.data.len() {
			self.ptr = 0;
		}
		rv
	}
	pub fn nextbits(&mut self, nbit: u8) -> u8 {
		if nbit == 0 {
			return 0;
		}
		if nbit > 7 {
			return self.next();
		}
		let mask = (1 << nbit) - 1;
		self.next() & mask
	}
	pub fn nextfloat(&mut self) -> Fpt {
		self.next() as Fpt / 255.0
	}
	pub fn rand(&mut self, max: i32) -> i32 {
		if max <= 0 {
			return 0;
		}
		(self.nextfloat() * (max as Fpt)) as i32
	}
}
