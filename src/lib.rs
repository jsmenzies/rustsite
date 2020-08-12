use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod route;
mod pages;
mod components;
mod types;
mod api;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}