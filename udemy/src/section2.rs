#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {

    println!("Variable types example:");

    let a: u8 = 252;
    println!("{}", a);

    let mut b: i8 = 0;
    println!("{}", b);
    b = 1;
    println!("{}", b);

    let c = 1;
    println!("{}, {}", c, mem::size_of_val(&c));

    println!("Stack and heap example:");
    let p1 = origin();
    let p2 = Box::new(origin);

    println!("p1: {}", mem::size_of_val(&p1));
    println!("p2: {}", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3: {}", mem::size_of_val(&p3));
}
