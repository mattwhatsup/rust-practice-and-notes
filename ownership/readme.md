## 所有权

- Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
- 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
- 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

实现了Copy Trait的类型（基本类型）

- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
- 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的

## 借用

获取变量的引用，称之为借用(borrowing)

解引用：`let x = 5;let y = &x;` *y 来解出引用所指向的值（也就是解引用）

## 不可变引用

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

ref

```rust
fn main() {
    let c = '中';

    let r1 = &c;
    // fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```

## 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**同一作用域，特定数据只能有一个可变引用**

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// r1 作用域已经结束
let r2 = &mut s;

println!("{}, {}", r1, r2); // 报错
```

**可变引用与不可变引用不能同时存在**

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);
```

NLL：Non-Lexical Lifetimes(NLL) 专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。


## 悬垂引用(Dangling References)

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // 错误
}
```

## 总结

- 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
- 引用必须总是有效的

