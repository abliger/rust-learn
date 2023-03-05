# rust文档

我们使用 rust-book 猜数游戏的代码文件完成这篇文档的编写.

```shell
cargo new start
```

我们在该项目中编写了三个模块和一个 lib 文件,让 main 函数使用.

## 生成命令

### rustdoc

使用 rustdoc 对 lib.rs 生成文档. 
```shell
$ rustdoc src/lib.rs --crate-name guess_game
```

### cargo doc

Cargo 整合了 rustdoc,使得生成文档更容易.代替 rustdoc 命令,我们可以这样做:

```shell
cargo doc
```

用 `cargo doc --verbose` 查看实际上等于

```shell
rustdoc --edition=2021 --crate-type bin --crate-name guess_game -o '/Users/fengsixue/...' ... # 剩余省略不写
``

这可以对 main.rs 也生成文档.

可以使用命令直接打开文档

```shell
cargo doc --open
```

## Outer 和 inner 文档

`///` 语法用来对下面一个 item 生成文档,所以称为 outer 文档.

还有语法`//!`,用来生成 item 内部的文档,也叫做 inner 文档,通常用来对整个 crate 生成文档,因为是 crate 的根,没有 item 在前面.

所以为了生成整个 crate 的文档，你需要在 lib.rs 中这样用 //!:


```rust
//! 
//!  这里描述这个 crate 的详细信息
//!

///
///  这里描述 mod a 的信息
///
pub mod a{
//! 也能使用 //! 在模块里面表示 mod a 的信息
//! 注意 不管是 mod 内还是外,生成的文档是不分开的

  /// this is fn test.outer
  pub fn test(){
  //! this is fn test.inner
  /// 而 /// 在函数里面,文档就不会包含这一行.

  }
}
```

### 连接 item

```rust
/// This struct is not [Bar]
pub struct Foo1;

/// This struct is also not [bar](Bar)
pub struct Foo2;

/// This struct is also not [bar][b]
///
/// [b]: Bar 
pub struct Foo3;

/// This struct is also not [`Bar`]
pub struct Foo4;

/// This struct *is* [`Bar`]!
pub struct Bar;

/// This is a special implementation of [positional parameters].
///
/// [positional parameters]: std::fmt#formatting-parameters 连接到文档的标题
struct MySpecialFormatter;

/// ========
/// 消除歧义
/// See also: [`Foo`](struct@Foo)
struct Bar;

/// This is different from [`Foo`](fn@Foo)
struct Foo {}

fn Foo() {}

/// This is different from [`foo!`]  指定连接宏,只有一个 foo 宏,不会歧义
fn foo() {}

/// This is different from [`foo()`] 指定连接函数
macro_rules! foo {
  () => {}
}
```

不像常规的 markdown,[bar][Bar] 语法也被支持.反引号会被删除,所以 [`Option`] 可以正确地链接到Option.

## 例子

通过例子理解模块和函数无疑是最好的方法.事例

```rust
/// Example
/// ```rust
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// let fortytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fortytwo, fortytwo+10);
/// #     Ok(())
/// # }
/// ```
```

通过 # 来隐藏文档事例中不关心的内容.在 rust 1.34.0 可以省略 mian.

对于错误处理的简单写法

```rust
/// ```
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
```

### 宏测试

```rust
/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(1 + 1 == 2, “Math is broken.”);
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(true == false, “I’m broken.”);
/// # }
/// ```
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}
```

我们需要自己增加 extern crate 一行,从而我们可以加上 #[macro_use] 属性.第二,我们需要自己增加 main().理由同上.最后 # 的使用使得一些内容不会出现在输出中.

### 代码块属性

代码块可以通过属性标注帮助 rustdoc 在测试例子代码时处理正确.

1. ignore         忽略代码
2. should_panic   运行会 panic
3. no_run         编译但不运行
4. compile_fail   应该编译失败
5. edition2018    设置 edition 版本 有 edition2015, edition2018 和 edition2021

```rust
/// ```compile_fail
/// let x = 5;
/// x += 2; // shouldn't compile!
/// ```
```

### readme 事例测试

如果你的 readme 文件中有着对模块的例子,使用下面的方式可以方便的测试

```rust

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
```

使用 #[cfg(doctest)] 表示只用来文档测试,ReadmeDoctests 结构体是不会再文档 api 中的.

使用 `rustdoc test [file]` 或 `cargo test --doc` 测试文档事例.

## doc属性

|注解|解释|
|---|---|
|#[doc(hidden)]|隐藏注解下的内容|
|#[doc = " This is a doc comment."]|处理文档内容,相当于 `///`|
|#[doc = include_str!("../readme.md")]|引入外部文件|
|#[doc(alias = "TheAlias")]|给搜索索引增加了别名|
|#[doc(alias("x", "big"))]|增加多个别名|
|#[doc(no_inline)]|用于 use 声明|
|#[doc(inline)]|用于 use 声明|

```rust
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}
```

对于上面代码,即使 mod bar 没有公开,生成的文档都会有指向 Bar 结构体的文档.使用下面代码会让文档不生成 Bar 结构体的连接.

```
#[doc(no_inline)]
pub use bar::Bar;
```

> 还有一些对于 create 和 item 的属性,对于编写文档不太重要,详情见 [rustdoc](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html).

## lint

```rust
#![allow(rustdoc::broken_intra_doc_links)] // allows the lint, no diagnostics will be reported
#![warn(rustdoc::broken_intra_doc_links)] // warn if there are broken intra-doc links
#![deny(rustdoc::broken_intra_doc_links)] // error if there are broken intra-doc links
```

### broken_intra_doc_links

没有找到连接的内容或连接歧义报警告

### private_intra_doc_links

公共 item 连接一个私有 item 

### missing_docs

缺少文档时提示. 默认允许.

### missing_crate_level_docs

提示 crate 根没有文档.默认允许.

### missing_doc_code_examples

缺少代码示例时提示

详情见 [rustdoc-lint](https://doc.rust-lang.org/rustdoc/lints.html)

## 条件编译

```rust
/// 在 windows 构建,生成所有文档
#[cfg(any(windows, doc))]
pub struct WindowsToken;
/// Token struct that can only be used on Unix.
#[cfg(any(unix, doc))]
pub struct UnixToken;
```


