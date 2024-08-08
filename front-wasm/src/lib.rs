use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

#[wasm_bindgen]
pub struct CanvasDrawer {
    context: CanvasRenderingContext2d,
    image: HtmlImageElement,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl CanvasDrawer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, image_url: &str) -> Result<CanvasDrawer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let image = HtmlImageElement::new()?;
        let image_url = image_url.to_string();
        let image_clone = image.clone();
        let context_clone = context.clone();
        
        let closure = Closure::wrap(Box::new(move || {
            context_clone.draw_image_with_html_image_element(&image_clone, 0.0, 0.0).unwrap();
        }) as Box<dyn Fn()>);

        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        image.set_src(&image_url);

        Ok(CanvasDrawer {
            context,
            image,
            x: 0.0,
            y: 0.0,
        })
    }

    #[wasm_bindgen]
    pub fn draw_image(&self) -> Result<(), JsValue> {
        self.context.clear_rect(
            0.0,
            0.0,
            self.context.canvas().unwrap().width().into(),
            self.context.canvas().unwrap().height().into(),
        );
        self.context.draw_image_with_html_image_element(&self.image, self.x, self.y)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn move_image(&mut self, dx: f64, dy: f64) -> Result<(), JsValue> {
        self.x += dx;
        self.y += dy;
        self.draw_image()?;
        Ok(())
    }
}
