use std::{cell::RefCell, borrow::{BorrowMut, Borrow}, thread::{JoinHandle, self}, process::Output};
use std::future::Future;

use crate::point::{Point, self};

thread_local! {
    static STATE: QueueState = QueueState::default();
}

#[derive(Default)]
pub struct QueueState {
    pub points: RefCell<Vec<Point>>,
    pub retry: RefCell<Vec<Point>>,
}

impl QueueState {
  pub fn add(&self, point: Point) -> Result<Point,String> {
    println!("add");
    let thread = thread::spawn(|| async move {
      println!(r#"HOLA HILITO"#);
      let result = foo(point.borrow().clone()).await;
      if result.is_err(){
        return Err(format!("error: {:?}", result.err()))
      } else {
        return Ok(point.borrow().clone());
      }
    });
    let result = thread.join();
    result.and_then(|point| return Ok(point))
  }

  pub fn add_retry(&self, point: Point) {
    self.retry.borrow_mut().push(point);
  }
}

pub async fn foo(point: Point) -> Result<Point, String> {
  let string: String = format!("hola");
  print!("foo-x: {:?}; foo-y:{:?}", point.x, point.y);
  return Ok(point);
}

pub fn add(point: Point) {
  STATE.with(|s| s.add(point));
}