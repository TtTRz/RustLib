extern crate htmlstream;

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
    for (pos, tag) in htmlstream::tag_iter(html) {
        log(&format!("{:?} {:?}", pos, tag));
        for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
            log(&format!("{:?} {:?}", pos, attr));
        }
    }
}
