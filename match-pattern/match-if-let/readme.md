## 模式匹配

```rust
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```

- match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
- match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
- X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可

```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```

## 模式绑定

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```

## if let 匹配

当你只要匹配一个条件，且忽略其他条件时就用 `if let` ，否则都用 `match`。

```rust
if let PATTERN = SOME_VALUE {

}
```

```rust
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}

// --->

if let Some(3) = v {
    println!("three");
}
```

## while let条件循环

```rust

#![allow(unused)]
fn main() {
  // Vec是动态数组
  let mut stack = Vec::new();

  // 向数组尾部插入元素
  stack.push(1);
  stack.push(2);
  stack.push(3);

  // stack.pop从数组尾部弹出元素
  while let Some(top) = stack.pop() {
      println!("{}", top);
  }
}

```

## matches!宏

Rust 标准库中提供了一个非常实用的宏：`matches!`，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 `true` or `false`。

```rust
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];

    // 错误
    //v.iter().filter(|x| x == MyEnum::Foo);
    v.iter().filter(|x| matches!(x, MyEnum::Foo));

    // 更多例子
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}
```

## 变量覆盖

```rust
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   if let Some(age) = age {
       println!("匹配出来的age是{}",age);
   }

   println!("在匹配后，age是{:?}",age);
}
```
注意，match 中的变量覆盖其实不是那么的容易看出，因此要小心！