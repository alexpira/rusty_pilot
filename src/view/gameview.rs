use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use web_sys::{HtmlElement,OffscreenCanvas,ImageBitmap};
use std::cmp::min;

use crate::common::*;
use crate::{attach,pt};
use crate::geom::Point;
// use crate::dlog;
use crate::engine::GameEngine;
use crate::levels::GameData;
use crate::view::menuview::{MenuView,MenuViewData};
use crate::rand::Random;

macro_rules! path {
	($ctx:expr, $vert:expr) => {
		$ctx.begin_path();
		$ctx.move_to($vert[0].x(), $vert[0].y());
		for p in 1..$vert.len() {
			$ctx.line_to($vert[p].x(), $vert[p].y());
		}
		$ctx.close_path();
	}
}

macro_rules! shape {
	($ctx:expr, $col:expr, $vert:expr) => {
		if $vert.len() > 2 {
			$ctx.set_fill_style(&JsValue::from_str($col));
			$ctx.set_stroke_style(&JsValue::from_str($col));
			path!($ctx, $vert);
			$ctx.fill();
		}
	}
}

macro_rules! stroke {
	($ctx:expr, $col:expr, $vert:expr) => {
		if $vert.len() > 2 {
			$ctx.set_stroke_style(&JsValue::from_str($col));
			path!($ctx, $vert);
			$ctx.stroke();
		}
	}
}

const GAME_DIV_STYLE: &str = "z-index: 0; background-color: #000;";
pub struct GameView {
	engine: Rc<RefCell<GameEngine>>,
	root: HtmlElement,
	config: MenuViewData,
	arrow: Rc<ImageBitmap>,
	background: Rc<ImageBitmap>,
}

impl GameView {
	pub fn new(root: HtmlElement, cfg: GameData, md: MenuViewData) -> Self {
		let eng = GameEngine::new(cfg);

		let osc = OffscreenCanvas::new(30,30).expect("OffscreenCanvas creation error");
		let context = osc.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
			.unwrap();
		context.set_fill_style(&JsValue::from_str("#000000"));
		context.fill_rect(0.0, 0.0, 30.0, 30.0);
		shape!(context, "#12fff7", vec!(
			pt!(1.0,5.0),
			pt!(8.0,5.0),
			pt!(21.0,15.0),
			pt!(8.0,25.0),
			pt!(1.0,25.0),
			pt!(14.0,15.0)
		));
		let arrow = osc.transfer_to_image_bitmap().unwrap();

		let mut rng = Random::new();
		let sx = (rng.rand(250) + 500) as u32;
		let sy = (rng.rand(250) + 500) as u32;
		let osc = OffscreenCanvas::new(sx, sy).expect("OffscreenCanvas creation error");
		let sx = sx as f64;
		let sy = sy as f64;
		let context = osc.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
			.unwrap();
		context.set_fill_style(&JsValue::from_str("#000000"));
		context.fill_rect(0.0, 0.0, sx, sy);

		let num = rng.rand(50) + 100;
		for _ in 0..num {
			context.set_fill_style(&JsValue::from_str(match rng.nextbits(2) {
				0 => "#666666",
				1 => "#999999",
				2 => "#cccccc",
				_ => "#ffffff"
			}));
			context.fill_rect(rng.nextfloat() * sx, rng.nextfloat() * sy, 2.0, 2.0);
		}

/*		context.set_fill_style(&JsValue::from_str("#ffffff"));
		context.fill_rect(10.0, 10.0, 2.0, 2.0);
		context.fill_rect(20.0, 20.0, 2.0, 2.0);
		context.fill_rect(30.0, 30.0, 2.0, 2.0);*/
		let bg = osc.transfer_to_image_bitmap().unwrap();

		Self {
			engine: Rc::new(RefCell::new(eng)),
			arrow: Rc::new(arrow),
			background: Rc::new(bg),
			config: md,
			root: root
		}
	}

