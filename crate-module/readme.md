## 包 crate

对于 Rust 而言，包是一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库。

## 项目 Package

由于 Package 就是一个项目，因此它包含有独立的 Cargo.toml 文件，以及因为功能性被组织在一起的一个或多个包。一个 Package 只能包含一个库(library)类型的包，但是可以包含多个二进制可执行类型的包。

- 二进制 Package
- 库 Package `cargo new my-lib --lib`
  - 它包含有一个库类型的同名包 my-lib，该包的根文件是 src/lib.rs

> 牢记 Package 是一个项目工程，而包只是一个编译单元，基本上也就不会混淆这个两个概念了：src/main.rs 和 src/lib.rs 都是编译单元，因此它们都是包。
>
> 上面创建的 Package 中仅包含 src/main.rs 文件，意味着它仅包含一个二进制同名包 my-project。如果一个 Package 同时拥有 src/main.rs 和 src/lib.rs，那就意味着它包含两个包：库包和二进制包，这两个包名也都是 my-project —— 都与 Package 同名。
>
> 一个真实项目中典型的 Package，会包含多个二进制包，这些包文件被放在 src/bin 目录下，每一个文件都是独立的二进制包(该目录下的每个文件都是一个独立的二进制包，包名与文件名相同，不再与 package 的名称相同。)，同时也会包含一个库包，该包只能存在一个 src/lib.rs：

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

- 唯一库包：src/lib.rs
- 默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
- 其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
- 集成测试文件：tests 目录下
- 基准性能测试 benchmark 文件：benches 目录下
- 项目示例：examples 目录下

## module 模块

使用 cargo new --lib restaurant 创建一个小餐馆，注意，这里创建的是一个库类型的 Package，然后将以下代码放入 src/lib.rs 中：

```rust

// 餐厅前厅，用于吃饭
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

模块树结构 (src/main.rs 和 src/lib.rs在crate root, 这两个文件的内容形成了一个模块 crate)

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### 用路径引用模块

- 绝对路径，从包根开始，路径名以包名或者 crate 作为开头
- 相对路径，从当前模块开始，以 self，super 或当前模块的标识符作为开头

文件名：src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

- super 上层模块
- self 同级模块


结构体和枚举的可见性特点：

- 将结构体设置为 pub，但它的所有字段依然是私有的
- 将枚举设置为 pub，它的所有字段也将对外可见


### 模块和文件分离

现在，把 front_of_house 前厅分离出来，放入一个单独的文件中 src/front_of_house.rs：


```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

src/lib.rs 中：

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- mod front_of_house; 告诉 Rust 从另一个和模块 front_of_house 同名的文件中加载该模块的内容
- 使用绝对路径的方式来引用 hosting 模块：crate::front_of_house::hosting;

## 练习分离模块


```rust
// in src/lib.rs

mod front_of_house;
mod back_of_house;
pub fn eat_at_restaurant() -> String {
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
```

```rust
// in src/back_of_house.rs

use crate::front_of_house;
pub fn fix_incorrect_order() {
    cook_order();
    front_of_house::serving::serve_order();
}

pub fn cook_order() {}
```

```rust
// in src/front_of_house/mod.rs

pub mod hosting;
pub mod serving;

```rust
// in src/front_of_house/hosting.rs

pub fn add_to_waitlist() {}

pub fn seat_at_table() -> String {
    String::from("sit down please")
}
```

```rust
// in src/front_of_house/serving.rs

pub fn take_order() {}

pub fn serve_order() {}

pub fn take_payment() {}

// Maybe you don't want the guest hearing the your complaining about them
// So just make it private
fn complain() {}
```

```rust
// src/main.rs

mod front_of_house;

fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
```


> 原来main.rs和lib.rs各自都是一个crate，并且名字都叫package的名字，但其实是两个crate
>
> 这就是为什么第五题里main.rs要写 mod front_of_house; ，实际上是在自己的crate里又创建了mod front_of_house，因为lib.rs里的这个模块不是公开的，这实在是坑啊。。为什么要这么设计呢，我觉得这个实在不合理
>
> 怪不得我看了其他教程看不懂，很多教程都没有把这个完整的结构给读者。

## use 引入

优先使用最细粒度(引入函数、结构体等)的引用方式，如果引起了某种麻烦(例如前面两种情况)，再使用引入模块的方式。

### as 别名

as 别名引用
对于同名冲突问题，还可以使用 as 关键字来解决，它可以赋予引入项一个全新的名称：


```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### 引入项再导出
当外部的模块项 A 被引入到当前模块中时，它的可见性自动被设置为私有的，如果你希望允许其它外部代码引用我们的模块项 A，那么可以对它进行再导出：


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

如上，使用 pub use 即可实现。这里 use 代表引入 hosting 模块到当前作用域，pub 表示将该引入的内容再度设置为可见。

当你希望将内部的实现细节隐藏起来或者按照某个目的组织代码时，可以使用 pub use 再导出，例如统一使用一个模块来提供对外的 API，那该模块就可以引入其它模块中的 API，然后进行再导出，最终对于用户来说，所有的 API 都是由一个模块统一提供的。

### 受限的可见性

如果我们想要让某一项可以在整个包中都可以被使用，那么有两种办法：

在包根中定义一个非 pub 类型的 X(父模块的项对子模块都是可见的，因此包根中的项对模块树上的所有模块都可见)
在子模块中定义一个 pub 类型的 Y，同时通过 use 将其引入到包根

```rust
mod a {
    pub mod b {
        pub fn c() {
            println!("{:?}",crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}",Y);
}
```

以下代码报错

```rust
// 目标：`a` 导出 `I`、`bar` and `foo`，其他的不导出
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        mod c {
            const J: i32 = 4;
        }
    }
}
```

修改

```rust
pub mod a {
    pub const I: i32 = 3;

    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4;
            pub fn semisecret(x: i32) -> i32 {
                x + J
            }
        }
    }
}
```

使用 `pub (in crate::a)`

```rust
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}
```

通过 pub(in crate::a) 的方式，我们指定了模块 c 和常量 J 的可见范围都只是 a 模块中，a 之外的模块是完全访问不到它们的。

限制可见性语法
pub(crate) 或 pub(in crate::a) 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：

- pub 意味着可见性无任何限制
- pub(crate) 表示在当前包可见
- pub(self) 在当前模块可见
- pub(super) 在父模块可见
- pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

### 综合例子

```rust
// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前包中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}

```

### 通过 #[path ="你的路径"] 可以放在任何目录都行，如：

```rust
#[path ="你的路径"]
mod core;
```

![](https://user-images.githubusercontent.com/100085326/164968138-0efae930-8bc0-4c8b-b4e8-163e6c566d5a.png)

![](https://tva1.sinaimg.cn/large/008vxvgGly1h8goqik2xjj312g0cywfk.jpg)