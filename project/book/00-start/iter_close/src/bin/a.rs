#![feature(fn_traits)]
fn main() {
    let num = 33;
    let mut a = || {
        println!("{}", num);
    };
    println!("{}", num);
    a.call(());
    a.call_mut(());
    a.call_once(());
}
