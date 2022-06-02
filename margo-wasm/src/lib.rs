use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(content: String) -> Result<JsValue, JsError> {
    let tokenstream = margo::parse(content).unwrap();
    let js = serde_wasm_bindgen::to_value(&tokenstream)?;
    Ok(js)
}
