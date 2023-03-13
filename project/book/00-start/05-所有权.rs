fn main() {
    let str = "str".to_string();
    // let str1 = &str;
    // println!("{str}");

    // test(str);
    // println!("{str}");

    test(str.clone());
    println!("{str}");
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 没问题
    println!("{}", r3);
    // println!("{} and {}", r1, r2); // r1 r2 在此处使用 即 r1 r2 作用域在此结束
    // 此位置之后 r1 和 r2 不再使用

    
}

fn test(str:String){
    println!("{str}");
}