	fn draw(canvas: &web_sys::HtmlCanvasElement, engine: &GameEngine, background: &ImageBitmap, arrow: &ImageBitmap, opacity: i32) {
		let context = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();

		if opacity < 100 {
			let _ = elem::<HtmlElement>("game").set_attribute("style", format!("opacity: {:.2}; {}", opacity as Fpt / 100.0, GAME_DIV_STYLE).as_str());
		}

		let cw = canvas.width().into();
		let ch = canvas.height().into();
		let _ = context.reset_transform();
		// context.set_fill_style(&JsValue::from_str("#000000"));
		// context.fill_rect(0.0, 0.0, cw, ch);
		let pat = context.create_pattern_with_image_bitmap(background, "repeat").unwrap().unwrap();

		if engine.scrollable() {
			let vpos = engine.viewport_pos();
			let vx = vpos.x();
			let vy = vpos.y();

			let _ = context.translate(-vx, -vy);
			context.set_fill_style(&pat);
			context.fill_rect(vx, vy, cw, ch);

			if vx < 0.0 {
				shape!(context, "#a83e3e", vec![pt!(vx,vy), pt!(0,vy), pt!(0,ch+vy), pt!(vx,ch+vy)]);
			}
			if vy < 0.0 {
				shape!(context, "#a83e3e", vec![pt!(vx,vy), pt!(vx,0), pt!(cw+vx,0), pt!(cw+vx,vy)]);
			}
			let out = vx + cw - engine.area_width();
			if out > 0.0 {
				shape!(context, "#a83e3e", vec![pt!(cw-out+vx,vy), pt!(cw+vx,vy), pt!(cw+vx,ch+vy), pt!(cw-out+vx,ch+vy)]);
			}
			let out = vy + ch - engine.area_height();
			if out > 0.0 {
				shape!(context, "#a83e3e", vec![pt!(vx,ch-out+vy), pt!(vx,ch+vy), pt!(cw+vx,ch+vy), pt!(cw+vx,ch-out+vy)]);
			}
		} else {
			context.set_fill_style(&pat);
			context.fill_rect(0.0, 0.0, cw, ch);
		}

		let wind_step = engine.get_wind_step();
		engine.iter_winds(|w,trig| {
			context.save();
			stroke!(context, "#12fff7", w.shape());
			path!(context, w.shape());
			let pat = context.create_pattern_with_image_bitmap(arrow, "repeat").unwrap().unwrap();
			let dir = w.direction();
			let _ = context.rotate(trig.rad(dir));
			if wind_step > 0 {
				let _ = context.translate(10.0, 0.0);
			}
			let shp : Vec<Point> = w.shape().iter().map(|p| {
				let mut p = trig.rot(p, -dir);
				if wind_step > 0 {
					p.add(&pt!(-10.0,0.0));
				}
				p
			}).collect();

			context.set_fill_style(&pat);
			path!(context, shp);
			context.fill();
			context.restore();
		});

		for o in engine.obs_shape().iter() {
			shape!(context, "#a83e3e", o);
		}
		for o in engine.aster_shape().iter() {
			shape!(context, "#b88b2c", o);
		}
		let mut ship_col = "#42a4f5";
		if engine.block_alert() {
			ship_col = "#e05f38";
		}
		shape!(context, ship_col, engine.ship_shape());
		shape!(context, "#8fffc3", engine.land_shape());
		engine.iter_part(|p| {
			context.set_global_alpha(p.alpha());
			context.set_fill_style(p.color());
			let pos = p.position();
			context.fill_rect(pos.x()-1.0, pos.y()-1.0, 2.0, 2.0);
		});
		context.set_global_alpha(1.0);

		/*
		let areaw = engine.viewport_width();
		let areah = engine.viewport_height();
		let hud_y0 : f64 = areah - 15.0;
		let hud_y1 : f64 = areah - 5.0;
		let hud_x0 : f64 = 30.0;
		let hud_x1 : f64 = areaw - 30.0;
		let hud_lwidth : f64 = hud_y1 - hud_y0;
		let hud_lspacing : f64 = 15.0;
		let hud_tick_size : f64 = 3.0;
		*/
		let pos = engine.ship_pos();
		let hud_y0 : f64 = pos.y() + 20.0;
		let hud_y1 : f64 = pos.y() + 23.0;
		let hud_x0 : f64 = pos.x() - 25.0;
		let hud_x1 : f64 = pos.x() + 25.0;
		let hud_lspacing : f64 = 3.0;
		let hud_tick_size : f64 = 0.0;

		let hud_width : f64 = hud_x1 - hud_x0;
		let hud_lwidth : f64 = hud_y1 - hud_y0;

		if engine.has_collided() {
			context.set_fill_style(&JsValue::from_str("#f00"));
			context.begin_path();
			context.move_to(hud_x1 + hud_lspacing + hud_lwidth, hud_y1);
			context.line_to(hud_x1 + hud_lspacing + hud_lwidth, hud_y0);
			context.line_to(hud_x1 + hud_lspacing, hud_y0);
			context.line_to(hud_x1 + hud_lspacing, hud_y1);
			context.close_path();
			context.fill();
		} else if engine.has_landed() {
			context.set_fill_style(&JsValue::from_str("#0f0"));
			context.begin_path();
			context.move_to(hud_x1 + hud_lspacing + hud_lwidth, hud_y1);
			context.line_to(hud_x1 + hud_lspacing + hud_lwidth, hud_y0);
			context.line_to(hud_x1 + hud_lspacing, hud_y0);
			context.line_to(hud_x1 + hud_lspacing, hud_y1);
			context.close_path();
			context.fill();
		}

		if engine.is_level() {
			context.set_fill_style(&JsValue::from_str("#0f0"));
			context.begin_path();
			context.move_to(hud_x0 - (hud_lwidth + hud_lspacing), hud_y1);
			context.line_to(hud_x0 - (hud_lwidth + hud_lspacing), hud_y0);
			context.line_to(hud_x0 - hud_lspacing, hud_y0);
			context.line_to(hud_x0 - hud_lspacing, hud_y1);
			context.close_path();
			context.fill();
		}

		let mut fuel_col = "#0f0";
		if engine.fuel_warn() {
			fuel_col = "#f00";
		}
		context.set_fill_style(&JsValue::from_str(fuel_col));
		context.begin_path();
		let fuel = engine.fuel_sz(hud_width) + hud_x0;
		context.move_to(hud_x0, hud_y1);
		context.line_to(fuel, hud_y1);
		context.line_to(fuel, hud_y0);
		context.line_to(hud_x0, hud_y0);
		context.close_path();
		context.fill();

		context.set_stroke_style(&JsValue::from_str("#fff"));
		context.begin_path();
		context.move_to(hud_x0, hud_y1);
		context.line_to(hud_x1, hud_y1);
		context.move_to(hud_x0, hud_y0);
		context.line_to(hud_x1, hud_y0);
		for x in vec![0.25, 0.5, 0.75].iter() {
			let x = hud_x0 + (hud_width * x);
			context.move_to(x, hud_y0);
			context.line_to(x, hud_y0 - hud_tick_size);
			context.move_to(x, hud_y1);
			context.line_to(x, hud_y1 + hud_tick_size);
		}
		context.stroke();

		//context.translate(self.engine.pos.x(), gd.pos.y());
		//context.rotate(self.engine.trig.rad(gd.rot));
		//let _ = context.reset_transform();
	}

