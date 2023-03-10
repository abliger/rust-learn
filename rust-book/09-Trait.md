# Trait

## 定义

一个类型的行为由其可供调用的方法构成.如果可以对不同类型调用相同的方法的话,这些类型就可以共享相同的行为了.trait 定义是一种将方法签名组合起来的方法,目的是定义一个实现某些目的所必需的行为的集合.

## 类型实现 trait

```rust
// 首先我们需要定义 trait
pub trait Summary {
    fn summarize(&self) -> String;
}
// 定义两个类型
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// 为类型添加实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

> 注意: 我们不能为外部类型实现外部 trait.即如果拥有一个名为 other 的 crate ,和 copy 的 trait .我们只能在 other 的 crate 中 为里面的类型添加 copy trait 实现.在主程序使用 use other , 不能给 other 作用域中的类型添加 copy trait 实现.类似的我们也能给外部类型 Vec<T> 实现 other crate 中的 trait.

## 默认实现

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

如果想要对 NewsArticle 实例使用这个默认实现,可以通过 impl Summary for NewsArticle {} 指定一个空的 impl 块.

> 默认实现允许调用相同 trait 中的其他方法,哪怕这些方法没有默认实现.
> 实现可以重载.

## trait 作为参数

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

指定 item 需要实现 Summary.

## trait bound 语法

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

如果有多个参数需要实现 Summary 可以使用泛型简化书写.

## 指定多个 trait bound

```rust
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {
```

## 通过 where 简化 trait bound

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

## 返回实现了 trait 的类型

如果有两个类型都实现了返回的 trait 类型且使用 if else 语句 分别返回两者,这会不能编译.这里只谈浅谈一点,其余的后面文章详谈.

## 使用 trait bound 有条件地实现方法

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

标准库为任何实现了 Display trait 的类型实现了 ToString trait.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

只有那些为 T 类型实现了 PartialOrd trait （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法.

注意对比 `impl Pair<i32>{}` 他表示为 泛型是 i32 的 Pair 实现方法.另一者是泛型需要实现 trait bound.
