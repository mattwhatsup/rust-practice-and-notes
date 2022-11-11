## 动态数组 Vector

### 创建

```rust
let v: Vec<i32> = Vec::new();
```

#### 容量

容量 capacity 是已经分配好的内存空间，用于存储未来添加到 Vec 中的元素。而长度 len 则是当前 Vec 中已经存储的元素数量。如果要添加新元素时，长度将要超过已有的容量，那容量会自动进行增长：Rust 会重新分配一块更大的内存空间，然后将之前的 Vec 拷贝过去，因此，这里就会发生新的内存分配( 目前 Rust 的容量调整策略是加倍，例如 2 -> 4 -> 8 ..)。

若这段代码会频繁发生，那频繁的内存分配会大幅影响我们系统的性能，最好的办法就是提前分配好足够的容量，尽量减少内存分配。

> 如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 创建动态数组，这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能

```rust
// 修复错误
fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...但是下面的代码会造成新的内存分配
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 20);


    // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!")
}

```

#### vec![] 宏

`vec![]`, `vec!()`, `vec!{}` 都可以

```rust
let v = vec![1, 2, 3];
```

#### 从数组

```rust
let arr: [u8; 3] = [1, 2, 3];
let v = Vec::from(arr);
```

#### from/into的用法
```rust
fn main() {
    // array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);


    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!")
 }
 ```

#### 用extend

```rust
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Success!")
}
```

### 操作

```rust
let mut v = Vec::new();
v.push(1); // 插入

// 读取
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {}", third),
    None => println!("去你的第三个元素，根本没有！"),
}

// 下标与get的区别
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100]; // panic!
let does_not_exist = v.get(100); // 返回Option

// 遍历
let v = vec![1, 2, 3];
for i in &v {
    println!("{}", i);
}

let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 10
}
```

遍历
```rust
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1
        } else {
            v.push(i + 2)
        }
    }

    assert_eq!(format!("{:?}",v), format!("{:?}", vec![2, 3, 4, 5, 6]));

    println!("Success!")
}


//Another solution

fn main() {
    let mut v = Vec::from([1, 2, 3,4,5]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
       v[i] +=1;
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}

fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))
    }

    for i in 0..5 {
       match v.get_mut(i) {
           Some(item) => *item += 1,
           None => v.push(i+2)
       }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
```

### 切片
与 String 的切片类似， Vec 也可以使用切片。如果说 Vec 是可变的，那它的切片就是不可变或者说只读的，我们可以通过 & 来获取切片。

在 Rust 中，将切片作为参数进行传递是更常见的使用方式，例如当一个函数只需要可读性时，那传递 Vec 或 String 的切片 &[T] / &str 会更加适合。

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    let slice2 = &v[0..];

    assert_eq!(slice1, slice2);

    // 切片是只读的 (不能修改长度，但可以对元素修改)
    // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..];
    // slice3.push(4);
    slice3[0] = 2;

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}
```

### 存储不同类型元素

用Option

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}
fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}
```

用特征对象

```rust
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
```