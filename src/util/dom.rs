use web_sys;

#[warn(dead_code)]
pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[warn(dead_code)]
pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[warn(dead_code)]
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}
