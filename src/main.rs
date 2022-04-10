mod queue;
mod point;
use crate::point::{Point};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let first_point = Point { x: 1, y: 1 };
    let _ = queue::add(first_point);

    println!("esperando 1 seg");
    // let ten_millis = time::Duration::from_millis(1000);
    // let now = time::Instant::now();

    // thread::sleep(ten_millis);
    println!("termine de esperar");

}
