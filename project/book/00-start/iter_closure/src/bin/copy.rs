#![feature(fn_traits)]

// https://rustyyato.github.io/rust/syntactic/sugar/2019/01/17/Closures-Magic-Functions.html
fn main() {
    // let f = || ();

    // f.call_once(());
    // f();
    // f.call(());
    // // f.call_mut(());

    // let a = 33_i32;
    // let f = || test_own(a);

    // println!("main {}", a);
    // f();
    // println!("main {}", a);

    let mut a = true;
    let mut f = || {
        a = false;
    };

    println!("main {}", a);
    f();
    f();

    // let mut a = true;
    // let mut f = move || {
    //     a = false;
    // };

    // println!("main {}", a);
    // f();
}

fn test_own<T: Sized>(_: T) {}
