mod app;
mod config;
mod date;
mod dom;
mod status;
mod theme;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_week_status_wasm(start_date: &str, current_date: &str) -> String {
    date::get_week_status(start_date, current_date)
}

#[wasm_bindgen]
pub async fn run_app() -> Result<(), JsValue> {
    app::run_app().await
}
