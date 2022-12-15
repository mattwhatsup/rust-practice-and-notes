## 智能指针解引用

```RS
fn main() {
    let x = Box::new(1);
    let sum = *x + 1;
}
```

自己实现

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let y = MyBox::new(5);

    assert_eq!(5, *y);
}
```

## 隐式deref转换

```rs
fn main() {
    let s = String::from("hello world");
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}
```

以上代码有几点值得注意：

- String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
- &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
- 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)

### 连续隐式deref转换
Deref 可以支持连续的隐式转换，直到找到适合的形式为止：
```rs
fn main() {
    let s = MyBox::new(String::from("hello world"));
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}
```

注意隐式deref得到了简单的代码，但使过程不清晰，降低了可读性

```rs
fn main() {
    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();
}
```

## Deref 规则总结

### 引用归一化

- 把智能指针（比如在库中定义的，Box、Rc、Arc、Cow 等）从结构体脱壳为内部的引用类型，也就是转成结构体内部的 &v
- 把多重&，例如 &&&&&&&v，归一成 &v

我们来看一段标准库源码：


```rs
impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}
```

在这段源码中，&T 被自动解引用为 T，也就是 &T: Deref<Target=T> 。 按照这个代码，&&&&T 会被自动解引用为 &&&T，然后再自动解引用为 &&T，以此类推， 直到最终变成 &T。

### 例子

```rs

#![allow(unused)]
fn main() {
    fn foo(s: &str) {}

    // 由于 String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();

    // 因此下面的函数可以正常运行：
    foo(&owned);
}

```

```rs

#![allow(unused)]
fn main() {
    use std::rc::Rc;

    fn foo(s: &str) {}

    // String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // 且 Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String ，然后 &String 再自动解引用为 &str
    let counted = Rc::new(owned);

    // 因此下面的函数可以正常运行:
    foo(&counted);
}

```

```rs

#![allow(unused)]
fn main() {
    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }

    let f = &&Foo;

    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

```

## 三种 Deref 转换

- 当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
- 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
- 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U

```rs
struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn main() {
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s)
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
```

以上代码有几点值得注意:

- 要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
- T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，对应上例中，就是将 &mut MyBox<String> 转换为 &mut String

> 如果从 Rust 的所有权和借用规则的角度考虑，当你拥有一个可变的引用，那该引用肯定是对应数据的唯一借用，那么此时将可变引用变成不可变引用并不会破坏借用规则；但是如果你拥有一个不可变引用，那同时可能还存在其它几个不可变的引用，如果此时将其中一个不可变引用转换成可变引用，就变成了可变引用与不可变引用的共存，最终破坏了借用规则。