	fn setup_html(&self) {
		let swidth = window().inner_width().unwrap().as_f64().unwrap() as u32 - 6;
		let sheight = window().inner_height().unwrap().as_f64().unwrap() as u32 - 6;

		let eng = (*self.engine).borrow();
		let engw = eng.viewport_width();
		let engh = eng.viewport_height();
		let widthstep: u32 = (engw / 10.0) as u32;
		let heightstep: u32 = (engh / 10.0) as u32;
		let ratio = min(swidth / widthstep, sheight / heightstep);
		let cw = widthstep * ratio;
		let ch = heightstep * ratio;

		self.root.set_inner_html(format!("\
			<div id=\"console\" class=\"log\" \
				style=\"\
					display: block;
				\"></div>\
			<div id=\"ctrl\" class=\"full\" \
					style=\"z-index: 200;\">\
				<!-- div id=\"thrust\"></div>\
				<div id=\"roll\">\
					<div id=\"roll_r\"></div>\
					<div id=\"roll_l\"></div>\
				</div -->\
			</div>\
			<div id=\"game\" class=\"full center\" \
				style=\"{}\">\
			<canvas id=\"canvas\" \
				style=\"width: {}px; height: {}px; border: 3px solid #fff;\" \
				width=\"{}\" height=\"{}\"/>\
			</div>", GAME_DIV_STYLE, cw, ch, engw as u32, engh as u32).as_str());
	}

