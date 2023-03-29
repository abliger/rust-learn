#![feature(fn_traits)]
#[derive(Debug)]
struct Test {
    num: i32,
    b: bool,
}
fn main() {
    let test = Test { num: 0, b: false };
    test.test();
    println!("{:?}", test);
}

impl Test {
    fn test(&self) {}
}
