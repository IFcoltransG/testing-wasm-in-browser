mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn unit();
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, world!");
}

#[wasm_bindgen]
pub fn addle(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

#[wasm_bindgen]
pub fn panic() {
    // unit();
    panic!("Aaah");
}

