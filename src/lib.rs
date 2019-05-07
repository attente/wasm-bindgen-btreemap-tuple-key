extern crate wasm_bindgen;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn btree_pass() -> Result<JsValue, JsValue> {
    JsValue::from_serde(&BTreeMap::<u32, ()>::new()).map_err(|error| JsValue::from_str(&format!("{}", error)))
}

#[wasm_bindgen]
pub fn btree_fail() -> Result<JsValue, JsValue> {
    JsValue::from_serde(&BTreeMap::<(u32, u32), ()>::new()).map_err(|error| JsValue::from_str(&format!("{}", error)))
}
