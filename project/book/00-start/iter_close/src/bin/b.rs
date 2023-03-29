#![feature(fn_traits)]
#[derive(Debug)]
struct Test {
    num: i32,
    b: bool,
}
fn main() {
    let test = Test { num: 33, b: true };
    let a = || test;
    //println!("{:?}", test);
    let test = a.call_once(());
    //a();
    println!("{:?}", test);
}
