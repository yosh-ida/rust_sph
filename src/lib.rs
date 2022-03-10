use engine::SPH;
use std::f64;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

mod engine;
mod util;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut engine: Option<Box<SPH>> = None;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    // #[cfg(debug_assertions)]
    // console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));
    init();
    console::log_1(&JsValue::from_str("initialized"));
    Ok(())
}

#[wasm_bindgen]
pub fn init() {
    unsafe {
        engine = Some(Box::new(SPH::new(1.0, 1.5)));
    }
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(500);
    canvas.set_height(500);
}

pub fn sph() -> &'static SPH {
    unsafe { engine.as_ref().unwrap() }
}

pub fn msph() -> &'static mut SPH {
    unsafe { engine.as_mut().unwrap() }
}

#[wasm_bindgen]
pub fn draw(ax: f64, ay: f64, az: f64, t: f64) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    util::console_log("stepping");
    let mut s = msph();
    s.step(t);
    let s = sph();
    util::console_log("steped");

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    // context.set_stroke_style(&JsValue::from_str("blue"));
    // context.begin_path();
    // for p in &s.particles {
    //     if p.movable {
    //         // console::log_1(&JsValue::from_str(
    //         //     format!("plot: ({} {})", p.position.x, p.position.y).as_str(),
    //         // ));
    //         context
    //             .arc(
    //                 p.position.x * 10.0,
    //                 p.position.y * 10.0,
    //                 5.0,
    //                 0.0,
    //                 f64::consts::PI * 2.0,
    //             )
    //             .unwrap();
    //     }
    // }
    // context.stroke();
    // util::console_log("write movable");

    // context.set_stroke_style(&JsValue::from_str("red"));
    // context.begin_path();
    // for p in &s.particles {
    //     if !p.movable {
    //         console::log_1(&JsValue::from_str(
    //             format!("plot: ({} {})", p.position.x, p.position.y).as_str(),
    //         ));
    //         context
    //             .arc(
    //                 p.position.x * 10.0,
    //                 p.position.y * 10.0,
    //                 5.0,
    //                 0.0,
    //                 f64::consts::PI * 2.0,
    //             )
    //             .unwrap();
    //     }
    // }
    // context.stroke();
    // util::console_log("write imovable");

    for p in &s.particles {
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str(if p.movable { "blue" } else { "red" }));
        context
            .arc(
                p.position.x * 10.0,
                p.position.y * 10.0,
                5.0,
                0.0,
                f64::consts::PI * 2.0,
            )
            .unwrap();
        context.stroke();
    }
}
