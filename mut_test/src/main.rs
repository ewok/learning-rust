fn push_all(xs: &mut Vec<i32>, ys: &Vec<i32>) {
    for &y in ys {
        xs.push(y);
    }
}

fn main() {
    let mut xs = vec![1, 2, 3, 4];
    let ys = vec![1, 2, 3, 4];
    push_all(&mut xs, &ys);

    println!("mut: Ok, {:?}", xs)
}
