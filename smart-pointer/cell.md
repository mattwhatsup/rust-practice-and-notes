# Cell & RefCell

## Cell

Cell 和 RefCell 在功能上没有区别，区别在于 `Cell<T>` 适用于 T 实现 Copy 的情况：

```rs
use std::cell::Cell;
fn main() {
  let c = Cell::new("asdf");
  let one = c.get();
  c.set("qwer");
  let two = c.get();
  println!("{},{}", one, two);
}
```

编译器会立刻报错，因为 String 没有实现 Copy 特征：

```rs
 let c = Cell::new(String::from("asdf"));
```

## RefCell

对比

| Rust 规则                            | 智能指针带来的额外规则                 |
| ------------------------------------ | -------------------------------------- |
| 一个数据只有一个所有者               | Rc/Arc 让一个数据可以拥有多个所有者    |
| 要么多个不可变借用，要么一个可变借用 | RefCell 实现编译期可变、不可变引用共存 |
| 违背规则导致编译错误                 | 违背规则导致运行时 panic               |


编译时不报错，运行时报错

```rs
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{},{}", s1, s2);
}
```


总之，当你确信编译器误报但不知道该如何解决时，或者你有一个引用类型，需要被四处使用和修改然后导致借用关系难以管理时，都可以优先考虑使用 RefCell。

### RefCell 简单总结

- 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
- RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
- RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
- 使用 RefCell 时，违背借用规则会导致运行期的 panic

### 选择 Cell 还是 RefCell

- Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
- Cell 不会 panic，而 RefCell 会

```rs
fn main() {

    // 1
    use std::cell::Cell;
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    // 2
    let mut x = 1;
    let y = &mut x;
    let z = &mut x;
    x = 2;
    *y = 3;
    *z = 4;
    println!("{}", x);
}
```

性能一致，但代码 1 拥有代码 2 不具有的优势：它能编译成功:)


与 Cell 的 zero cost 不同，RefCell 其实是有一点运行期开销的，总之，当非要使用内部可变性时，首选 Cell，只有你的类型没有实现 Copy 时，才去选择 RefCell。

## 内部可变性

```rs

// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
struct MsgQueue {
    msg_cache: Vec<String>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.push(msg)
    }
}
```

如上所示，外部库中定义了一个消息发送器特征 Messenger，它只有一个发送消息的功能：fn send(&self, msg: String)，因为发送消息不需要修改自身，因此原作者在定义时，使用了 &self 的不可变借用，这个无可厚非。

我们要在自己的代码中使用该特征实现一个异步消息队列，出于性能的考虑，消息先写到本地缓存(内存)中，然后批量发送出去，因此在 send 方法中，需要将消息先行插入到本地缓存 msg_cache 中。但是问题来了，该 send 方法的签名是 &self，因此上述代码会报错.

使用RefCell解决：

```rs
use std::cell::RefCell;
pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn main() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
}
```

## Rc和RefCell组合

```rs
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", on yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
```

## 通过 Cell::from_mut 解决借用冲突

- Cell::from_mut，该方法将 `&mut T` 转为 `&Cell<T>`
- Cell::as_slice_of_cells，该方法将 `&Cell<[T]>` 转为 `&[Cell<T>]`

报错：
```rs
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for num in nums.iter().filter(|&num| is_even(*num)) {
        nums[i] = *num;
        i += 1;
    }
    nums.truncate(i);
}
```

报错是因为同时借用了不可变与可变引用，你可以通过索引的方式来避免这个问题：

```rs
fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}
```

但是这样就违背我们的初衷了，毕竟迭代器会让代码更加简洁，那么还有其它的办法吗？

这时就可以使用 Cell 新增的这两个方法：


```rs
use std::cell::Cell;

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
```

## 总结
Cell 和 RefCell 都为我们带来了内部可变性这个重要特性，同时还将借用规则的检查从编译期推迟到运行期，但是这个检查并不能被绕过，该来早晚还是会来，RefCell 在运行期的报错会造成 panic。

RefCell 适用于编译器误报或者一个引用被在多个代码中使用、修改以至于难于管理借用关系时，还有就是需要内部可变性时。

从性能上看，RefCell 由于是非线程安全的，因此无需保证原子性，性能虽然有一点损耗，但是依然非常好，而 Cell 则完全不存在任何额外的性能损耗。

Rc 跟 RefCell 结合使用可以实现多个所有者共享同一份数据，非常好用，但是潜在的性能损耗也要考虑进去，建议对于热点代码使用时，做好 benchmark。