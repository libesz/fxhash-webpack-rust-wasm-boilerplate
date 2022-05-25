use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    #[wasm_bindgen()]
    fn fxrand() -> f64;

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureBool(name: &str) -> bool;

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureNumber(name: &str) -> f64;

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureString(name: &str) -> String;
}

#[derive(Copy, Clone)]
pub struct Color{
  red: u8,
  green: u8,
  blue: u8
}

impl Color {
  pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    Color{red: red, green: green, blue: blue}
  }
  pub fn to_string(self) -> String {
  format!("rgb({}, {}, {})", self.red, self.green, self.blue)
  }
}

fn get_random_u8() -> u8 {
  (fxrand() * 256.0).floor() as u8
}


#[wasm_bindgen(start)]
pub fn start() {
  let window = web_sys::window().unwrap();

  let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
    log("handling resize event");
    draw();
  }) as Box<dyn FnMut(_)>);
  window
      .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
      .unwrap();
  closure.forget();
  draw();
}

fn draw() {
  let feature_string_example = getFxHashFeatureString("StringExample");
  log(&*feature_string_example);

  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
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

  let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
  let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
  canvas.set_height(height);
  canvas.set_width(width);
  context.set_fill_style(&Color::from_rgb(get_random_u8(), get_random_u8(), get_random_u8()).to_string().into());
  context.rect(0.0, 0.0, width.into(), height.into());
  context.fill();
}
