use wasm_bindgen_futures::{spawn_local};
use web_sys;
use wasm_bindgen::prelude::{*};

// #[wasm_bindgen]
// pub async fn foo() {
//     web_sys::console::log_1(&JsValue::from_str("Hello, world!"));
//     // ...
// }

async fn get_from_js() -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::resolve(&42.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let el = document.create_element("p").unwrap();
    el.set_inner_html("Hello, world!");
    body.append_child(&el).unwrap();
    let v = get_from_js();
    spawn_local(async {
        let result = v.await;
        web_sys::console::log_1(&result.unwrap());
    });
    // web_sys::console::log_1(&"Hello, world!".into());
    // spawn_local(|| get_from_js());
    // spawn_local()

    // async {
    //     foo().await;
    // }.await;
    // foo().await;
}
