trait Test{
    fn test(&self);
}

enum Enum{
    One,Two(i32)
}

impl Test for Enum{
    fn test(&self){
    println!("this is Enum test");
    }
}


enum Test2<T>{ one(T) }

impl<T> Test for Test2<T>{
    fn test(&self){
        println!("this is Test2")
    }
}

trait newTrait{
    
}

impl<T: newTrait> Test2<T>{
    fn test3(&self){
        println!("this is Test test3");
    }
}

impl newTrait for i32{}

impl Test2<&str>{
    fn test4(&self){}
}

fn main(){
    let a = Enum::Two(0);
    a.test();

    let b = Test2::one(0);
    b.test();
    b.test3();
    let c = Test2::one("one");
    c.test4();
}
