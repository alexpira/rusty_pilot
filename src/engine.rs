use std::f64;
use crate::common::*;
use crate::rand::Random;
use crate::levels::GameData;
use crate::geom::{Trig,Point,collide,inside_rect};
use wasm_bindgen::JsValue;

pub struct Wind {
	shape: Vec<Point>,
	power: Fpt,
	orientation: i32,
	accel: Option<Point>,
}

impl Wind {
	pub fn new(shape: Vec<Point>, power: Fpt, orientation: i32) -> Self {
		Self {
			shape: shape,
			power: power,
			orientation: orientation,
			accel: None,
		}
	}
	pub fn shape(&self) -> &Vec<Point> { &self.shape }
	pub fn accel(&mut self, trig: &Trig) -> &Point {
		match &self.accel {
			None => {
				let acc = Point::new(
					self.power * trig.cos(self.orientation),
					self.power * trig.sin(self.orientation)
				);
				self.accel = Some(acc);
			},
			Some(_) => {}
		}
		self.accel.as_ref().unwrap()
	}
	pub fn direction(&self) -> i32 { self.orientation }
}

pub struct Particle {
	pos: Point,
	dir: Point,
	col: JsValue,
	life: u32
}

impl Particle {
	pub fn new(col: &str, pos: Point, dir: Point) -> Self {
		Self {
			pos: pos,
			dir: dir,
			col: JsValue::from_str(col),
			life: 20
		}
	}

	pub fn move_step(&mut self) {
		if self.life > 0 {
			self.pos.add(&self.dir);
			self.life -= 1;
		}
	}
	pub fn finished(&self) -> bool {
		self.life == 0
	}
	pub fn alpha(&self) -> f64 {
		if self.life > 10 {
			return 1.0;
		}
		return (self.life as f64) / 10.0;
	}
	pub fn color(&self) -> &JsValue {
		&self.col
	}
	pub fn position(&self) -> &Point {
		&self.pos
	}
}

pub struct Asteroid {
	vert: Vec<Point>,
	pos: Point,
	rot: i32,
	dpos: Point,
	drot: i32,
}
impl Asteroid {
	pub fn move_step(&mut self, area: &Point) {
		self.pos.add(&self.dpos);
		self.rot += self.drot;
		const EXTRASIZE : Fpt = 30.0;
		if self.pos.x() < -EXTRASIZE && self.dpos.x() < 0.0 {
			self.dpos.flipx();
		}
		if self.pos.x() >= area.x() + EXTRASIZE && self.dpos.x() > 0.0 {
			self.dpos.flipx();
		}
		if self.pos.y() < -EXTRASIZE && self.dpos.y() < 0.0 {
			self.dpos.flipy();
		}
		if self.pos.y() >= area.y() + EXTRASIZE && self.dpos.y() > 0.0 {
			self.dpos.flipy();
		}
	}
	pub fn finished(&self) -> bool {
		false
	}
	pub fn remap4coll(&self, trig: &Trig) -> Vec<Point> {
		let mut rv = Vec::with_capacity(1 + self.vert.len());
		rv.push(self.pos.clone());
		let mut remap = self.remap(trig);
		// close the surrounding shape or one side will be left open
		let p0 = remap[0].clone();
		rv.append(&mut remap);
		rv.push(p0);
		rv
	}
	pub fn remap(&self, trig: &Trig) -> Vec<Point> {
		self.vert.iter().map(|p| {
			let mut tr = trig.rot(p, self.rot);
			tr.add(&self.pos);
			tr
		}).collect()
	}
}

