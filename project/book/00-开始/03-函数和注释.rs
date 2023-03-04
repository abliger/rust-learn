fn main() -> () {
   let a = test("打印 函数和注释"); 
   println!("{a}");
}

/// 
/// # test
/// test 函数我们要打印传入的字符串
fn test(x:&str) -> i32{
    // 打印
    println!("{x}");
    // ..
    /* 注释 */
    2+3
}
