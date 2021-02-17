#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

fn vector() {
    println!("vectors example:");

    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    a.push(44);

    println!("{:?}", a);

    println!("first: {}", a[0]);

    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("error, no such element"),
    }

    for x in &a {
        print!("{} ", x)
    }
    println!();

    while let Some(x) = a.pop() {
        print!("{:?}: ", x)
    }

    println!();
    println!("Should be empty: '{:?}'", a);
}

fn hashmap() {
    println!("hashmap example:");

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    // Iterating
    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    // Overriding
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    // Safe overriding
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);

    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

fn hashset() {
    println!("hashset example:");

    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    // Only unique elements
    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    println!("we added vega: {}", added_vega);
    println!("{:?}", greeks);

    if !greeks.contains("kappa") {
        println!("we don't have kappa")
    };

    println!("we removed delta: {}", greeks.remove("delta"));

    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!(
        "is {:?} a subset of {:?}? {:?}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoined
    println!(
        "is {:?} disjoined of {:?}? {:?}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // union and intersection
    println!(
        "{:?} is union of {:?} {:?}",
        _2_8.union(&_6_10),
        _2_8,
        _6_10
    );

    println!(
        "{:?} is intersection of {:?} {:?}",
        _2_8.intersection(&_6_10),
        _2_8,
        _6_10
    );

    println!(
        "{:?} is difference of {:?} {:?}",
        _2_8.difference(&_6_10),
        _2_8,
        _6_10
    );

    println!(
        "{:?} is symmetric difference of {:?} {:?}",
        _2_8.symmetric_difference(&_6_10),
        _2_8,
        _6_10
    );
}

fn iterators() {
    println!("iteration example:");

    let vec = vec![3, 2, 1];

    for x in &vec {
        println!("{}", *x);
    }

    for x in &vec {
        println!("{}", *x);
    }

    for x in vec.iter() {
        println!("{}", *x);
    }

    // Mutable
    let mut vec2 = vec![3, 2, 1];

    for x in vec2.iter_mut().rev() {
        *x += 1;
        println!("{}", *x);
    }
    println!("{:?}", vec2);

    // into
    let mut vec3 = vec![3, 2, 1];
    let vec4 = vec![1, 2, 3];

    vec3.extend(vec4);
    println!("{:?}", vec3);
}

pub fn show_sections() {
    println!("STD Collections:");
    vector();
    hashmap();
    hashset();
    iterators();
}

fn main() {
    show_sections();
}
