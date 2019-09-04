use js_sys::Math::random;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LAMPS: Vec<String> = vec![
        // "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/1.png".to_string(),
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/2.jpg".to_string(),
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/3.jpg".to_string(),
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/4.jpg".to_string(),
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/5.jpg".to_string(),
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/6.jpg".to_string(),
    ];
}

fn map_range(from_range: (usize, usize), to_range: (usize, usize), s: usize) -> usize {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
pub fn get_random_lamp() -> String {
    let i = (random() * 100 as f64) as usize;
    return LAMPS[map_range((0, 100), (0, LAMPS.len()), i)].clone();
}

#[wasm_bindgen]
pub fn handle(route: String) -> String {
    if route == "/like" {
        get_random_lamp()
    } else if route == "/no" {
        "https://s3.that-test.site/cdn-cgi/image/h=500/lava-lamps/1.png".to_string()
    } else {
        "broken image".to_string()
    }
}
