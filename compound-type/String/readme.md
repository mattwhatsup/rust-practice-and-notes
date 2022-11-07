
- `#![allow(unused_variables)]` 编译器忽略未使用变量的警告
- `unimplemented!()` 指明函数没有实现

## 切片

对于字符串而言，切片就是对 String 类型中某一部分的引用，String类型的切片就是&str

```rust
let s = String::from("hello");

let slice: &str = &s[0..2];
let slice: &str = &s[..2];
```

在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：

```rust
let s = "中国人";
let a = &s[0..2];
println!("{}",a);
```

错误，在获得了不可变借用后，使用可变借用，然后又打印先前的不可变借用产生错误，两者不能共存。

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 获得不可用借用

    s.clear(); // error! // 获得可变借用

    println!("the first word is: {}", word); // 打印不可变借用
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
```

修复上面的代码

```rust
fn main() {
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let word = first_word(&s);

    println!("the first word is: {}", word);
    s.clear();
}
fn first_word(s: &str) -> &str {
    &s[..1]
}
```

其他切片
```rust
#![allow(unused)]
fn main() {
    let a = [1, 2, 3, 4, 5];

    let copy = a; // 复制，地址与a不同
    let slice1 = &a[..];
    let slice2 = &a[..2];

    println!("{:p}, {:p}, {:p}, {:p}", &a, &copy, slice1, slice2)
    // 0x7ffee4da2cf0, 0x7ffee4da2d04, 0x7ffee4da2cf0, 0x7ffee4da2cf0
}
```

> 一个切片引用占用了2个字大小的内存空间( 从现在开始，为了简洁性考虑，如无特殊原因，我们统一使用切片来特指切片引用 )。 该切片的第一个字是指向数据的指针，第二个字是切片的长度。字的大小取决于处理器架构，例如在 x86-64 上，字的大小是 64 位也就是 8 个字节，那么一个切片引用就是 16 个字节大小。

- 切片签名 &[T]
- 数组签名 [T; length]

```rust
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    assert!(std::mem::size_of_val(&slice) == 16);
}
```

## 字符串

⭐️ 字符串字面量 是 切片

> 虽然 `String` 的底层是 `Vec<u8>` 也就是字节数组的形式存储的，但是它是基于 `UTF-8` 编码的字符序列。`String` 分配在堆上、可增长且不是以 `null` 结尾。

> 而 `&str` 是切片引用类型( `&[u8]` )，指向一个合法的 `UTF-8` 字符序列，总之，`&str` 和 `String` 的关系类似于 `&[T]` 和 `Vec<T>` 。

```rust
let s = "Hello, world!";
// ->
let s: &str = "Hello, world!";
// s是不可变引用
```

> str 类型是**硬编码进可执行文件，也无法被修改**，但是 String 则是**一个可增长、可改变且具有所有权的 UTF-8 编码字符串**，当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

### String 与 &str 的转换

`&str` to `String`
- `String::from("hello,world")`
- `"hello,world".to_string()`

`String` to `&str`:
- 取引用（切片）`let slice1 = &s;`
- `let slice1: &str = s.as_str(); `

```rust
fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}
```

⭐️ Rust 不允许去索引字符串

报错

```rust
#![allow(unused)]
fn main() {
   let s1 = String::from("hello");
   let h = s1[0];
}
```

对字符串切片是危险的

### 操作字符串

追加（String可用）
`push(char)`/`push_str(&str)`


插入（String可用）
`insert(idx: usize, char)`/`insert_str(idx: usize, &str)`

替换（String/&str可用）
`replace(needle: &str, haystack: &str)`/`replacen(needle: &str, haystack: &str, n)`

替换范围（String可用）
`replace_range(range, &str)`

```rust
fn main() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}
```

删除（String可用）

- `pop` - 删除并返回字符串的最后一个字符。其返回值是一个 `Option` 类型，如果字符串为空，则返回 `None`。


- `remove` —— 删除并返回字符串中指定位置的字符，`remove()` 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。

```rust
fn main() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}
```

- `truncate` —— 删除字符串中从指定位置开始到结尾的全部字符，无返回值

- `clear` —— 清空字符串，相当于`truncate(0)`

### 连接字符串

- 用 `+` 或 `+=`: `s = s1 + &s2` （s, s1, s2都是 `String`，&s2自动解引用为 `&str` 类型）

```rust
fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}
```

⚠️ 注意，之所以可以使用 `+` 连接字符串是因为，调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。`+` 和 `+=` 都是返回一个新的字符串。**所以变量声明可以不需要 mut 关键字修饰**。


⚠️ add() 定义
```rust
fn add(self, s: &str) -> String
```
因此
```rust
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}
```

- 使用 `format!` 连接字符串

```rust
fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

```

### 转义

使用 `\`

```rust
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

禁止转义 `r"..."`，包含双引号 `r#"..."#`

```rust
fn main() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

## 操作UTF-8字符串

遍历 `char`

```rust
for c in "中国人".chars() {
    println!("{}", c);
}
```

遍历 `byte(字节)`

```rust
for b in "中国人".bytes() {
    println!("{}", b);
}
```

取子字符串：[utf8_slice](https://crates.io/crates/utf8_slice)。

## 习题

如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
```rust
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(s)
}

fn greetings(s: Box<str>) {
    println!("{}", &s)
}
```
或
```rust
fn main() {
    let s: Box<&str> = "hello, world".into();
    greetings(*s)
}

fn greetings(s: &str) {
    println!("{}", s);
}
```

字节字符串 （Byte String）

```rust
use std::str;

fn main() {
    // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";


    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
```

练习：
```rust
// 填空
fn main() {
    let mut s = String::new();
    __;

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = __;


    assert_eq!(s, s1);

    println!("Success!")
}
```

答案
```rust
// FILL in the blanks
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    // some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a bytes vector into a String
    // We know these bytes are valid, so we'll use `unwrap()`.
    let s1 = String::from_utf8(v).unwrap();


    assert_eq!(s, s1);

    println!("Success!")
}
```

utf8_slice

```rust
use utf8_slice;
fn main() {
    let s = "The 🚀 goes to the 🌑!";

    let rocket = utf8_slice::slice(s, 4, 5);
    // 结果是 "🚀"
}
```


## 参考

https://doc.rust-lang.org/std/string/struct.String.html