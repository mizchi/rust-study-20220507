use web_sys;

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let el = document.create_element("p").unwrap();
    el.set_inner_html("Hello, world!");
    body.append_child(&el).unwrap();
    web_sys::console::log_1(&"Hello, world!".into());
}
