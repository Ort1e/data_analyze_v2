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

#[cfg(target_arch = "wasm32")]
fn create_canvas(id : &str, width : usize, height : usize) -> web_sys::HtmlCanvasElement {
        use eframe::wasm_bindgen::JsCast as _;

        let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

        let body = document.body().expect("No body");
        let elt = document.create_element("canvas").expect("Failed to create the canvas");
        elt.set_attribute("id", id).expect("Failed to set the id of the canvas");
        elt.set_attribute("width", &width.to_string()).expect("Failed to set the width of the canvas");
        elt.set_attribute("height", &height.to_string()).expect("Failed to set the height of the canvas");
        body.append_child(&elt).expect("Failed to append the canvas to the body");

        elt.dyn_into::<web_sys::HtmlCanvasElement>().expect("Failed to convert the canvas to a HtmlCanvasElement")
}

#[cfg(target_arch = "wasm32")]
fn remove_canvas(id : &str) {
        use eframe::wasm_bindgen::JsCast as _;

        let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

        let canvas = document.get_element_by_id(id).expect("Failed to find the canvas");
        document.body().expect("No body").remove_child(&canvas).expect("Failed to remove the canvas");
}