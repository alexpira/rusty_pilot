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
			$ctx.stroke();
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
	arrow: Rc<ImageBitmap>
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
		let bmp = osc.transfer_to_image_bitmap().unwrap();

		Self {
			engine: Rc::new(RefCell::new(eng)),
			arrow: Rc::new(bmp),
			config: md,
			root: root
		}
	}

	fn draw(engine: &GameEngine, arrow: &ImageBitmap, opacity: i32) {
		let canvas = canvas();

		let context = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();

		if opacity < 100 {
			let _ = elem::<HtmlElement>("game").set_attribute("style", format!("opacity: {:.2}; {}", opacity as Fpt / 100.0, GAME_DIV_STYLE).as_str());
		}

		context.set_fill_style(&JsValue::from_str("#000000"));
		context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

		let step = engine.get_step();
		engine.iter_winds(|w,trig| {
			context.save();
			stroke!(context, "#12fff7", w.shape());
			path!(context, w.shape());
			let pat = context.create_pattern_with_image_bitmap(arrow, "repeat").unwrap().unwrap();
			let dir = w.direction();
			let _ = context.rotate(trig.rad(dir));
			if step > 0 {
				let _ = context.translate(10.0, 0.0);
			}
			let shp : Vec<Point> = w.shape().iter().map(|p| {
				let mut p = trig.rot(p, -dir);
				if step > 0 {
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
			context.set_fill_style(&JsValue::from_str(p.color()));
			let pos = p.position();
			context.fill_rect(pos.x()-1.0, pos.y()-1.0, 2.0, 2.0);
		});
		context.set_global_alpha(1.0);

		/*
		let areaw = engine.area_width();
		let areah = engine.area_height();
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
		let engw = eng.area_width();
		let engh = eng.area_height();
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

/*		let ptrfn = move |event: web_sys::PointerEvent| {
			event.prevent_default();
			event.stop_propagation();
			dlog!(format!("{} {} {:.2} {:.2}", event.type_().as_str(), event.pointer_id(), event.page_x(), event.page_y()).as_str());
		};
		attach!("ctrl", "pointerover", ptrfn.clone());
		attach!("ctrl", "pointerenter", ptrfn.clone());
		attach!("ctrl", "pointerdown", ptrfn.clone());
		attach!("ctrl", "pointermove", ptrfn.clone());
		attach!("ctrl", "pointercancel", ptrfn.clone());
		attach!("ctrl", "pointerout", ptrfn.clone());
		attach!("ctrl", "pointerleave", ptrfn.clone()); */
	}

	fn setup_triggers(&mut self) {
		let fading = Rc::new(RefCell::new(-1));
		let animf = Rc::new(RefCell::new(None));
		let animfc = animf.clone();

		let faderef = fading.clone();
		let engref = Rc::clone(&self.engine);
		let arrow = Rc::clone(&self.arrow);
		*animfc.borrow_mut() = Some(Closure::new(move || {
			let engine = (*engref).borrow();
			let fading = (*faderef).borrow();

			if *fading > 100 {
				let _ = animf.borrow_mut().take();
				return;
			}
			Self::draw(&engine, &arrow, 100 - *fading);
			request_animation_frame(animf.borrow().as_ref().unwrap());
		}));

		request_animation_frame(animfc.borrow().as_ref().unwrap());


		let timer: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

		let faderef = fading.clone();
		let engref = Rc::clone(&self.engine);
		let rootc = self.root.clone();
		let timerc = timer.clone();
		let config = self.config.clone();
		let process = Closure::<dyn Fn()>::new(move || {
			let mut engine = (*engref).borrow_mut();
			if engine.block_alert() {
				return;
			}
			let mut fading = (*faderef).borrow_mut();

			if *fading < 0 && engine.finished() {
				*fading = 0;
			} else if *fading > 100 {
				window().clear_interval_with_handle(timerc.borrow().unwrap());
				Self::cleanup();
				let _ = elem::<HtmlElement>("game").set_attribute("style",GAME_DIV_STYLE);
				rootc.set_inner_html("<div id=\"game\" class=\"full center\" style=\"z-index: 0; background-color: #000;\">");
				let view = MenuView::new(target_elem(), config.clone());
				view.show();

				return;
			} else if *fading >= 0 {
				*fading += 1;
			}

			engine.move_step();
		});
		if let Ok(t_handle) = window().set_interval_with_callback_and_timeout_and_arguments_0(process.as_ref().unchecked_ref(),25) {
			*timer.borrow_mut() = Some(t_handle);
		}

		process.forget();
	}

	pub fn show(&mut self) {
		self.setup_html();
		self.setup_keyboard_events();
		self.setup_mouse_events();
		self.setup_touch_events();
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