	fn setup_keyboard_events(&self) {
		let engref = Rc::clone(&self.engine);
		let keyfn = move |event: web_sys::KeyboardEvent| {
			let mut engine = (*engref).borrow_mut();
			let kc = event.key_code();
			let mut lrot: Option<bool> = None;
			let mut rrot: Option<bool> = None;

			// dlog!(format!("{} {}", event.type_().as_str(), event.key_code()).as_str());
			match event.type_().as_str() {
				"keydown" => {
					if kc == web_sys::KeyEvent::DOM_VK_UP || kc == web_sys::KeyEvent::DOM_VK_SPACE {
						event.prevent_default();
						engine.set_thrust(true);
					} else if kc == web_sys::KeyEvent::DOM_VK_LEFT {
						event.prevent_default();
						lrot = Some(true);
					} else if kc == web_sys::KeyEvent::DOM_VK_RIGHT {
						event.prevent_default();
						rrot = Some(true);
					}
				},
				"keyup" => {
					if kc == web_sys::KeyEvent::DOM_VK_UP || kc == web_sys::KeyEvent::DOM_VK_SPACE {
						event.prevent_default();
						engine.set_thrust(false);
					} else if kc == web_sys::KeyEvent::DOM_VK_LEFT {
						event.prevent_default();
						lrot = Some(false);
					} else if kc == web_sys::KeyEvent::DOM_VK_RIGHT {
						event.prevent_default();
						rrot = Some(false);
					}
				},
				_ => {}
			}
			engine.set_rotation(lrot, rrot);
		};

		attach!("keydown", keyfn.clone());
		attach!("keyup", keyfn);
	}

	#[allow(dead_code)]
	fn setup_mouse_events(&self) {
		elem::<HtmlElement>("ctrl").request_pointer_lock();
		// let _ = js_sys::eval("document.getElementById('ctrl').requestPointerLock({unadjustedMovement: true})");
		let engref = Rc::clone(&self.engine);
		let mousefn = move |event: web_sys::MouseEvent| {
			event.prevent_default();
			event.stop_propagation();
			let mut engine = (*engref).borrow_mut();
			let evt = event.type_();
			// dlog!(format!("{} {} {}", evt.as_str(), event.movement_x(), event.movement_y()).as_str());
			match evt.as_str() {
				"mousemove" => {
					let mx = event.movement_x();
					let my = event.movement_y();
					let delta = if i32::abs(mx) > i32::abs(my) { mx } else { my };
					let delta = if i32::abs(delta) <= 20 { delta } else if delta < 0 { -20 } else { 20 };
					engine.rotate(delta);
				},
				"mousedown" => {
					engine.set_thrust(true);
				},
				"mouseup" => {
					engine.set_thrust(false);
				},
				_ => {}
			}
		};

		attach!("ctrl", "mousemove", mousefn.clone());
		attach!("ctrl", "mousedown", mousefn.clone());
		attach!("ctrl", "mouseup", mousefn);
	}

