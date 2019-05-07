extern crate wasm_bindgen;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn btree_pass() -> Result<JsValue, JsValue> {
    let mut map: BTreeMap<u32, ()> = BTreeMap::new();
    map.insert(42, ());
    JsValue::from_serde(&map).map_err(|error| JsValue::from_str(&format!("{:?}", error)))
}

#[wasm_bindgen]
pub fn btree_fail() -> Result<JsValue, JsValue> {
    let mut map: BTreeMap<(u32, u32), ()> = BTreeMap::new();
    map.insert((1, 2), ());
    JsValue::from_serde(&map).map_err(|error| JsValue::from_str(&format!("{:?}", error)))
}
