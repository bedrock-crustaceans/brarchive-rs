use brarchive::SerializeOptions;
use js_sys::Uint8Array;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

/// Serialize a JS object or Map<string, string> into .brarchive bytes.
///
/// ```js
/// const bytes = serialize({ "entity.json": '{"id":"zombie"}' });
/// ```
#[wasm_bindgen]
pub fn serialize(entries: JsValue) -> Result<Uint8Array, JsError> {
    let map: BTreeMap<String, String> =
        serde_wasm_bindgen::from_value(entries).map_err(|e| JsError::new(&e.to_string()))?;
    let bytes = brarchive::serialize(map).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(Uint8Array::from(bytes.as_slice()))
}

/// Serialize with options. `dedup` skips writing duplicate content blocks.
#[wasm_bindgen]
pub fn serialize_with_options(entries: JsValue, dedup: bool) -> Result<Uint8Array, JsError> {
    let map: BTreeMap<String, String> =
        serde_wasm_bindgen::from_value(entries).map_err(|e| JsError::new(&e.to_string()))?;
    let bytes = brarchive::serialize_with(map, SerializeOptions { dedup })
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(Uint8Array::from(bytes.as_slice()))
}

/// List entry names in a .brarchive file without reading content. Returns a `string[]`.
#[wasm_bindgen]
pub fn list(data: &[u8]) -> Result<JsValue, JsError> {
    let names = brarchive::list(data).map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&names).map_err(|e| JsError::new(&e.to_string()))
}

/// Deserialize .brarchive bytes into a plain JS object `{ [key: string]: number[] }`.
///
/// Entry content is returned as byte arrays because archives may contain binary
/// entries (e.g. Mojang's compiled `MCB` files). Decode to text on the JS side
/// with `new TextDecoder().decode(new Uint8Array(bytes))` when appropriate.
#[wasm_bindgen]
pub fn deserialize(data: &[u8]) -> Result<JsValue, JsError> {
    let map: BTreeMap<String, Vec<u8>> =
        brarchive::deserialize(data).map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&map).map_err(|e| JsError::new(&e.to_string()))
}