	fn setup_pointer_events(&self) {
		elem::<HtmlElement>("ctrl").request_pointer_lock();

		let swidth = window().inner_width().unwrap().as_f64().unwrap() as u32;
		let touches : Rc<RefCell<HashMap<i32,i32>>> = Rc::new(RefCell::new(HashMap::new()));

		let vsplit = (swidth / 2) as i32;

		let mut ptcache : Option<bool> = None;
		let mut is_touch = move |ptype: &String| {
			// "mouse" | "pen" | "touch"
			*ptcache.get_or_insert_with(|| { ptype.len() == 5 && ptype.chars().next().unwrap() == 't' })
		};

		let open_pointer_fn = |event: &web_sys::PointerEvent| {
			event.prevent_default();
			event.stop_propagation();
			let evt = event.type_();
			let id = event.pointer_id();
			let x = event.page_x();
			let ptype = event.pointer_type();
			// dlog!(format!("{} {} {} {:.2} {:.2}", event.pointer_type(), evt.as_str(), event.pointer_id(), event.page_x(), event.page_y()).as_str());
			/* event.get_coalesced_events().for_each(&mut |jev, ix, ar| {
				let ce = jev.dyn_into::<web_sys::PointerEvent>().map_err(|_| ()).unwrap();
				let evt = ce.type_();
				dlog!(format!("{} - {} {} {} {} {}", ix, evt.as_str(), ce.pointer_id(), ce.page_x(), ce.movement_x(), ce.movement_y()).as_str());
			}); */

			(evt,id,x,ptype)
		};

		let engref = Rc::clone(&self.engine);
		let touchref = Rc::clone(&touches);
		let ptrmovefn = move |event: web_sys::PointerEvent| {
			let (_,id,x,pt) = open_pointer_fn(&event);
			let mut touches = (*touchref).borrow_mut();
			let mut engine = (*engref).borrow_mut();
			let mut delta : i32 = 0;
			let touch = is_touch(&pt);
			if x >= vsplit {
				if touch {
					delta += event.movement_y();
				} else {
					let mx = event.movement_x();
					let my = event.movement_y();
					delta = if i32::abs(mx) > i32::abs(my) { mx } else { my };
					delta = if i32::abs(delta) <= 20 { delta } else if delta < 0 { -20 } else { 20 };
				}
			}
			if delta != 0 {
				engine.rotate(delta);
			}
			if touch {
				touches.insert(id, x);
				engine.set_thrust(touches.values().any(|e| { e < &vsplit }));
			}
		};

		let engref = Rc::clone(&self.engine);
		let touchref = Rc::clone(&touches);
		let ptrstartfn = move |event: web_sys::PointerEvent| {
			let (_,id,x,pt) = open_pointer_fn(&event);
			let mut touches = (*touchref).borrow_mut();
			let mut engine = (*engref).borrow_mut();
			if is_touch(&pt) {
				engine.set_block_alert(false);
				touches.insert(id, x);
				engine.set_thrust(touches.values().any(|e| { e < &vsplit }));
			} else {
				engine.set_thrust(true);
			}
		};

		let engref = Rc::clone(&self.engine);
		let touchref = Rc::clone(&touches);
		let ptrendfn = move |event: web_sys::PointerEvent| {
			let (_,id,_,pt) = open_pointer_fn(&event);
			let mut touches = (*touchref).borrow_mut();
			let mut engine = (*engref).borrow_mut();
			if is_touch(&pt) {
				touches.remove(&id);
				engine.set_thrust(touches.values().any(|e| { e < &vsplit }));
			} else {
				engine.set_thrust(false);
			}
		};

		let engref = Rc::clone(&self.engine);
		let touchref = Rc::clone(&touches);
		let ptrcancelfn = move |event: web_sys::PointerEvent| {
			let (_,_,_,pt) = open_pointer_fn(&event);
			let mut touches = (*touchref).borrow_mut();
			let mut engine = (*engref).borrow_mut();
			if is_touch(&pt) {
				engine.set_block_alert(true);
				touches.clear();
				engine.set_thrust(touches.values().any(|e| { e < &vsplit }));
			}
		};

		attach!("ctrl", "pointermove", ptrmovefn);

//		attach!("ctrl", "pointerenter", ptrstartfn.clone());
		attach!("ctrl", "pointerdown", ptrstartfn);

//		attach!("ctrl", "pointerleave", ptrendfn.clone());
//		attach!("ctrl", "pointerout", ptrendfn.clone());
//		attach!("ctrl", "pointerover", ptrendfn.clone());
		attach!("ctrl", "pointerup", ptrendfn);

		attach!("ctrl", "pointercancel", ptrcancelfn);
	}

