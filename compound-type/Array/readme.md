## 数组

- 长度固定
- 元素必须有相同的类型
- 依次线性排列

## 创建
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
}
```

数组的类型是 [T; Length]，就如你所看到的，数组的长度是类型签名的一部分，因此数组的长度必须在编译期就已知

```rust
fn create_arr(n: i32) {
    let arr = [1; n];
}
```
👆函数将报错，因为编译器无法在编译期知道 n 的具体大小。

```rust
fn main() {
    // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // 填空
    // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
    // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
    assert!(std::mem::size_of_val(&arr) == 12);
}
```

当类型为非基础类型，👇代码错误

```rust
let array = [String::from("rust is good!"); 8];

println!("{:#?}", array);
```

> 基本类型在Rust中赋值是以Copy的形式，这时候你就懂了吧，let array=[3;5]底层就是不断的Copy出来的，但很可惜复杂类型都没有深拷贝，只能一个个创建。

```rust
let array: [String; 8] = core::array::from_fn(|i| String::from("rust is good!"));

println!("{:#?}", array);
```

## 获取

```rust
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // but indexing is not safe
    let _name1 = &names[1];
}
```

## 数组切片

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

- 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
- 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
- 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理

## 总结

```rust
fn main() {
  // 编译器自动推导出one的类型
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
}
```

- 数组类型容易跟数组切片混淆，[T;n]描述了一个数组的类型，而[T]描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
- [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
- 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小