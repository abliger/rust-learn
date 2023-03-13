#[derive(Debug)]
struct MyStruct {
    field1: String
}

#[derive(Debug)]
struct MyStruct2(i32,bool,String);
#[derive(Debug)]
struct MyStruct3;

impl MyStruct{
    // add code here
    fn print(&self){
       println!("{:?}",self); 
    }
}
impl MyStruct2{
    // add code here
    fn print(&self){
       println!("{:?}",self); 
    }
}
impl MyStruct3{
    fn print_test(str:String){
        println!("我是 MyStruct3 {str}");
    }
    // add code here
    fn print(&self){
       println!("{:?}",self); 
    }
}

fn main() {
    let a = MyStruct{
        field1 : String::from("test1")
    };
    let b = MyStruct{
        ..a
    };
    let c = MyStruct2(1,true,"test2".to_string());
    let d = MyStruct3;
    b.print();
    c.print();
    d.print();
    MyStruct3::print_test("MyStruct3".to_string());
    println!("b field1 is {}",b.field1);
}
