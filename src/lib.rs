use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, EventTarget, Response};
extern crate console_error_panic_hook;
use std::panic;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
    request(&img, "initial");

    let img = Rc::new(img);

    add_vote_buttons(
        &document,
        &app,
        {
            let img = Rc::clone(&img);
            Box::new(move || {
                request(&img, "like");
            })
        },
        {
            let img = Rc::clone(&img);
            Box::new(move || {
                request(&img, "no");
            })
        },
    )?;

    Ok(())
}

fn request(img: &Element, endpoint: &str) {
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_str(&format!(
        "https://tinder-for-lava-lamps.sven.workers.dev/{}",
        endpoint
    ));

    let img_clone = img.clone();

    let get_text = Closure::<dyn FnMut(JsValue)>::wrap(Box::new(move |text: JsValue| {
        draw_new_lamp(&img_clone, text.as_string().unwrap());
    }));

    let cb = Closure::<dyn FnMut(JsValue)>::wrap(Box::new(move |value: JsValue| {
        let resp: Response = value.dyn_into().unwrap();
        resp.text().unwrap().then(&get_text);
    }));

    request_promise.then(&cb);

    cb.forget();
}

pub fn draw_new_lamp(img: &Element, value: String) {
    img.set_attribute("src", &value).unwrap();
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
