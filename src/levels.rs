
use std::cmp::max;
use crate::common::*;
use crate::geom::Point;
use crate::pt;
use crate::engine::Wind;

pub struct GameData {
	pub area: Point,
	pub pos0: Point,
	pub speed0: Point,
	pub target_y: Fpt,
	pub target_x0: Fpt,
	pub target_x1: Fpt,
	pub num_asteroids: u32,
	pub asteroid_pos0: Point,
	pub asteroid_area: Point,
	pub levelling_rot: i32,
	pub levelling_speed_x: Fpt,
	pub levelling_speed_y: Fpt,
	pub initial_fuel: u32,
	pub full_fuel: Fpt,
	pub thrust_pow: Fpt,
	pub gravity: Point,
	pub friction: Fpt,
	pub walls: Vec<Vec<Point>>,
	pub winds: Vec<Wind>
}

impl GameData {
/*    pub fn area(&self) -> &Point { &self.area }
    pub fn pos0(&self) -> &Point { &self.pos0 }
    pub fn speed0(&self) -> &Point { &self.speed0 }
    pub fn target_y(&self) -> Fpt { self.target_y }
    pub fn target_x0(&self) -> Fpt { self.target_x0 }
    pub fn target_x1(&self) -> Fpt { self.target_x1 }
    pub fn initial_fuel(&self) -> Fpt { self.initial_fuel }
    pub fn num_asteroids(&self) -> u32 { self.num_asteroids }
    pub fn gravity(&self) -> &Point { &self.gravity }
    pub fn winds(&self) -> &Vec<Wind> { &self.winds }
    pub fn levelling_rot(&self) -> i32 { self.levelling_rot }
    pub fn levelling_speed_x(&self) -> Fpt { self.levelling_speed_x }
    pub fn levelling_speed_y(&self) -> Fpt { self.levelling_speed_y }
    */
	fn ast_default(ast: u32) -> u32 {
		match ast {
			0 => 0,
			1 => 1,
			2 => 3,
			3 => 5,
			4 => 7,
			_ => 3
		}
	}
	fn fuel_default(fuel: u32) -> u32 {
		match fuel {
			0 => 150,
			1 => 300,
			2 => 500,
			3 => 700,
			_ => 500
		}
	}
	fn fuel_increased(fuel: u32) -> u32 {
		match fuel {
			0 => 300,
			1 => 500,
			2 => 750,
			3 => 1000,
			_ => 750
		}
	}
	fn friction_default(friction: u32) -> Fpt {
		match friction {
			0 => 1.0,
			1 => 0.996,
			2 => 0.98,
			3 => 0.97,
			4 => 0.93,
			_ => 0.996
		}
	}
	fn thrust_default(thrust: u32) -> Fpt {
		match thrust {
			0 => 0.10,
			1 => 0.12,
			2 => 0.14,
			3 => 0.24,
			_ => 0.12
		}
	}
	fn gravity_default(grav: u32) -> Fpt {
		match grav {
			0 => 0.0,
			1 => 0.3,
			2 => 1.0,
			3 => 1.5,
			4 => 1.8,
			_ => 1.0
		}
	}

	pub fn simple(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 320;
		let w = wi as Fpt;
		let h = 640 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_default(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(160.0, 50.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 600.0,
			target_x0: 100.0,
			target_x1: 220.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 120.0),
			asteroid_area: Point::new(w, h - 120.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,500) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(),
			walls: vec!()
		}
	}

	pub fn up(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 360;
		let w = wi as Fpt;
		let h = 700 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_increased(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(180.0, 650.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 100.0,
			target_x0: 150.0,
			target_x1: 210.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 0.0),
			asteroid_area: Point::new(w, 580.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,750) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(),
			walls: vec!(vec!(
				pt!(20,40),
				pt!(50,50),
				pt!(20,60)
			),vec!(
				pt!(340,40),
				pt!(310,50),
				pt!(340,60)
			),

			vec!( pt!(80,160), pt!(90,130), pt!(100,160) ),
			vec!( pt!(260,160), pt!(270,130), pt!(280,160) ),

			vec!( pt!(20,260), pt!(30,230), pt!(40,260) ),
			vec!( pt!(170,260), pt!(180,230), pt!(190,260) ),
			vec!( pt!(320,260), pt!(330,230), pt!(340,260) ),

			vec!( pt!(80,360), pt!(90,330), pt!(100,360) ),
			vec!( pt!(260,360), pt!(270,330), pt!(280,360) ),

			vec!( pt!(20,460), pt!(30,430), pt!(40,460) ),
			vec!( pt!(170,460), pt!(180,430), pt!(190,460) ),
			vec!( pt!(320,460), pt!(330,430), pt!(340,460) ),

			vec!( pt!(80,560), pt!(90,530), pt!(100,560) ),
			vec!( pt!(260,560), pt!(270,530), pt!(280,560) ),
			),
		}
	}

	pub fn shifted(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 320;
		let w = wi as Fpt;
		let h = 640 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_default(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(160.0, 50.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 600.0,
			target_x0: 100.0,
			target_x1: 220.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 120.0),
			asteroid_area: Point::new(w, h - 120.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,500) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(
                    Wind::new(vec!(
						pt!(0,100), pt!(w,100),
						pt!(w,200), pt!(0,200)
					),0.06,0),
                    Wind::new(vec!(
						pt!(0,200), pt!(w,200),
						pt!(w,300), pt!(0,300)
					),0.06,180),
                    Wind::new(vec!(
						pt!(0,300), pt!(w,300),
						pt!(w,400), pt!(0,400)
					),0.055,0),
                    Wind::new(vec!(
						pt!(0,400), pt!(w,400),
						pt!(w,500), pt!(0,500)
					),0.055,180),
                    Wind::new(vec!(
						pt!(0,500), pt!(w,500),
						pt!(w,600), pt!(0,600)
					),0.055,90),
                ),
			walls: vec!()
		}
	}

