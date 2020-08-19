use rand::Rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> { Ok(()) }

#[wasm_bindgen]
pub fn test_rand1016() -> u32 {
    rand::thread_rng().gen()
}
