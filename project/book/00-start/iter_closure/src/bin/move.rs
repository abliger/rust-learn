#![feature(fn_traits)]

#[derive(Debug /* , Clone, Copy */)]
struct Test();
// Copy Fn (&T i32 char bool Option Some)
// T FnOnce
// &mut T FnMut
// move Copy  !
//      !     所有权
//
fn main() {
    // let t = Test();
    // let f = || test_own(t);

    // // println!("main {:?}", t);

    // // f.call_once(());
    // //f();
    // f();

    //&T
    // let t = Test();
    // let f = || test(&t);

    // println!("{:?}", t);
    // // test_own(t);
    // f.call_once(());
    // f.call(());
    // f();

    let t = Test();
    let f = move || test(&t);
    // println!("{:?}", t);
    // test_own(t);
    // f.call_once(());
    f.call(());
    f();
    f();
    // print!("{:?}", t);

    // let mut t = Test();
    // let mut f = || {
    //     test_mut(&mut t);
    // };

    // // println!("main {:?}", t);
    // f();
    // f();
    // f.call(());

    // let mut t = Test();
    // let mut f = move || test_mut(&mut t);

    // // println!("main {:?}", t);
    // fn_closure_own(f);
    // f();
    // f();
}

fn test<T: Sized>(_: &T) {}
fn test_own<T: Sized>(_: T) {}
fn test_mut<T: Sized>(_: &mut T) {}
fn fn_closure_own<T>(mut f: T)
where
    T: FnMut() -> (),
{
    f();
}