	pub fn tunnel(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 350;
		let w = wi as Fpt;
		let h = 700 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_increased(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(40.0, 30.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 650.0,
			target_x0: 200.0,
			target_x1: 350.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(60.0, 60.0),
			asteroid_area: Point::new(w - 60.0, h - 60.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,750) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(),
			walls: vec!(vec!(
				pt!(0,80),
				pt!(10,80),
				pt!(10,400),
				pt!(0,400)
			),vec!(
				pt!(10,280),
				pt!(280,280),
				pt!(290,350),
				pt!(10,400)
			),vec!(
				pt!(75,80),
				pt!(75,200),
				pt!(100,200),
				pt!(100,80),
			),vec!(
				pt!(100,0),
				pt!(100,200),
				pt!(150,200),
				pt!(170,0),
			),vec!(
				pt!(215,350),
				pt!(240,80),
				pt!(260,80),
				pt!(290,110),
				pt!(290,350),
			),vec!(
				pt!(350,410),
				pt!(85,460),
				pt!(85,550),
				pt!(350,550),
			),vec!(
				pt!(85,550),
				pt!(100,570),
				pt!(190,570),
				pt!(200,550),
			),vec!(
				pt!(0,400),
				pt!(10,400),
				pt!(30,700),
				pt!(0,700),
			),vec!(
				pt!(10,640),
				pt!(200,640),
				pt!(200,700),
				pt!(10,700),
			))
		}
	}

	pub fn windy_pillars(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 300;
		let w = wi as Fpt;
		let h = 700 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_default(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(w / 2.0, 30.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 650.0,
			target_x0: 50.0,
			target_x1: 250.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 150.0),
			asteroid_area: Point::new(w, h - 150.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,500) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(Wind::new(vec!(
						pt!(0,320),
						pt!(w,320),
						pt!(w,450),
						pt!(0,450)
					),0.04,0)),
			walls: vec!(vec!(
				pt!(150,310),
				pt!(180,340),
				pt!(150,370),
				pt!(120,340)
			),vec!(
				pt!(80,410),
				pt!(110,440),
				pt!(80,470),
				pt!(50,440)
			),vec!(
				pt!(220,410),
				pt!(250,440),
				pt!(220,470),
				pt!(190,440)
			))
		}
	}

	pub fn cave(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 320;
		let w = wi as Fpt;
		let h = 640 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_default(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(w / 2.0, 30.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 600.0,
			target_x0: 130.0,
			target_x1: 190.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 150.0),
			asteroid_area: Point::new(w, h - 150.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,500) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(),
			walls: vec!(vec!(
				pt!(0,50),
				pt!(50,90),
				pt!(100,80),
				pt!(130,90),
				pt!(128,100),
				pt!(145,115),
				pt!(155,145),
				pt!(175,175),
				pt!(215,210),
				pt!(180,230),
				pt!(120,250),
				pt!(80,240),
				pt!(0,260),
			),vec!(
				pt!(wi, 300),
				pt!(wi - 50, 320),
				pt!(wi - 100, 330),
				pt!(wi - 120, 350),
				pt!(wi - 175, 360),
				pt!(wi - 220, 370),
				pt!(wi - 205, 390),
				pt!(wi - 160, 400),
				pt!(wi - 50, 420),
				pt!(wi, 425),
			))
		}
	}

	pub fn choice(ast: u32, thrust: u32, fuel: u32, gravity: u32, fric: u32) -> Self {
		let wi = 300;
		let w = wi as Fpt;
		let h = 700 as Fpt;
		let na = Self::ast_default(ast);
		let f = Self::fuel_default(fuel);
		let fr = Self::friction_default(fric);
		let th = Self::thrust_default(thrust);
		let grav = Self::gravity_default(gravity);

		Self {
			area: Point::new(w,h),
			pos0: Point::new(w / 2.0, 30.0),
			speed0: Point::new(0.0, 0.0),
			target_y: 660.0,
			target_x0: 30.0,
			target_x1: 270.0,
			num_asteroids: na,
			asteroid_pos0: Point::new(0.0, 150.0),
			asteroid_area: Point::new(w, h - 150.0),
			levelling_rot: 15,
			levelling_speed_x: 3.5,
			levelling_speed_y: 2.5,
			initial_fuel: f,
			full_fuel: max(f,500) as Fpt,
			thrust_pow: th,
			gravity: Point::new(0.0, 0.06 * grav),
			friction: fr,
			winds: vec!(Wind::new(vec!(
						pt!(200,250),
						pt!(250,250),
						pt!(250,570),
						pt!(200,570)
					),0.04,180),
					Wind::new(vec!(
						pt!(250,250),
						pt!(300,250),
						pt!(300,570),
						pt!(250,570)
					),0.04,0),
					/*Wind::new(vec!(
						pt!(60,250),
						pt!(190,250),
						pt!(190,400),
						pt!(60,400)
					),0.04,90),*/
					Wind::new(vec!(
						pt!(60,400),
						pt!(190,400),
						pt!(190,500),
						pt!(60,500)
					),0.14 + 0.06*grav,270),
					Wind::new(vec!(
						pt!(60,500),
						pt!(190,500),
						pt!(190,570),
						pt!(60,570)
					),0.08,90)
					),
			walls: vec!(vec!(
				pt!(50,200),
				pt!(60,200),
				pt!(60,580),
				pt!(50,580)
			),vec!(
				pt!(190,200),
				pt!(200,200),
				pt!(200,580),
				pt!(190,580)
			))
		}
	}

}
