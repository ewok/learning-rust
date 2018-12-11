extern crate crossbeam;

fn parallel_hello() {
    println!("-> Hello world:");
    std::thread::spawn(|| {
        println!("Hello world, from parallel universe");
    });

    let xs = vec![1, 2, 3, 4];

    std::thread::spawn(move || {
        println!("{:?}", xs);
    });
}

fn sync() {
    println!("-> Channels:");
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    std::thread::spawn(move || {
        let xs = rx.recv().unwrap();
        println!("{:?}", xs);
    });

    let xs = vec![1,2,3,4];

    tx.send(xs).unwrap();

}

fn crossbeam_test() {
    println!("-> Crossbeam:");

    let mut xs = [0,0,0,0];

    crossbeam::scope(|scope| {
        for i in &mut xs {
            scope.spawn(move |_| {
                *i += 1;
            });
        }
    }).unwrap();

    println!("{:?}", xs)
}

fn mutex_test() {
    println!("-> Crossbeam+Mutex:");

    let xs = std::sync::Mutex::new([0,0,0,0]);

    crossbeam::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|_| {
                let mut guard = xs.lock().unwrap();
                let xs: &mut [i32; 4] = &mut guard;
                for i in xs {
                    *i += 1;
                }
            });
        }
    });

    println!("{:?}", *xs.lock().unwrap())
}

fn main() {
    parallel_hello();
    sync();
    crossbeam_test();
    mutex_test();

    println!("Thread: Ok")
}
