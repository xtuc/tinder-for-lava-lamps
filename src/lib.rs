use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, EventTarget};
extern crate console_error_panic_hook;
use js_sys::Math::random;
use std::panic;
use std::rc::Rc;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn get_random_lamp() -> String {
    let i = (random() * 100 as f64) as usize;
    return LAMPS[map_range((0, 100), (0, LAMPS.len()), i)].clone();
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let app = document
        .get_element_by_id("app")
        .expect("could not find app dom element");

    let img = document.create_element("img")?;
    img.set_attribute("id", "img")?;
    app.append_child(&img)?;
    draw_new_lamp(&img);

    let img = Rc::new(img);

    add_vote_buttons(
        &document,
        &app,
        {
            let img = Rc::clone(&img);
            Box::new(move || {
                draw_new_lamp(&img);
            })
        },
        {
            let img = Rc::clone(&img);
            Box::new(move || {
                draw_new_lamp(&img);
            })
        },
    )?;

    Ok(())
}

pub fn draw_new_lamp(img: &Element) {
    img.set_attribute("src", &get_random_lamp()).unwrap();
}

pub fn add_vote_buttons(
    document: &Document,
    app: &Element,
    like_fn: Box<dyn Fn()>,
    no_fn: Box<dyn Fn()>,
) -> Result<(), JsValue> {
    let container = document.create_element("div")?;
    container.set_attribute("class", "center")?;
    app.append_child(&container)?;

    let like_btn = document.create_element("button")?;
    like_btn.set_inner_html("❤️");
    container.append_child(&like_btn)?;

    let no_btn = document.create_element("button")?;
    no_btn.set_inner_html("👎");
    container.append_child(&no_btn)?;

    set_onlick(like_btn, like_fn);
    set_onlick(no_btn, no_fn);

    Ok(())
}

pub fn set_onlick(element: Element, handler: Box<dyn Fn()>) {
    let callback = Closure::wrap(handler);

    let event_target = element.dyn_into::<EventTarget>().unwrap();
    event_target
        .add_event_listener_with_event_listener("click", callback.as_ref().unchecked_ref())
        .unwrap();

    // important!
    // https://github.com/rustwasm/wasm-bindgen/blob/1f39a3045fcb652256797cde5a0aef8871fca74d/examples/closures/src/lib.rs#L75-L83
    callback.forget();
}

fn map_range(from_range: (usize, usize), to_range: (usize, usize), s: usize) -> usize {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
