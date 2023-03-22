use std::env;
use std::error::Error;

use minigrep::{read_file, search, search_ignore_case, Config};
/// 1. 获得命令行参数
/// 2. 解析 成 config 对象
/// 3. 读取文件内容
/// 4. 按行扫描 查找是否有查找内容
/// 忽略大小写
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args)?;

    let content = read_file(conf.path)?;

    if conf.case {
        search(&content, conf.search);
    } else {
        search_ignore_case(&content, conf.search);
    }
    Ok(())
}
