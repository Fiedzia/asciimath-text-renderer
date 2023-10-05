use wasm_bindgen::prelude::*;
use asciimath_text_renderer;

#[wasm_bindgen]
pub fn render(s:&str) -> String {
    asciimath_text_renderer::render(s)
}


