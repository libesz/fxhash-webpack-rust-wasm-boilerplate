use std::f64;
use std::f64::consts::PI;
use wasm_bindgen::closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use std::panic;
use std::sync::{Arc, Mutex};
use hex::FromHex;

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

    #[wasm_bindgen()]
    fn fxpreview();

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureBool(name: &str) -> bool;

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureNumber(name: &str) -> f64;

    #[wasm_bindgen(js_name = getFxHashFeature)]
    fn getFxHashFeatureString(name: &str) -> String;

    #[wasm_bindgen(js_name = getFxHashParam)]
    fn getFxHashParamBool(name: &str) -> bool;

    #[wasm_bindgen(js_name = getFxHashParam)]
    fn getFxHashParamNumber(name: &str) -> f64;

    #[wasm_bindgen(js_name = getFxHashParam)]
    fn getFxHashParamString(name: &str) -> String;

    #[wasm_bindgen(js_name = getFxHashParamColor)]
    fn getFxHashParamColor(name: &str) -> String;
}

#[derive(Copy, Clone)]
pub struct Color{
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8
}

impl Color {
  pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    Color{red: red, green: green, blue: blue, alpha: 255}
  }
  pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color{red: red, green: green, blue: blue, alpha: alpha}
  }
  pub fn from_param(param_id: &str) -> Color {
    let mut color_string = getFxHashParamColor(param_id);
    assert_eq!(color_string.len(), 9);

    _ = color_string.remove(0);

    let decoded = <[u8; 4]>::from_hex(color_string).expect("Decoding failed");

    println!("{:?}", decoded);
    Color{red: decoded[0], green: decoded[1], blue: decoded[2], alpha: decoded[3]}
  }
  pub fn to_string(self) -> String {
    format!("rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha as f64 / 256.0)
  }
  pub fn set_red(&mut self, value: u8) {
    self.red = value;
  }
  pub fn set_green(&mut self, value: u8) {
    self.green = value;
  }
  pub fn set_blue(&mut self, value: u8) {
    self.blue = value;
  }
  pub fn set_alpha(&mut self, value: u8) {
    self.alpha = value;
  }
  pub fn red(self) -> u8 {
    self.red
  }
  pub fn green(self) -> u8 {
    self.green
  }
  pub fn blue(self) -> u8 {
    self.blue
  }
  pub fn alpha(self) -> u8 {
    self.alpha
  }
}

struct RandCache {
  index: u32,
  cache: Vec<u8>
}
#[derive(Debug, Clone)]
pub struct RandCacheError;

impl core::fmt::Display for RandCacheError {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
      write!(f, "index is not available")
  }
}

impl RandCache {
  pub fn new() -> RandCache {
    RandCache { index: 0, cache: Vec::new() }
  }
  pub fn rewind(&mut self) {
    _ = self.rewind_to(0);
  }
  pub fn rewind_to(&mut self, index: u32) -> Result<u32, RandCacheError>{
    match self.cache.get(index as usize) {
      Some(_) => {
        let original_index = self.index;
        self.index = index;
        Ok(original_index)
      }
      None => {
        Err(RandCacheError)
      },
    }
  }
  pub fn get_cache_index(&self) -> u32 {
    self.index
  }
  pub fn get_u8(&mut self) -> u8 {
    let result: u8;
    match self.cache.get(self.index as usize) {
      Some(result_from_cache) => result = *result_from_cache,
      None => {result = RandCache::get_random_u8(); self.cache.push(result)},
    }
    self.index += 1;
    result
  }
  fn get_random_u8() -> u8 {
    (fxrand() * 256.0).floor() as u8
  }
}

#[wasm_bindgen(start)]
pub fn start() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let window = web_sys::window().unwrap();

  let rand_mutex = Arc::new(Mutex::new(RandCache::new()));
  let closure_func = move || {
    log("handling resize event");
    draw(&rand_mutex);
    fxpreview();
  };
  // initial draw
  closure_func();
  let closure = closure::Closure::wrap(Box::new(closure_func) as Box<dyn FnMut()>);
  window
      .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
      .unwrap();
  closure.forget();
}

fn draw(rand_cache: &Arc<Mutex<RandCache>>) {
  let mut unlocked_rand_cache = rand_cache.lock().unwrap();
  unlocked_rand_cache.rewind();

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

  let bg_color = Color::from_param("color_id");
  context.set_fill_style(&bg_color.to_string().into());
  context.rect(0.0, 0.0, width.into(), height.into());
  context.fill();

  let fg_color = Color::from_rgb(unlocked_rand_cache.get_u8(), unlocked_rand_cache.get_u8(), unlocked_rand_cache.get_u8());
  context.set_fill_style(&fg_color.to_string().into());

  context.begin_path();
  match context.arc(width as f64 / 2.0, height as f64 / 2.0, height as f64 / 4.0, 0.0, 2.0 * PI) {
    Ok(_) => {},
    Err(e) => log(&e.as_string().unwrap()),
  }
  context.fill();
}
