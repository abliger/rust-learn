fn test<'a>(a:&'a str,b:&'a str) -> &'a str{
    if a.len() > 0 { a } else { b }
}

fn test1(a:&str) -> (&str,&str){
    (&a[..],&a[..5])
}

enum Test3<'a>{One(&'a str)}

impl<'a> Test3<'a>{
    fn print(&'a self,str:&'a str)  -> &str{
        if let Test3::One(a) = self {
        println!("{} {}",a,str);
        }
        str
    }
}

 fn main(){
    let a = "a";
    let b = "b";

    test(&a,&b);
    let c ="abcdefg";
    test1(&c);

    let a = Test3::One(&b);
    a.print("this is print");
 }
