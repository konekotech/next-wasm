use std::collections::HashMap;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;


pub async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    return wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}

#[derive(Deserialize, Debug)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}

#[derive(Deserialize, Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Deserialize, Debug)]
pub struct Cell {
    pub frame: Rect,
}