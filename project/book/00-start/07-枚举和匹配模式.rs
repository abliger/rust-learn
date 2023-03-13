#[derive(Debug)]
enum Test{
    Test1,
    Test2(String),
    Test3(i32),
    Test4 { x:i32,y:i32},
    Test5(i32,i32,bool)
}
fn main() {
   let a = Test::Test5(0,0,false);
   let b = Test::Test4{x:1,y:1};
   let c = Test::Test2("str test2".to_string());
   match c {
       //Test::Test1 => println!("Test1"),
       //Test::Test2(str) => println!("{str}"),
       //Test::Test3(i) => println!("{i}"),
       Test::Test4{x,..} => println!("{x}"),
       Test::Test5(_,_,z) => println!("{z}"),
       other => println!("{:?}",other),
   }

   if let Test::Test4{x} = b{
     println!("{:?}",x );
   }

   if let Test::Test5(x,..) = a{
       println!("{:?}",x);
   }
}
