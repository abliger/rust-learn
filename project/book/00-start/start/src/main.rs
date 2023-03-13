mod A{
    pub fn test1(){
        println!("test1");
    }
}
mod B;
mod C;
use start::{test4,d};
fn main() {
    A::test1();
    B::test2();
    C::test3();
    test4();
    d::test5();
}
