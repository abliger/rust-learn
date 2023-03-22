use std::{io::stdin, process::exit};

use rand::Rng;

/// 1. 生成随机数 1-100
/// 2. 读取数字 并 解析 失败再次读取
/// 3. 判断
/// 4. 相等 打印你赢了 退出程序
/// 5. 不相等 循环 2-5
fn main() {
    let num: u32 = rand::thread_rng().gen_range(1..=100);
    println!("请输入 1 - 100 间的数字");
    loop {
        let mut buf = String::new();
        if let Err(e) = stdin().read_line(&mut buf) {
            eprintln!("读入失败 {}", e);
            continue;
        }
        match buf.trim().parse::<u32>() {
            Err(_) => {
                eprintln!("你应该输入数字");
                continue;
            }
            Ok(i @ 1..=100) => match i.cmp(&num) {
                std::cmp::Ordering::Equal => {
                    println!("你赢了");
                    exit(0);
                }
                std::cmp::Ordering::Less => println!("猜测的过小"),
                std::cmp::Ordering::Greater => println!("猜测的过大"),
            },
            Ok(_) => println!("输入的数字应该在 1 - 100 之间"),
        }
    }
}
