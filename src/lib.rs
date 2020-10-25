extern crate cfg_if;
extern crate wasm_bindgen;

mod qty;
mod utils;
mod recipe;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use recipe::Recipe;



cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn recipe_to_html(x: String) -> String {
    let maybe_r = Recipe::from_string(x);
    if let Some(r) = maybe_r {
	r.to_html()
    } else {
	"Invalid format".to_string()
    }

}
