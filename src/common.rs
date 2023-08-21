use wasm_bindgen::prelude::*;

pub type Fpt = f64;

#[macro_export]
macro_rules! dlog {
	( $text:expr ) => {
		let el = document().create_element("p").unwrap();
		el.set_text_content(Some($text));
        let csl = document().get_element_by_id("console").unwrap();
		let _ = csl.append_child(&el);
		while csl.child_element_count() > 60 {
			csl.first_element_child().unwrap().remove();
		}
	}
}

#[macro_export]
macro_rules! attach {
	( $element:expr, $event:expr, $code:expr ) => {
		let closure = Closure::<dyn FnMut(_)>::new($code);
		let el = document().get_element_by_id( $element ).unwrap();
		el.add_event_listener_with_callback($event, closure.as_ref().unchecked_ref()).expect("Cannot attach event");

		closure.forget();
	};
	( $event:expr, $code:expr ) => {
		let closure = Closure::<dyn FnMut(_)>::new($code);
		document().add_event_listener_with_callback($event, closure.as_ref().unchecked_ref()).expect("Cannot attach event");
		closure.forget();
	};
}

#[macro_export]
macro_rules! deg2rad {
	($a:expr) => { ($a as f64) * f64::consts::PI / 180.0 }
}
#[macro_export]
macro_rules! min {
	($v1:expr,$v2:expr,$v3:expr) => { min($v1,min($v2,$v3)) }
}
#[macro_export]
macro_rules! max {
	($v1:expr,$v2:expr,$v3:expr) => { max($v1,max($v2,$v3)) }
}

#[macro_export]
macro_rules! pt {
	($x:expr,$y:expr) => { Point::new($x as Fpt, $y as Fpt) }
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}

pub fn window() -> web_sys::Window {
	web_sys::window().expect("no global window exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("failed requestAnimationFrame");
}

pub fn document() -> web_sys::Document {
	window()
		.document()
		.expect("no document")
}

pub fn body() -> web_sys::HtmlElement {
	document().body().expect("no body")
}

pub fn elem<T: wasm_bindgen::JsCast>(id: &str) -> T {
	document().get_element_by_id(id).unwrap()
		.dyn_into::<T>()
		.map_err(|_| ())
		.unwrap()
}

pub fn target_elem() -> web_sys::HtmlElement {
	body()
}

pub fn canvas() -> web_sys::HtmlCanvasElement {
	elem::<web_sys::HtmlCanvasElement>("canvas")
}

