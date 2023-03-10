# 高级 trait 和 高级类型

## 高级类型

### 类型别名

使用 type 关键字来给予现有类型另一个名字.

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

我们可以通过类型别名减少重复书写类型,如:

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

更常用的是用在 Result<T,E> 中

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

### never type

在函数从不返回的时候充当返回值.

```rust
fn bar() -> ! {
    // --snip--
}
```

描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型.对于 函数中直接使用 return; 或者使用 panic! 的使用的返回值都是 !.

### 动态大小类型

动态大小类型就是指只有在运行时才知道大小的类型.我命常用的 str 和 trait 都是动态大小类型的.

动态大小类型由于不知道大小,而 rust 中必须要知道类型的大小.所以我们对于 str 和 trait 类型,使用引用类型代替.如 &str、box<str>、&dyn trait、Box<dyn trait>.

#### Sized

Sized trait 决定了一个类型是否是在编译时可知.这个 trait 自动为编译器在编译时就知道大小的类型实现.另外,Rust 隐式的为每一个泛型函数增加了 Sized bound.

泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制：

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

?Sized 上的 trait bound 意味着 “T 可能是也可能不是 Sized” 同时这个注解会覆盖泛型类型必须在编译时拥有固定大小的默认规则.这种意义的 ?Trait 语法只能用于 Sized ,而不能用于任何其他 trait.

另外注意我们将 t 参数的类型从 T 变为了 &T:因为其类型可能不是 Sized 的,所以需要将其置于某种指针之后.在这个例子中选择了引用.

### newtype

如果想要在 Vec<T> 上实现 Display.而 rust 不允许我们实现外部 crate 类型.我们可以构建新结构,在新结构上实现 Display 并使用 Vec<T>.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 Vec<T> 的所有方法，这样就可以代理到 self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper.

如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait 并返回其内部类型是一种解决方案.

> 就算 newtype 封装的类型一样,但实际上是不同的类型,不能混用.

## 高级 trait

### 关联类型

关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

#### 使用

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {}
}
```

这个语法类似于泛型。那么为什么 Iterator trait 不定义成泛型呢?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

定义成泛型我们可以根据泛型不同定义多个 trait,再通过类型注解调用.使用关联类型指定定义一次.

### 默认泛型参数类型

```rust
pub trait Watch<Inner=String> {
    type Item;
    fn inner(&self) -> Option<Self::Item>;
    fn info(&self) -> Inner;
}

struct B {
    data: String,
}

impl Watch for B {
    type Item = String;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data.clone())
    }
    fn info(&self) -> String {
        println!("B inner is {}", self.data);
        self.data.clone()
    }
}

impl Watch<i32> for A {
    type Item = i32;
    fn inner(&self) -> Option<Self::Item> {
        Some(self.data)
    }
    fn info(&self) -> i32 {
        println!("A inner is {}", self.data);
        self.data
    }
}

fn main() {
    let a = A{data: 10};
    let b = B{data: String::from("B")};
    assert_eq!(10, a.info());
    assert_eq!(Some(String::from("B")), b.inner());
}
```

默认参数类型主要用于如下两个方面：

- 扩展类型而不破坏现有代码。
- 在大部分用户都不需要的特定情况进行自定义。

### 运算符重载

rust 在 std::ops 中定义了可以被重载的运算符.

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

### 调用相同名称的方法

- 对于不同 trait 中的相同函数,使用调用关联函数的方式调用.

  ```rust
  trait Pilot {
      fn fly(&self);
  }

  trait Wizard {
      fn fly(&self);
  }

  struct Human;

  impl Pilot for Human {
      fn fly(&self) {
          println!("This is your captain speaking.");
      }
  }

  impl Wizard for Human {
      fn fly(&self) {
          println!("Up!");
      }
  }

  impl Human {
      fn fly(&self) {
          println!("*waving arms furiously*");
      }
  }
  fn main() {
      let person = Human;
      Pilot::fly(&person);
      Wizard::fly(&person);
  }
  ```

- 对于方法或关联函数与 trait 定义的函数相同

  - 方法和关联函数的调用使用 关联函数的方式调用
  - trait 函数通过使用 as 关键字调用

  ```rust
  trait Animal {
      fn baby_name() -> String;
  }

  struct Dog;

  impl Dog {
      fn baby_name() -> String {
          String::from("Spot")
      }
  }

  impl Animal for Dog {
      fn baby_name() -> String {
          String::from("puppy")
      }
  }

  fn main() {
      println!("A baby dog is called a {}", Dog::baby_name());
      println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
  }
  ```

### 父 trait 用于在另一个 trait 中使用某 trait 的功能

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
