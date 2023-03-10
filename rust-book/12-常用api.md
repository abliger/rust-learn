# 常用 api

## String 和 &str

### 关联函数

1. new 创建空字符串
2. from(&str) 根据字符串字面量创建字符
3. String::from_utf8(Vec<i32>) i32 集合转换 String

### 方法

1. to_string() 获得字符串对象
2. chars() 获得字符迭代器 chars().nth(2) 获得第二个字符
3. split(&str) 分割字符
4. contains(&str) 检测字符串是否含有某字符串
5. parse() 解析字符串 parse::<i32>() 把字符串解析成 i32
6. trim() 删除字符串两边的空格,自动删除 `\n`
7. start_with(&str) 是否以某字符串开始
8. end_with(&str) 是否以某字符串结束
9. push_str(&str) 添加字符串
10. eq(&str) 检测字符串是否相等
11. len() 获得字符串长度
12. get(Slice) 获得装载切片的 Option 对象

## 数字

1. eq(num) 等于
2. ge(num) 大于等于
3. gt(num) 大于
4. le(num) 小于等于
5. lt(num) 小于
6. ne(num) 不等于
7. cmp(num) 比较大小 等于输出 0 大于输出 1 小于输出 -1
8. abs() 获得绝对值
9. to_string() 转变为字符串

## Option

1. expect(message) None 调用,停止程序,打印自定义错误消息
2. unwrap None 调用,停止程序,打印默认信息
3. unwrap_or(T) None 调用,返回 T
4. unwrap_or_default() 实现 Default 的 T 调用,返回 default
5. unwrap_or_else(fn) None 调用后,调用闭包

6. ok_or(T) 把 Option 类型转变为 Result 类型,且如果是 None,返回 Ok(T)
7. ok_or_else(fn) 把 Option 类型转变为 Result 类型,且如果是 None,调用 fn
8. transpose()

   ```rust
   struct SomeErr;

   let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
   let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
   assert_eq!(x, y.transpose());
   ```

9. filter(fn) fn 判断元素是否应保留, false None ;true Some(time)
10. or(T) 如果是 none 调用,使用 T 替换 返回 Some(T)
11. flatten 对于 形如 Some(Some(T)) 去掉最外面的 Some 变为 Some(T)
12. map(fn)
13. map_or(T,fn)
14. mao_or_else(fn,fn) 上面三种都是对 T 进行类型转换,或对 T 进行操作
15. zip(T) 生成 元组 Some(1).zip(Some("hi")) 会生成 Some((1,"hi"))
16. zip_with

    ```rust
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
    }

    let x = Some(17.5);
    let y = Some(42.7);

    assert_eq!(x.zip_with(y, Point::new), Some(Point { x: 17.5, y: 42.7 }));
    assert_eq!(x.zip_with(None, Point::new), None);
    ```

17. is_some()
18. is_none()

## Result

1. is_ok()
2. is_err()
3. err() 把 Err(T) 转换为 Some(T)
4. ok() 把 Ok(T) 转换为 Some(T)

## env

### 获得命令行参数

```rust
use std::env;

// Prints each argument on a separate line
for argument in env::args() {
    println!("{argument}");
}
```

### 获得环境变量

#### 根据 key 获得 vlaue

```rust
use std::env;

let key = "HOME";
match env::var(key) {
    Ok(val) => println!("{key}: {val:?}"),
    Err(e) => println!("couldn't interpret {key}: {e}"),
}
```

#### 获得所有环境变量

```rust
use std::env;

// We will iterate through the references to the element returned by
// env::vars();
for (key, value) in env::vars() {
    println!("{key}: {value}");
}
```

## io

### 控制台输入输出

#### 控制台输入

```rust
use std::io;

let mut input = String::new();

io::stdin().read_line(&mut input).unwrap();
```

#### 控制台输出

```rust
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    io::stdout().write(&[42])?;
    Ok(())
}
```

```rust
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    io::stderr().write(&[42])?;
    Ok(())
}
```

## fs

### 读文件

```rust
use std::fs;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let foo: SocketAddr = String::from_utf8_lossy(&fs::read("address.txt")?).parse()?;
    Ok(())
}
```

### 写文件

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("foo.txt", b"Lorem ipsum")?;
    fs::write("bar.txt", "dolor sit")?;
    Ok(())
}
```

### 其他命令

1. rename(filename,filename) 重命名
2. create_dir(dir) 创建目录 如果目录已存在、父路径不存在会报错
3. create_dir_all(dir)
4. read_link 读取软连接
5. head_link(dir,dir) 创建硬连接
6. remove_dir
7. remove_dir_all
8. remove_file 删除

## net

### 监听者

```rust
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
```

### 发送者

```rust
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here
```

## 宏

1. todo!() 标记代码需要完成
2. dbg!(expr) 调试打印 表达式
3. cfg! arch 判断平台
