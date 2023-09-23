
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use std::rc::Rc;
use std::cell::RefCell;

use crate::view::gameview::GameView;
use crate::levels::GameData;
use crate::rand::Random;
use crate::common::*;
use crate::attach;

#[derive(Clone)]
pub struct MenuViewData {
	map: u32,
	asteroids: u32,
	fuel: u32,
	thrust: u32,
	gravity: u32,
	friction: u32,
}

pub struct MenuView {
    root: HtmlElement,
    rng: Rc<RefCell<Random>>,
	data: Rc<RefCell<MenuViewData>>
}

impl MenuView {
    pub fn new(root: HtmlElement, data: MenuViewData) -> Self {
        Self {
            root: root,
            rng: Rc::new(RefCell::new(Random::new())),
			data: Rc::new(RefCell::new(data)),
        }
    }
	fn setup_html(&self) {
		let data = (*self.data).borrow();
		self.root.set_inner_html(format!("\
			<div class=\"menu full center\">\
				<div class=\"menuheader\">\
					<div class=\"gametitle\">Rusty Pilot</div>\
					<div class=\"subtitle\">\
						A game by\
					</div>\
					<div class=\"subtitle\">\
						<svg viewBox=\"0 0 350 200\" style=\"width: 7em; height: 4em; background-color: #000\">\
							<path d=\"M310 0 A 40 40 0 0 1 350 40 L350 160 A 40 40 0 0 1 310 200 L40 200 A 40 40 0 0 1 0 160 L0 40 A 40 40 0 0 1 40 0 Z\" style=\"fill:#f7ff57\"/>\
							<path d=\"M60 160 l100 -130 l80 0 a40 40 0 0 1 0 80 l0 50 l-50 0 l0 -50 l-40 0 l-40 50 Z\" style=\"fill:#000\"/>\
						</svg>\
					</div>\
					<div class=\"subtitle\">\
						Alessandro Pira<br>productions\
					</div>\
				</div>\
				<div id=\"map\" class=\"menuitem\">{}</div>\
				<div class=\"menusep\"></div>
				<div id=\"asteroids\" class=\"menuitem\">{}</div>\
				<div id=\"gravity\" class=\"menuitem\">{}</div>\
				<div id=\"friction\" class=\"menuitem\">{}</div>\
				<div class=\"menusep\"></div>
				<div id=\"fuel\" class=\"menuitem\">{}</div>\
				<div id=\"thrust\" class=\"menuitem\">{}</div>\
				<div class=\"buttons\">\
					<div id=\"rand\" class=\"button\">\
						RANDOM\
					</div>\
					<div id=\"play\" class=\"button\">\
						PLAY\
					</div>\
				</div>\
			</div>\
			<div id=\"console\" class=\"log\" \
				style=\"\
					display: block;
				\"></div>\
			",
			Self::label_map(data.map),
			Self::label_asteroids(data.asteroids),
			Self::label_gravity(data.gravity),
			Self::label_friction(data.friction),
			Self::label_fuel(data.fuel),
			Self::label_thrust(data.thrust)
			).as_str());
	}

	pub fn default_data() -> MenuViewData {
		MenuViewData {
			map: 0u32,
			gravity: 2u32,
			friction: 1u32,
			asteroids: 2u32,
			fuel: 2u32,
			thrust: 1u32,
		}
	}
	pub fn rand_data(rng: &mut Random) -> MenuViewData {
		MenuViewData {
			map: rng.rand(8) as u32,
			gravity: rng.rand(5) as u32,
			friction: rng.rand(5) as u32,
			asteroids: rng.rand(5) as u32,
			fuel: rng.rand(4) as u32,
			thrust: rng.rand(4) as u32,
		}
	}

