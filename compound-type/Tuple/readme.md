## 元组

元组（Tuple）的长度是固定的，元组中元素的顺序也是固定的

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

### 解构

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

### 访问

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

## 超长无法打印

```rust
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}
```