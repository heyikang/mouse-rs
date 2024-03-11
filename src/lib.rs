#![deny(clippy::all)]

use mouse_rs::{types::keys::Keys as MouseRsKeys, Mouse};
use napi_derive::napi;

#[napi]
pub enum Keys {
  LEFT,
  RIGHT,
  MIDDLE,
  WHEEL,
  X,
  X2,
  UP,
  DOWN,
  VERTICAL,
  HORIZONTAL,
}

#[napi(object)]
pub struct MousePosition {
  pub x: i32,
  pub y: i32,
}

fn mouse_key_map(key: Keys) -> MouseRsKeys {
  match key {
    Keys::LEFT => MouseRsKeys::LEFT,
    Keys::RIGHT => MouseRsKeys::RIGHT,
    Keys::MIDDLE => MouseRsKeys::MIDDLE,
    Keys::WHEEL => MouseRsKeys::WHEEL,
    Keys::X => MouseRsKeys::X,
    Keys::X2 => MouseRsKeys::X2,
    Keys::UP => MouseRsKeys::UP,
    Keys::DOWN => MouseRsKeys::DOWN,
    Keys::VERTICAL => MouseRsKeys::VERTICAL,
    Keys::HORIZONTAL => MouseRsKeys::HORIZONTAL,
  }
}

#[napi]
pub enum MouseButton {
  Left,
  Right,
  Middle,
}

#[napi]
pub fn move_to(x: i32, y: i32) -> bool {
  let mouse = Mouse::new();
  mouse.move_to(x, y).is_ok()
}

#[napi]
pub fn press(button: Keys) -> bool {
  let key = mouse_key_map(button);
  let mouse = Mouse::new();
  let is_ok = mouse.press(&key).is_ok();
  is_ok
}

#[napi]
pub fn release(button: Keys) -> bool {
  let key = mouse_key_map(button);
  let mouse = Mouse::new();
  let is_ok = mouse.release(&key).is_ok();
  is_ok
}

#[napi]
pub fn get_position() -> Option<MousePosition> {
  let mouse = Mouse::new();
  match mouse.get_position() {
    Ok(point) => Some(MousePosition {
      x: point.x,
      y: point.y,
    }),
    Err(_) => None,
  }
}

#[napi]
pub fn wheel(delta: i32) -> bool {
  let mouse = Mouse::new();
  let is_ok = mouse.wheel(delta).is_ok();
  is_ok
}

#[napi]
pub fn scroll(delta: i32) -> bool {
  wheel(delta)
}

#[napi]
pub fn click(key: Keys) -> bool {
  let key = mouse_key_map(key);
  let mouse = Mouse::new();
  let is_ok = mouse.click(&key).is_ok();
  is_ok
}
