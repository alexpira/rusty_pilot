

use crate::common::Fpt;
use crate::deg2rad;
use std::f64;

struct Angle { pub rad: f64, pub sin: f64, pub cos: f64 }

pub struct Trig {
	data: Vec<Angle>
}
impl Trig {
	pub fn new() -> Self {
		let mut d = Vec::with_capacity(360);
		for i in 0..360 {
			let r = deg2rad!(i);
			d.push(Angle {
				rad: r,
				sin: r.sin(),
				cos: r.cos()
			});
		}
		Self { data: d }
	}
	fn el(&self, deg: i32) -> &Angle {
		let mut deg = deg.clone();
		while deg < 0 { deg += 360; }
		let idx = (deg % 360) as usize;
		&self.data[idx]
	}
	pub fn rad(&self, deg: i32) -> f64 { self.el(deg).rad }
	pub fn sin(&self, deg: i32) -> f64 { self.el(deg).sin }
	pub fn cos(&self, deg: i32) -> f64 { self.el(deg).cos }

	pub fn rot(&self, p: &Point, deg: i32) -> Point {
		Point::new(
			self.cos(deg)*p.x - self.sin(deg)*p.y,
			self.sin(deg)*p.x + self.cos(deg)*p.y,
		)
	}
}

#[derive(Clone)]
pub struct Point {
	x: Fpt,
	y: Fpt,
}
impl Point {
	pub fn new(x: Fpt, y: Fpt) -> Self {
		Self {
			x: x,
			y: y,
		}
	}
	/* pub fn from(rot: i32, len: Fpt) -> Self {
		Self {
			x: len * deg2rad!(rot).sin(),
			y: -len * deg2rad!(rot).cos(),
		}
	} */

	pub fn x(&self) -> Fpt { self.x }
	pub fn y(&self) -> Fpt { self.y }

	#[allow(dead_code)]
	pub fn length2(&self) -> Fpt {
		self.x*self.x + self.y*self.y
	}

	#[allow(dead_code)]
	pub fn length(&self) -> Fpt {
		Fpt::sqrt(self.length2())
	}

	pub fn add(&mut self, p: &Point) {
		self.x += p.x;
		self.y += p.y;
	}

	pub fn mul(&mut self, amt: Fpt) {
		self.x *= amt;
		self.y *= amt;
	}

	pub fn flipx(&mut self) {
		self.x = -self.x;
	}
	pub fn flipy(&mut self) {
		self.y = -self.y;
	}
}
impl std::fmt::Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:.2},{:.2}", self.x, self.y)
	}
}

fn side(a1: &Point, a2: &Point, b: &Point) -> i32 {
/*
 * # translate origin to a1
 * a1t = (0,0)
 * a2t = (a2.x-a1.x, a2.y-a1.y)
 * bt = (b.x-a1.x, b.y-a1.y)
 *
 * # rotate a2t 90 degrees
 * a2tr = (-a2t.y, a2t.x) = (a1.y-a2.y, a2.x-a1.x)
 *
 * # dot product of bt and a2tr
 * adr = a2tr.x*bt.x + a2tr.y*bt.y =
 *       (a1.y-a2.y)*(b.x-a1.x) + (a2.x-a1.x)*(b.y-a1.y)
 */

	if (a1.y-a2.y)*(b.x-a1.x) + (a2.x-a1.x)*(b.y-a1.y) > 0.0 {
		return 1;
	}
	-1
}

#[allow(dead_code)]
fn same_side(a1: &Point, a2: &Point, b1: &Point, b2: &Point) -> bool {
	side(a1, a2, b1) * side(a1, a2, b2) > 0
}

fn collide_tri(
		t1a: &Point, t1b: &Point, t1c: &Point,
		t2a: &Point, t2b: &Point, t2c: &Point
	) -> bool {

	/*
	let minx1 = min!(t1a.x,t1b.x,t1c.x);
	let minx2 = min!(t2a.x,t2b.x,t2c.x);

	let maxx1 = max!(t1a.x,t1b.x,t1c.x);
	let maxx2 = max!(t2a.x,t2b.x,t2c.x);

	let miny1 = min!(t1a.y,t1b.y,t1c.y);
	let miny2 = min!(t2a.y,t2b.y,t2c.y);

	let maxy1 = max!(t1a.y,t1b.y,t1c.y);
	let maxy2 = max!(t2a.y,t2b.y,t2c.y);

	if minx1 > maxx2 { return false; }
	if maxx1 < minx2 { return false; }
	if miny1 > maxy2 { return false; }
	if maxy1 < miny2 { return false; }
	*/

	for (s1,s2,r,a,b,c) in vec![
		(t1a, t1b, t1c, t2a, t2b, t2c),
		(t1b, t1c, t1a, t2a, t2b, t2c),
		(t1c, t1a, t1b, t2a, t2b, t2c),
		(t2a, t2b, t2c, t1a, t1b, t1c),
		(t2b, t2c, t2a, t1a, t1b, t1c),
		(t2c, t2a, t2b, t1a, t1b, t1c),
	].iter() {
		let sr = side(s1,s2, r);
		if sr * side(s1,s2, a) > 0 { continue; }
		if sr * side(s1,s2, b) > 0 { continue; }
		if sr * side(s1,s2, c) > 0 { continue; }
		return false;
	}
	true
}

pub fn collide(s1: &Vec<Point>, s2: &Vec<Point>) -> bool {
	for i1 in 1..s1.len()-1 {
		for i2 in 1..s2.len()-1 {
			if collide_tri(
				&s1[0],&s1[i1],&s1[i1+1],
				&s2[0],&s2[i2],&s2[i2+1]) {
				return true;
			}
		}
	}
	false
}

pub fn inside_rect(p: &Point, x0: Fpt, y0: Fpt, x1: Fpt, y1: Fpt) -> bool {
	if p.x < x0 { return false; }
	if p.x > x1 { return false; }
	if p.y < y0 { return false; }
	if p.y > y1 { return false; }
	true
}
