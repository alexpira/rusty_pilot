use wasm_bindgen::prelude::*;

use crate::common::*;
use crate::view::menuview::MenuView;

mod common;
mod rand;
mod geom;
mod levels;
mod engine;
mod view;

#[wasm_bindgen(start)]
fn start() {
	let view = MenuView::new(target_elem(), MenuView::default_data());
	view.show();
}

