#![allow(dead_code)]

fn main() {
    let a = 1;
    let b = 2;
    let result = add(a, b);
    println!("Hello, result is {result}");
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}