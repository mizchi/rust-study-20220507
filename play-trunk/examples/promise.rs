use wasm_bindgen_futures::{spawn_local};
use web_sys;
use wasm_bindgen::prelude::{*};

async fn get_from_js() -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::resolve(&42.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

fn main() {
    let v = get_from_js();
    spawn_local(async {
        let result = v.await;
        web_sys::console::log_1(&result.unwrap());
    });
}
