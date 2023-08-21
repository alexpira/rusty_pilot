use wasm_bindgen::prelude::*;

use crate::common::*;
use crate::view::menuview::MenuView;

mod common;
mod rand;
mod geom;
mod levels;
mod engine;
mod view;

/*
#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
	pub fn r2j(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert("Calling...");
	r2j(&format!("Hell, {}!", name));
	alert("Called");
}
*/

#[wasm_bindgen(start)]
fn start() {
	let view = MenuView::new(target_elem(), MenuView::default_data());
	view.show();
}

