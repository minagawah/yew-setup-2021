#![recursion_limit = "512"]

mod app;
pub mod components;
pub mod constants;
pub mod entry;
pub mod message;
pub mod state;

use wasm_bindgen::prelude::*;
use yew::App;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    if let Some(elem) = yew::utils::document().query_selector("#app").unwrap() {
        App::<app::App>::new().mount(elem);
        Ok(())
    } else {
        Err(JsValue::from("No element to wrap"))
    }
}
