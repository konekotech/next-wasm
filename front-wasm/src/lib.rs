mod engine;

use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use engine::sprite::Sheet;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    web_sys::console::log_1(&JsValue::from("WASM module started"));
    draw_image_from_sprite()?;
    Ok(())
}

fn draw_image() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let image = HtmlImageElement::new()?;
    let image_url = "/assets/original/rhb/Idle (1).png";
    let image_clone = image.clone();
    let context_clone = context.clone();

    let closure = Closure::wrap(Box::new(move || {
        context_clone.draw_image_with_html_image_element(&image_clone, 0.0, 0.0).unwrap();
    }) as Box<dyn Fn()>);

    image.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    image.set_src(image_url);

    Ok(())
}

fn draw_image_from_sprite() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let json = engine::sprite::fetch_json("/assets/sprite_sheets/rhb.json")
            .await
            .expect("Failed to fetch JSON");

        let sheet: Sheet = json.into_serde().unwrap();

        web_sys::console::log_1(&JsValue::from("Sheet loaded"));

        let image = HtmlImageElement::new().unwrap();
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let success_callback = {
            let success_tx = Rc::clone(&success_tx);
            Closure::once(move || {
                if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                    success_tx.send(Ok(())).unwrap();
                }
            })
        };

        let error_callback = {
            let error_tx = Rc::clone(&error_tx);
            Closure::once(move |err: JsValue| {
                if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                    error_tx.send(Err(err)).unwrap();
                }
            })
        };

        image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        success_callback.forget();
        error_callback.forget();

        image.set_src("/assets/sprite_sheets/rhb.png");

        success_rx.await.expect("Failed to load image").expect("Failed to load image");

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");

            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                300.0,
                300.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
            ).expect("TODO: panic message");
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            interval_callback.as_ref().unchecked_ref(),
            50,
        ).expect("TODO: panic message");
        interval_callback.forget();
    });

    Ok(())
}