	fn label_map(v: u32) -> &'static str {
		match v {
			0 => "<span>Map: SIMPLE</span>",
			1 => "<span>Map: CAVE</span>",
			2 => "<span>Map: WINDY PILLARS</span>",
			3 => "<span>Map: TUNNEL</span>",
			4 => "<span>Map: SHIFTED</span>",
			5 => "<span>Map: CHOICE</span>",
			6 => "<span>Map: UP</span>",
			7 => "<span>Map: HUGE</span>",
			_ => "<span>Map: SIMPLE</span>",
		}
	}
	fn label_asteroids(v: u32) -> &'static str {
		match v {
			0 => "<span>Asteroids: OFF</span>",
			1 => "<span>Asteroids: ONE</span>",
			2 => "<span>Asteroids: FEW</span>",
			3 => "<span>Asteroids: SOME</span>",
			4 => "<span>Asteroids: A LOT</span>",
			_ => "<span>Asteroids: FEW</span>",
		}
	}
	fn label_thrust(v: u32) -> &'static str {
		match v {
			0 => "<span>Engine: POOR</span>",
			1 => "<span>Engine: NORMAL</span>",
			2 => "<span>Engine: GOOD</span>",
			3 => "<span>Engine: AFTERBURN</span>",
			_ => "<span>Engine: NORMAL</span>",
		}
	}
	fn label_fuel(v: u32) -> &'static str {
		match v {
			0 => "<span>Fuel: PANIC</span>",
			1 => "<span>Fuel: LOW</span>",
			2 => "<span>Fuel: AVERAGE</span>",
			3 => "<span>Fuel: ENOUGH</span>",
			_ => "<span>Fuel: AVERAGE</span>",
		}
	}
	fn label_gravity(v: u32) -> &'static str {
		match v {
			0 => "<span>Gravity: OFF</span>",
			1 => "<span>Gravity: MOON</span>",
			2 => "<span>Gravity: NORMAL</span>",
			3 => "<span>Gravity: HEAVY</span>",
			4 => "<span>Gravity: INSANE</span>",
			_ => "<span>Gravity: NORMAL</span>",
		}
	}
	fn label_friction(v: u32) -> &'static str {
		match v {
			0 => "<span>Friction: SPACE</span>",
			1 => "<span>Friction: AIR</span>",
			2 => "<span>Friction: WATER</span>",
			3 => "<span>Friction: OIL</span>",
			4 => "<span>Friction: MERCURY</span>",
			_ => "<span>Friction: AIR</span>",
		}
	}

	fn setup_events(&self, evt: &str) {
		let data = Rc::clone(&self.data);
		attach!("map", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).map;
			let v = (v + 1) % 8;
			(*data).map = v;
			elem::<HtmlElement>("map").set_inner_html(Self::label_map(v));
        });

		let data = Rc::clone(&self.data);
		attach!("asteroids", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).asteroids;
			let v = (v + 1) % 5;
			(*data).asteroids = v;
			elem::<HtmlElement>("asteroids").set_inner_html(Self::label_asteroids(v));
        });

		let data = Rc::clone(&self.data);
		attach!("fuel", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).fuel;
			let v = (v + 1) % 4;
			(*data).fuel = v;
			elem::<HtmlElement>("fuel").set_inner_html(Self::label_fuel(v));
        });

		let data = Rc::clone(&self.data);
		attach!("thrust", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).thrust;
			let v = (v + 1) % 4;
			(*data).thrust = v;
			elem::<HtmlElement>("thrust").set_inner_html(Self::label_thrust(v));
        });

		let data = Rc::clone(&self.data);
		attach!("gravity", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).gravity;
			let v = (v + 1) % 5;
			(*data).gravity = v;
			elem::<HtmlElement>("gravity").set_inner_html(Self::label_gravity(v));
        });

		let data = Rc::clone(&self.data);
		attach!("friction", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut data = data.borrow_mut();
			let v = (*data).friction;
			let v = (v + 1) % 5;
			(*data).friction = v;
			elem::<HtmlElement>("friction").set_inner_html(Self::label_friction(v));
        });

		let data = Rc::clone(&self.data);
        let rng = Rc::clone(&self.rng);
		attach!("rand", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let mut rng = rng.borrow_mut();
            data.replace(Self::rand_data(&mut rng));
            let data = data.borrow();
			elem::<HtmlElement>("map").set_inner_html(Self::label_map(data.map));
			elem::<HtmlElement>("asteroids").set_inner_html(Self::label_asteroids(data.asteroids));
			elem::<HtmlElement>("fuel").set_inner_html(Self::label_fuel(data.fuel));
			elem::<HtmlElement>("thrust").set_inner_html(Self::label_thrust(data.thrust));
			elem::<HtmlElement>("gravity").set_inner_html(Self::label_gravity(data.gravity));
			elem::<HtmlElement>("friction").set_inner_html(Self::label_friction(data.friction));
        });

		let data = Rc::clone(&self.data);
		attach!("play", evt, move |event: web_sys::Event| {
			event.prevent_default();
			let data = data.borrow();
			let ast = (*data).asteroids;
			let fuel = (*data).fuel;
			let fr = (*data).friction;
			let th = (*data).thrust;
			let gravity = (*data).gravity;
			let gd : GameData = match (*data).map {
				0 => GameData::simple(ast, th, fuel, gravity, fr),
				1 => GameData::cave(ast, th, fuel, gravity, fr),
				2 => GameData::windy_pillars(ast, th, fuel, gravity, fr),
				3 => GameData::tunnel(ast, th, fuel, gravity, fr),
				4 => GameData::shifted(ast, th, fuel, gravity, fr),
				5 => GameData::choice(ast, th, fuel, gravity, fr),
				6 => GameData::up(ast, th, fuel, gravity, fr),
				7 => GameData::huge(ast, th, fuel, gravity, fr),
				_ => GameData::simple(ast, th, fuel, gravity, fr),
			};

			Self::to_game(gd, (*data).clone());
        });
	}

	fn to_game(gd : GameData, cfg: MenuViewData) {
    	let mut view = GameView::new(target_elem(), gd, cfg);
	    view.show();
	}

    pub fn show(&self) {
		self.setup_html();
		self.setup_events("click");
		self.setup_events("touchstart");
    }
}

