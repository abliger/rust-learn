fn main() {
    let f = || ();

    fn_closure_own(f);
    fn_closure_own(&f);
    // fn_closure_mut(&f);
}

fn fn_closure_own<T>(_: T)
where
    T: Fn() -> (),
{
}
fn fn_closure_mut<T>(_: &mut T)
where
    T: Fn() -> (),
{
}
