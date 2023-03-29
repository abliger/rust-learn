#![feature(fn_traits)]

#[derive(Debug)]
struct Test {
    num: i32,
    b: bool,
}
fn main() {
    let mut test = Test { num: 33, b: true };
    let mut a = || {
        println!("{:?}", &mut test);
    };
    a.call_once(());
    a();
    a();
}
