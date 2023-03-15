# clap

前置命令

```rust
cargo add clap --features derive
rustup default nightly
```

## 解析

### 命令解析

使用 Command api 构建命令行工具最简单的例子如下:

```rust
let m = Command::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(
        Arg::new("in_file")
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();
```

但同时我们可以使用 command! 宏来获得 Command 对象.

#### 读取 cargo 信息

在 cargo.toml 中 打开 `clap = { version = "4.1.8", features = ["derive","cargo"] }` cargo features 可以让 clap 以宏的形式创建 Command 对象, 读取 cargo 配置信息.宏如下:

- command 
  它是以下所有宏的总命令
- crate_authors
  获得项目作者信息
- crate_description
  获得项目描述
- crate_name
  获得项目名
- crate_version
  获得项目版本

要注意的是 这些命令 是根据 env! 宏获得的项目信息. 如 crate_name 这个宏实际上就是调用了 `env!("CARGO_PKG_NAME")`.

没有 cargo features 就会编译报错.

```rust
#[cfg(not(feature = "cargo"))]
#[macro_export]
macro_rules! command {
    () => {{
        compile_error!("`cargo` feature flag is required");
    }};
    ($name:expr) => {{
        compile_error!("`cargo` feature flag is required");
    }};
}
```

#### Command api

至于 api 除了例子的几个,详见[command docs](https://docs.rs/clap/4.1.8/clap/struct.Command.html). 此处没什么好说的,根据名字基本上就把功能猜的八九不离十.

#### Arg 结构体

Arg 是用来为命令行工具添加参数.

```rust
let cfg = Arg::new("config")
      .short('c')
      .long("config")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("Provides a config file to myprog");
```

#### Arg api

- action 
  设置参数的保存行为
- exclusive
  如果出现 exclusive 修饰的参数,其他参数不能与其一起使用
- ignore_case
  忽略大小写
- required 
  必须出现该参数
- value_parser
  解析参数内容,到 ArgMatches 中.我们可以规定 解析后的类型、范围.`clap::value_parser!` 可以让我们更简单的使用它.
  也定实现 `clap::builder::ValueParserFactory` ,规定自己的解析函数
  ```rust
  clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .long("color")
            .value_parser(["always", "auto", "never"])
            .default_value("auto")
    )
    .arg(
        clap::Arg::new("hostname")
            .long("hostname")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .action(clap::ArgAction::Set)
            .required(true)
    )
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );
  ```


其他详见[arg docs](https://docs.rs/clap/4.1.8/clap/builder/struct.Arg.html)

#### ArgAction 枚举

此枚举是用来配合 Arg 的action 方法使用. 其有 7 个成员:

- Set
  action 默认设置,把 参数后面的内容存储到 ArgMatches
- Append
  追加参数内容到 ArgMatches
- Count 
  记录参数出现的次数,默认为 0
- SetFalse
  出现参数 ,设置 参数内容为 false,否则为 true
- SetTrue
  和上面相反
- Version
  参数功能变为查看命令行工具版本
- Help
  参数功能变为查看命令行工具帮助

#### ArgMatches

我们对 Command 使用 get_matches 得到 ArgMatches 对象.使用 get_one 或 try_get_one 来获得解析内容.[argmatches docs](https://docs.rs/clap/4.1.8/clap/struct.ArgMatches.html)

### 使用注解解析

使用 api 来解析命令无疑是有些麻烦的.所以更推荐使用注解来解析命令.官方的例子如下:

```rust
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // 读取 cargo.toml
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}
```

我们可以看到 command 和 arg 注解,内容就是其 api 内容.而且我们可以直接得到解析完后的结构体.

详情见[derive doc](https://docs.rs/clap/4.1.8/clap/_derive/_tutorial/index.html)
