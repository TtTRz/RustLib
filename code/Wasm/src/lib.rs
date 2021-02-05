use md5;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn say_hello() -> String {
    let hello = format!("hello");
    hello
}

#[wasm_bindgen]
pub fn md5rust(name: &str) -> String {
    let result = md5::compute(name);
    format!("{:?}", result)
}
