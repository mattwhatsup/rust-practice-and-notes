## 文档注释

文档行注释 `///`

文档块注释 `/** */`

- 文档注释需要位于 lib 类型的包中，例如 src/lib.rs 中
- 文档注释可以使用 markdown语法！例如 # Examples 的标题，以及代码块高亮
- 被注释的对象需要使用 pub 对外可见，记住：文档注释是给用户看的，内部实现细节不应该被暴露出去

生成打开文档

```
cargo doc --open
```

## 包/模块注释

行注释 `//!` 和块注释 `/*! ... */`

## 文档测试

```rust
/// `add_one` 将指定值加1
///
/// # Examples11
///
/// ```
/// let arg = 5;
/// let answer = world_hello::compute::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

```
cargo test
```

### 测试panic

```rust
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust
/// // panics on division by zero
/// world_hello::compute::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}
```

如果想要通过这种测试，可以添加 should_panic：

```rust
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// world_hello::compute::div(10, 0);
/// ```
```

## 隐藏测试

```rust
/// ```
/// # // 使用#开头的行会在文档中被隐藏起来，但是依然会在文档测试中运行
/// # fn try_main() -> Result<(), String> {
/// let res = world_hello::compute::try_div(10, 0)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// #
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
```

## 注释跳转

```rust
/// `add_one` 返回一个[`Option`]类型
pub fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}
```

### 使用完整路径跳转到指定项

```rust
pub mod a {
    /// `add_one` 返回一个[`Option`]类型
    /// 跳转到[`crate::MySpecialFormatter`]
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }
}

pub struct MySpecialFormatter;
```

### 同名项跳转

```rust
/// 跳转到结构体  [`Foo`](struct@Foo)
pub struct Bar;

/// 跳转到同名函数 [`Foo`](fn@Foo)
pub struct Foo {}

/// 跳转到同名宏 [`foo!`]
pub fn Foo() {}

#[macro_export]
macro_rules! foo {
  () => {}
}
```

## 文档搜索别名

Rust 文档支持搜索功能，我们可以为自己的类型定义几个别名，以实现更好的搜索展现，当别名命中时，搜索结果会被放在第一位：


```rust
#[doc(alias = "x")]
#[doc(alias = "big")]
pub struct BigX;

#[doc(alias("y", "big"))]
pub struct BigY;
```

## 综合例子

1. `cargo new art`
2. 创建 `src/lib.rs`

```rust
//! # Art
//!
//!  未来的艺术建模库，现在的调色库

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //! 定义颜色的类型

    /// 主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 副色
    #[derive(Debug,PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! 实用工具，目前只实现了调色板
    use crate::kinds::*;

    /// 将两种主色调成副色
    /// ```rust
    /// use art::utils::mix;
    /// use art::kinds::{PrimaryColor,SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
```
3. `src/main.rs`

```rs
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}",mix(blue, yellow));
}
```

## 文档属性

### inline
可以用于内联文档, 而不是链接到一个单独的页面。

```rs
#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}
```

### no_inline
用于防止链接到单独的页面或其它地方。

```rs
// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;
```

### hidden
通过这个属性让 rustdoc 不要将下面的项包含在文档中:

```rs
// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;
```

[doc-comments完整例子](https://github.com/sunface/rust-by-practice/tree/master/practices/doc-comments)

