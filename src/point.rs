
use std::clone::Clone;


#[derive(Clone, Copy)]
pub struct Point {
  pub x: u32,
  pub y: u32
}

impl Default for Point {
  fn default()  -> Self {
      Self { x: Default::default(), y: Default::default() }
  }
}