pub struct GameEngine {
	rot: i32,
	pos: Point,
	speed: Point,
	thrust: bool,
	block_alert: bool,
	lrot: bool,
	rrot: bool,
	fuel: u32,
	fuel_warn: u32,
	collided: bool,
	landed: bool,
	particles: Vec<Particle>,
	asteroids: Vec<Asteroid>,
	blownup: bool,
	rng: Random,
	trig: Trig,
	draw_step: u32,
	viewport_pos: Option<Point>,
	config: GameData
}
impl GameEngine {
	pub fn new(cfg: GameData) -> Self {
		let mut rv = Self {
			rot: 0i32,
			pos: cfg.pos0.clone(),
			speed: cfg.speed0.clone(),
			thrust: false,
			block_alert: false,
			lrot: false, rrot: false,
			fuel: cfg.initial_fuel as u32,
			fuel_warn: (cfg.full_fuel / 5.0) as u32,
			collided: false,
			landed: false,
			particles: Vec::new(),
			asteroids: Vec::new(),
			rng: Random::new(),
			blownup: false,
			trig: Trig::new(),
			draw_step: 0u32,
			viewport_pos: match &cfg.viewport_pos0 {
				None => None,
				Some(x) => Some(x.clone())
			},
			config: cfg
		};
		for _ in 0..rv.config.num_asteroids {
			let ast = rv.new_asteroid();
			rv.asteroids.push(ast);
		}
		rv
	}
	pub fn fuel_warn(&self) -> bool {
		self.fuel <= self.fuel_warn
	}
	pub fn ship_pos(&self) -> Point {
		self.pos.clone()
	}
	#[allow(dead_code)]
	fn accel(&mut self, p: &Point) {
		self.speed.add(p);
	}
	fn apply_gravity(&mut self) {
		self.speed.add(&self.config.gravity);
	}
	fn apply_wind(&mut self, ship: &Vec<Point>) {
		for wind in self.config.winds.iter_mut() {
			if collide(&ship, wind.shape()) {
				self.speed.add(wind.accel(&self.trig));
			}
		}
	}
	fn friction(&mut self, amt: Fpt) {
		self.speed.mul(amt);
	}
	pub fn set_thrust(&mut self, value: bool) {
		self.thrust = value;
	}
	pub fn rotate(&mut self, value: i32) {
		if ! self.landed {
			let mut v = self.rot + value;
			while v < 0 { v += 360; }
			self.rot = v % 360;
		}
	}
	pub fn set_rotation(&mut self, left: Option<bool>, right: Option<bool>) {
		if let Some(v) = left {
			self.lrot = v;
		}
		if let Some(v) = right {
			self.rrot = v;
		}
	}
	pub fn is_level(&self) -> bool {
		!self.blownup &&
		(self.rot < self.config.levelling_rot || self.rot > (360 - self.config.levelling_rot)) &&
			(self.speed.x().abs() < self.config.levelling_speed_x) &&
			(self.speed.y().abs() < self.config.levelling_speed_y)
	}
	fn apply_thrust(&mut self, amt: Fpt) {
		if !self.blownup && self.thrust && self.fuel > 0 {
			self.fuel -= 1;
			self.speed.add(&Point::new(
				amt * self.trig.sin(self.rot),
				-amt * self.trig.cos(self.rot)
			));

			let drot = self.rot - 31 + (self.rng.nextbits(6) as i32);
			let pt = self.remap_ship(&Point::new(0.0, 10.0));
			let mut	delta = Point::new(
				-0.5 * self.trig.sin(drot),
				0.5 * self.trig.cos(drot)
            );
			delta.add(&self.speed);
			self.particles.push(Particle::new("#fcdb03", pt, delta));
		}
	}
	fn apply_rotation(&mut self, amt: i32) {
		let mut drot: i32 = 0;
		if self.lrot {
			drot -= amt;
		}
		if self.rrot {
			drot += amt;
		}
		self.rotate(drot);
	}
	/* pub fn has_fuel(&self) -> bool {
		self.fuel > 0
	} */
	pub fn has_collided(&self) -> bool {
		self.collided
	}
	pub fn has_landed(&self) -> bool {
		self.landed
	}
	fn collision(&self, ship: &Vec<Point>) -> bool {
		for p in ship.iter() {
			if !inside_rect(&p, 0.0, 0.0, self.config.area.x(), self.config.area.y()) {
				return true;
			}
		}
		for obs in self.obs_shape().iter() {
			if collide(&ship, &obs) {
				return true;
			}
		}
		for ast in self.asteroids.iter() {
			if collide(&ship, &ast.remap4coll(&self.trig)) {
				return true;
			}
		}
		if collide(&ship, &self.land_shape()) {
			return true;
		}
		false
	}
	fn landing(&self) -> bool {
		if !self.is_level() || self.blownup {
			return false;
		}
		let touch = self.remap_ship(&Point::new(0.0, 10.0));
		if inside_rect(&touch, self.config.target_x0, self.config.target_y, self.config.target_x1, self.config.target_y + 5.0) {
			return true;
		}
		if touch.x() < self.config.target_x0 || touch.x() > self.config.target_x1 {
			return false;
		}
		let touch = self.remap_ship(&Point::new(-10.0, 10.0));
		if inside_rect(&touch, self.config.target_x0, self.config.target_y, self.config.target_x1, self.config.target_y + 5.0) {
			return true;
		}
		let touch = self.remap_ship(&Point::new(10.0, 10.0));
		inside_rect(&touch, self.config.target_x0, self.config.target_y, self.config.target_x1, self.config.target_y + 5.0)
	}

