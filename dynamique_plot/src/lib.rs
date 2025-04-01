#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
pub(crate) mod commands;
#[cfg(target_arch = "wasm32")]
pub use app::MyApp;

#[cfg(target_arch = "wasm32")]
fn get_canvas(id : &str) -> web_sys::HtmlCanvasElement {
        use eframe::wasm_bindgen::JsCast as _;

        let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

        document
                .get_element_by_id("the_canvas_id")
                .expect("Failed to find the_canvas_id")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("the_canvas_id was not a HtmlCanvasElement")
}