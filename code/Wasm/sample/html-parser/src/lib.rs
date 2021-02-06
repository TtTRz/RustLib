extern crate htmlstream;

mod virtual_dom;

use virtual_dom::{VD, VirtualDom};

use wasm_bindgen::prelude::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn warn(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn error(text: &str);
}

#[wasm_bindgen]
pub fn parse(html: &str) -> () {
    let vd = VD::new();
}
