fn factorial(a: u64) -> u64 {
    match a {
        0 => 1,
        _ => a * factorial(a - 1),
    }
}

fn main() {
    println!("{}", factorial(10));
}

#[test]
fn test() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(10), 3628800);
}