	fn blowup(&mut self) {
		let base = self.remap_ship(&Point::new(0.0,0.0));
		for _ in 0..200 {
			let x = base.x() - 15.0 + (self.rng.nextbits(5) as Fpt);
			let y = base.y() - 15.0 + (self.rng.nextbits(5) as Fpt);
			let dx = (x-base.x()) / ((self.rng.nextbits(3) + 5) as Fpt) + self.speed.x();
			let dy = (y-base.y()) / ((self.rng.nextbits(3) + 5) as Fpt) + self.speed.y();
			self.particles.push(Particle::new("#42a4f5", Point::new(x,y), Point::new(dx,dy)));
		}
	}

	pub fn get_step(&self) -> u32 {
		if self.draw_step < 15 {
			return 0u32;
		}
		1u32
	}

	pub fn move_step(&mut self) {
		self.draw_step = (self.draw_step + 1) % 30;

		for a in self.asteroids.iter_mut() {
			a.move_step(&self.config.area);
		}
		self.asteroids.retain(|p| { !p.finished() });

		if !self.blownup {
			self.pos.add(&self.speed);
			self.landed = self.landed || self.landing();
			if self.landed {
				self.collided = false;
				self.rot = 0;
				self.speed = Point::new(0.0,0.0);
			} else {
				let ship = self.ship_shape();
				self.collided = self.collision(&ship);
				if self.collided {
					self.blownup = true;
					self.blowup();
				}
				self.apply_gravity();
				self.apply_wind(&ship);
				self.apply_rotation(6);
				self.apply_thrust(self.config.thrust_pow);
				self.friction(self.config.friction);
			}
		}

		for p in self.particles.iter_mut() {
			p.move_step();
		}
		self.particles.retain(|p| { !p.finished() });
		self.reposition_viewport();
	}

	pub fn remap_ship(&self, p: &Point) -> Point {
		let mut tr = self.trig.rot(p, self.rot);
		tr.add(&self.pos);
		tr
	}

	pub fn fuel_sz(&self, maxref: Fpt) -> Fpt {
		(self.fuel as Fpt) * maxref / self.config.full_fuel
	}
	pub fn ship_shape(&self) -> Vec<Point> {
		if self.blownup {
			return vec![];
		}
		vec![
			Point::new(0.0, -20.0),
			Point::new(-10.0, 10.0),
			Point::new(10.0, 10.0),
		].iter().map(|p| self.remap_ship(p)).collect()
	}
	pub fn land_shape(&self) -> Vec<Point> {
		vec!(
			Point::new(self.config.target_x0, self.config.target_y),
			Point::new(self.config.target_x1, self.config.target_y),
			Point::new(self.config.target_x1, self.config.target_y + 5.0),
			Point::new(self.config.target_x0, self.config.target_y + 5.0)
		)
	}
	pub fn aster_shape(&self) -> Vec<Vec<Point>> {
		let mut rv = Vec::with_capacity(self.asteroids.len());
		for ast in self.asteroids.iter() {
			rv.push(ast.remap(&self.trig));
		}
		rv
	}
	pub fn obs_shape(&self) -> &Vec<Vec<Point>> {
		&self.config.walls
	}

