#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    println!("Structures example:");
    let p1 = Point { x: 3.0, y: 4.0 };
    println!("point p is {}:{}", p1.x, p1.y);

    let p2 = Point { x: 5.0, y: 10.0 };

    let l1 = Line { start: p1, end: p2 };

    println!(
        "Line start at {}:{}, ends at {}:{}",
        l1.start.x, l1.start.y, l1.end.x, l1.end.y
    );
}

fn enumerations() {
    println!("Enumeration example:");

    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8, u8, u8),
        Cmyk {
            cyan: u8,
            magenta: u8,
            yellow: u8,
            black: u8,
        },
    }

    // let c: Color = Color::RgbColor(1, 2, 3);
    let c: Color = Color::Cmyk {
        cyan: 0,
        magenta: 1,
        yellow: 2,
        black: 3,
    };
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("{} {} {}", r, g, b),
        Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        _ => println!("Other color"),
    }
}

fn unions() {
    println!("Unions example:");

    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;
    let value = unsafe { iof.i };

    println!("{}", value);

    fn process_value(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i: 42 } => {
                    println!("meaning of life value")
                }
                IntOrFloat { f } => {
                    println!("value us eq {}", f)
                } // Unreachable pattern
                  // IntOrFloat { i } => {
                  //     println!("value us eq {}", i)
                  // }
            }
        }
    }

    process_value(IntOrFloat { f: 42.0 });
}

fn option_t() {
    println!("Option<T> example:");

    let x = 3.0;
    let y = 0.0;

    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero"),
    }

    if let Some(z) = result {
        println!("result = {}", z)
    }
}

fn arrays() {
    println!("arrays example:");

    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{} elements, first is {}", a.len(), a[0]);

    a[0] = 321;

    println!("{} elements, first is {}", a.len(), a[0]);

    println!("debug: {:?}", a);

    // size of
    use std::mem;

    let b = [1; 10];
    println!("{:?} is {} bytes", b, mem::size_of_val(&b));

    let b = [1u8; 10];
    println!("{:?} is {} bytes", b, mem::size_of_val(&b));

    // matrix
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);
}

fn slices() {
    println!("slices example:");

    fn use_slice(slice: &mut [i32]) {
        println!("{:?}", slice);
        slice[2] = 123;
    }

    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{:?}", sp);
    println!("{0} {1}", sp.0, sp.1);

    let (a, b) = sp;
    println!("{} {}", a, b)
}

fn pattern_matching() {
    fn how_many(x: i32) -> &'static str {
        match x {
            0 => "no",
            1 | 2 => "one or two",
            12 => "a dozen",
            z @ 9..=11 => "lots of",
            _ if (x % 2 == 0) => "some",
            _ => "a few",
        }
    }

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x))
    }

    let point = (0, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("{}", y),
    }
}

fn generics() {
    println!("generics example:");

    struct Point<T> {
        x: T,
        y: T
    }

    let a:Point<f64> = Point { x: 0.0, y: 0.0 };
    let b:Point<f64> = Point { x: 1.2, y: 3.4 };

    struct Line<T> {
        start: Point<T>,
        end: Point<T>
    }

    let l1 = Line { start: a, end: b };
}

pub fn show_sections() {
    structures();
    enumerations();
    unions();
    option_t();
    arrays();
    slices();
    tuples();
    pattern_matching();
    generics();
}

fn main() {
    show_sections();
}
