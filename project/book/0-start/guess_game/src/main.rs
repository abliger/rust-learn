use std::io;
fn main() {
    println!("猜数字");
    println!("输入数字");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("读取失败");
    let mut a = 1;
    a = 2;
}