	pub fn iter_winds<F>(&self, mut f: F) where F: FnMut(&Wind, &Trig) -> () {
		for w in self.config.winds.iter() {
			f(&w, &self.trig);
		}
	}

	pub fn iter_part<F>(&self, mut f: F) where F: FnMut(&Particle) -> () {
		for p in self.particles.iter() {
			f(&p);
		}
	}

	fn new_asteroid(&mut self) -> Asteroid {
		let mut v = vec![];
		let mut vgen = self.rng.nextbits(6) as i32;
		while vgen < 360 {
			let vdist = (self.rng.nextbits(5) + 30) as Fpt;
			v.push(Point::new(
				vdist * self.trig.sin(vgen),
				vdist * self.trig.cos(vgen)
			));
			vgen += self.rng.nextbits(5) as i32 + 30;
		}
		let p = Point::new(
			self.config.asteroid_pos0.x() + self.rng.rand(self.config.asteroid_area.x() as i32) as Fpt,
			self.config.asteroid_pos0.y() + self.rng.rand(self.config.asteroid_area.y() as i32) as Fpt
		);
		let dp = Point::new(
			self.rng.nextfloat() + 0.2,
			self.rng.nextfloat() + 0.2
		);
		let dr = self.rng.nextbits(3) as i32 - 4;
		Asteroid {
			vert: v,
			pos: p,
			rot: 0,
			dpos: dp,
			drot: dr,
		}
	}
	pub fn finished(&self) -> bool {
		(self.blownup && (self.particles.len() == 0)) || self.stuck() || self.landed
	}
	fn stuck(&self) -> bool {
		!self.landed && !self.blownup && self.fuel == 0u32 && self.speed.is_zero() && self.config.gravity.is_zero()
	}
	pub fn area_width(&self) -> Fpt {
		self.config.area.x()
	}
	pub fn area_height(&self) -> Fpt {
		self.config.area.y()
	}
	pub fn scrollable(&self) -> bool {
		match &self.config.viewport {
			Some(_) => true,
			None => false
		}
	}
	fn reposition_viewport(&mut self) {
		if ! self.scrollable() {
			return;
		}
		let margin = 150.0;

		let aw = self.config.area.x();
		let ah = self.config.area.y();
		let vpw = match &self.config.viewport { Some(v) => v.x(), None => return };
		let vph = match &self.config.viewport { Some(v) => v.y(), None => return };
		let vpx = match &self.viewport_pos { Some(v) => v.x(), None => 0.0 };
		let vpy = match &self.viewport_pos { Some(v) => v.y(), None => 0.0 };

		let vpx = f64::min(vpx, self.pos.x() - margin);
		let vpx = f64::max(vpx + vpw, self.pos.x() + margin) - vpw;
		let vpx = f64::max(vpx, -30.0);
		let vpx = f64::min(vpx, aw - vpw + 30.0);

		let vpy = f64::min(vpy, self.pos.y() - margin);
		let vpy = f64::max(vpy + vph, self.pos.y() + margin) - vph;
		let vpy = f64::max(vpy, -30.0);
		let vpy = f64::min(vpy, ah - vph + 30.0);
		self.viewport_pos = Some(Point::new(vpx, vpy));
	}

	pub fn viewport_width(&self) -> Fpt {
		match &self.config.viewport {
			Some(v) => v.x(),
			None => self.config.area.x()
		}
	}
	pub fn viewport_height(&self) -> Fpt {
		match &self.config.viewport {
			Some(v) => v.y(),
			None => self.config.area.y()
		}
	}
	pub fn viewport_pos(&self) -> Point {
		match &self.viewport_pos {
			Some(v) => v.clone(),
			None => Point::new(0.0, 0.0)
		}
	}

	pub fn block_alert(&self) -> bool {
		self.block_alert
	}
	pub fn set_block_alert(&mut self, v: bool) {
		self.block_alert = v;
	}

	/* pub fn iter_aster<F>(&self, mut f: F) where F: FnMut(&Asteroid) -> () {
		for p in self.asteroids.iter() {
			f(&p);
		}
	} */
}

