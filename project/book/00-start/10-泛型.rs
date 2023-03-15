fn test<T>(param:T) ->T{
    param
}

#[derive(Debug)]
struct Test<T>{
    a:T,
    b:T,
}

#[derive(Debug)]
struct Point<T,B>{
    a:T,
    b:B,
}

impl<T,B> Point<T,B>{
    fn test2(&self){}
    fn test3<G,H>(self,p:Point<G,H>) -> Point<T,B>{
        self
    }
}

struct Test2<T>(T);

impl<T> Test2<T>{
    fn test4(&self){}
}

impl Test2<i32>{
    fn test5(self){
        println!("this is Test2 test5");
    }
}


fn main(){
    let a = test(3);
    let b = test(3.14);

    println!("{a}");
    println!("{b}");
    
    let c = Test{a:3,b:5};
    println!("{:?}",c);

    let d = Point{a : 0,b: true};
    let e = Point{a : 3.13,b: 'c'};

    d.test2();
    let f = d.test3(e);
    println!("{:?}",f);

    let z = Test2(0);
    let x = Test2("Test");

    z.test4();
    z.test5();
    x.test4();
    // x.test5();
}
