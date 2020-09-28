#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    // Boxing: Use heap
    let p2 = Box::new(origin());
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // Boxing: Use stack
    let p3 = *p2;
    println!("p3.x = {}", p3.x);
}