	#[allow(dead_code)]
	fn setup_touch_events(&self) {
		let swidth = window().inner_width().unwrap().as_f64().unwrap() as u32;
		let vsplit = (swidth / 2) as i32;
		let touches : Rc<RefCell<HashMap<i32,i32>>> = Rc::new(RefCell::new(HashMap::new()));
		let engref = Rc::clone(&self.engine);
		let touchfn = move |event: web_sys::TouchEvent| {
			event.prevent_default();
			event.stop_propagation();
			let evt = event.type_();
			let mut touches = (*touches).borrow_mut();
			let mut engine = (*engref).borrow_mut();
			let mut thrusting = false;
			let blocked = evt == "touchcancel";

			if blocked {
				touches.clear()
			} else {
				let tl = event.touches();
				// elem::<HtmlElement>("console").set_inner_html("");
				// dlog!(format!("{} {}", event.type_().as_str(), tl.length()).as_str());
				for n in 0..tl.length() {
					let t = tl.item(n).unwrap();
					let id = t.identifier();
					let y = t.page_y();
					let thrust = t.page_x() < vsplit;
					thrusting = thrusting || thrust;

					// dlog!(format!("ptr {} {:.2},{:.2} {}", id, t.page_x(), t.page_y(), thrusting).as_str());
					let mut delta : i32 = 0;
					match evt.as_str() {
						"touchend" => {
							touches.remove(&id);
						},
						"touchstart" => {
							touches.insert(id, y);
						}
						_ => {
							if !thrust {
								if let Some(last) = touches.get(&id) {
									delta += y - last;
								}
							}
							touches.insert(id, y);
						}
					}
					if delta != 0 {
						engine.rotate(delta);
						// dlog!(format!("roll {} {}", delta, engine.rot).as_str());
					}
				}
			}
			engine.set_thrust(thrusting);
			engine.set_block_alert(blocked);
		};

		attach!("ctrl", "touchmove", touchfn.clone());
		attach!("ctrl", "touchstart", touchfn.clone());
		attach!("ctrl", "touchcancel", touchfn.clone());
		attach!("ctrl", "touchend", touchfn);
	}

	fn setup_triggers(&mut self) {
		const ENGINE_STEP_MS : u64 = 25u64;
		let animf = Rc::new(RefCell::new(None));
		let animfc = animf.clone();
		let rootc = self.root.clone();
		let config = self.config.clone();

		let engref = Rc::clone(&self.engine);
		let arrow = Rc::clone(&self.arrow);
		let background = Rc::clone(&self.background);
		let canvas = canvas();

		let mut fading = -1;
		let mut last_engine_run = js_sys::Date::now() as u64;

		*animfc.borrow_mut() = Some(Closure::new(move || {
			let mut engine = (*engref).borrow_mut();
			let now = js_sys::Date::now() as u64;

			if engine.block_alert() {
				last_engine_run = now;
			} else {
				let target_step = now - ENGINE_STEP_MS;
				while last_engine_run < target_step {
					last_engine_run += ENGINE_STEP_MS;
					if fading < 0 && engine.finished() {
						fading = 0;
					} else if fading > 100 {
						let _ = animf.borrow_mut().take();
						Self::cleanup();

						let _ = elem::<HtmlElement>("game").set_attribute("style",GAME_DIV_STYLE);
						rootc.set_inner_html("<div id=\"game\" class=\"full center\" style=\"z-index: 0; background-color: #000;\">");
						let view = MenuView::new(target_elem(), config.clone());
						view.show();

						return;
					} else if fading >= 0 {
						fading += 1;
					}

					engine.move_step();
				}
			}

			Self::draw(&canvas, &engine, &background, &arrow, 100 - fading);
			request_animation_frame(animf.borrow().as_ref().unwrap());
		}));

		request_animation_frame(animfc.borrow().as_ref().unwrap());
	}

	pub fn show(&mut self) {
		self.setup_html();
		self.setup_keyboard_events();
		// self.setup_mouse_events();
		// self.setup_touch_events();
		self.setup_pointer_events();
		self.setup_triggers();
	}

	fn cleanup() {
		document().exit_pointer_lock();
	}
}

/*
	let canvas = document().get_element_by_id("canvas").unwrap();
	let canvas: web_sys::HtmlCanvasElement = canvas
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.map_err(|_| ())
		.unwrap();

	let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
		log(format!("Hello {} {}", event.client_x(), event.client_y()).as_str());
	});
	canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).expect("Cannot attach event");
	closure.forget();

	let window = window();
	dlog!(format!("win size {} {}",
			window.inner_width().unwrap().as_f64().unwrap() as u32,
			window.inner_height().unwrap().as_f64().unwrap() as u32
		).as_str());

	canvas.set_width(window.inner_width().unwrap().as_f64().unwrap().to_bits() as u32);
	canvas.set_height(window.inner_height().unwrap().as_f64().unwrap().to_bits() as u32);
	dlog!(format!("wwww {} {}", u32::from(canvas.width()), u32::from(canvas.height())).as_str());
//	run();
//	*/

