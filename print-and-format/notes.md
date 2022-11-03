## 相关模块

`std::fmt`

## 宏

- format!：将格式化文本写到字符串。
- print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
- println!: 与 print! 类似，但输出结果追加一个换行符。
- eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
- eprintln!：与 eprint! 类似，但输出结果追加一个换行符。

- fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
- fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。

## 一些常见格式的罗列

```rust
fn main() {
    println!("{}", 1); // 默认用法,打印Display
    println!("{:o}", 9); // 八进制
    println!("{:x}", 255); // 十六进制 小写
    println!("{:X}", 255); // 十六进制 大写
    println!("{:p}", &0); // 指针
    println!("{:b}", 15); // 二进制
    println!("{:e}", 10000f32); // 科学计数(小写)
    println!("{:E}", 10000f32); // 科学计数(大写)
    println!("{:?}", "test"); // 打印Debug
    println!("{:#?}", ("test1", "test2")); // 带换行和缩进的Debug打印
    println!("{a} {b} {b}", a = "x", b = "y"); // 命名参数


    assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !"); // <右边界宽度
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!"); // <右边界宽度+填充
    assert_eq!(format!("Hello {:^5}!", "x"),  "Hello   x  !"); // ^居中
    assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!"); // >右边界宽度

    println!("Hello {:+}", 5); // +显示正号
    println!("{:#x}!", 27); // #显示十六进制
    assert_eq!(format!("Hello {:05}!", 5),  "Hello 00005!"); // 宽度
    println!("{:#09x}!", 27); // 十六进制数字宽度
    println!("Hello {0} is {1:.5}", "x", 0.01); // 小数位

    // $代入符号
    println!("Hello {0} is {1:.5}", "x", 0.01);
    // 0的位置是5，1的位置是x，2的位置是0.01
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    // 0的位置是x，1的位置是5，2的位置是0.01
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    // *统配代入符号
    // 0的位置是x，1的位置是5，2的位置是0.01
    println!("Hello {} is {:.*}",    "x", 5, 0.01);
    // 0的位置是5，1的位置是x，2的位置是0.01
    println!("Hello {1} is {2:.*}",  5, "x", 0.01);

    // 转义{}
    assert_eq!(format!("Hello {{}}"), "Hello {}");
    assert_eq!(format!("{{ Hello"), "{ Hello");
}
```

## Debug trait

`#[derive(Debug)]` 所有类型都能推导fmt::Debug 的实现

```rust
// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
```

使用 `{:#?}` 美化打印

## Display trait

`fmt::Display` 需要自己实现

```rust
// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.0)
    }
}
```

如果想实现 `{:b}` 打印二进制则需要自己实现 `std::fmt::Binary`