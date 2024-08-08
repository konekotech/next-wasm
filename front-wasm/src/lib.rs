use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

// グローバル変数
static mut CONTEXT: Option<CanvasRenderingContext2d> = None;
static mut IMAGE: Option<HtmlImageElement> = None;
static mut X: f64 = 0.0;
static mut Y: f64 = 0.0;

#[wasm_bindgen]
pub fn setup(canvas_id: &str, image_url: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let image = HtmlImageElement::new()?;
    image.set_src(image_url);

    unsafe {
        CONTEXT = Some(context);
        IMAGE = Some(image);
    }

    Ok(())
}

#[wasm_bindgen]
pub fn draw_image(x: f64, y: f64) -> Result<(), JsValue> {
    unsafe {
        if let Some(context) = &CONTEXT {
            if let Some(image) = &IMAGE {
                context.clear_rect(0.0, 0.0, context.canvas().unwrap().width().into(), context.canvas().unwrap().height().into());
                context.draw_image_with_html_image_element(image, x, y)?;
                X = x;
                Y = y;
            }
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn move_image(dx: f64, dy: f64) -> Result<(), JsValue> {
    unsafe {
        draw_image(X + dx, Y + dy)?;
    }
    Ok(())
}
