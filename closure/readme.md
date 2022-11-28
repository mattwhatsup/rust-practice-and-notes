## closure 闭包

```rs
|param1, param2,...| {
    语句1;
    语句2;
    返回表达式
}
```

闭包可以享受编译器的类型推导能力，无需标注参数和返回值的类型。

## 结构体中的闭包

```rs
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

```rs
struct Cacher<T, E: Clone>
where
    T: Fn(E) -> E,
{
    query: T,
    value: Option<E>,
}

impl<T, E: Clone> Cacher<T, E>
where
    T: Fn(E) -> E,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> E {
        match self.value.clone() {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                let ret = v.clone();
                self.value = Some(v);
                ret
            }
        }
    }
}
```

## 三种 Fn 特征

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用

### FnOnce

FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次

```rs
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4)); // <-- 仅实现 FnOnce 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```


func 的类型 F 实现了 Copy 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。
```rs
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,// 改动在这里
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```

如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。


```rs
use std::thread;
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
handle.join().unwrap();
```


### FnMut

FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值：

```rs
fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str); // <-- 错误声明
    update_string("hello");

    println!("{:?}",s);
}
```


```rs
fn main() {
    let mut s = String::new();

    let mut update_string =  |str| s.push_str(str);
    //  +++
    update_string("hello");

    println!("{:?}",s);
}
```

### Fn

Fn 特征，它以不可变借用的方式捕获环境中的值

```rs
fn main() {
    let s = "hello, ".to_string();

    let update_string =  |str| println!("{},{}",s,str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: Fn(String) -> ()>(f: F)  {
    f("world".to_string())
}
```

### Fn和move

```rs
fn main() {
    let s = String::new();

    let update_string =  move || println!("{}",s);

    exec(update_string);

    println!("{}", s); // <-- 报错
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

//如果改为Fn特征仍然可以
fn exec<F: Fn()>(f: F)  {
    f()
}
```

### 小结

一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
- 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
- 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
- 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征


```rs
fn main() {
    let s = String::new();

    let update_string =  || println!("{}",s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}
```

关于第二条规则，有如下示例：
```rs
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s }; // 移动了s的所有权，所以自动实现了FnOnce（跟是否使用 move 没有必然联系），但是下面要求的是FnMut

    exec(update_string);
}

fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}
```

修改

```rs
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) {
    f("hello");
}
```

> 感觉三种特征的限制优先级 FnOnce > FnMut > Fn

我们来看看这三个特征的简化版源码：


```rs
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

> 从特征约束能看出来 Fn 的前提是实现 FnMut，FnMut 的前提是实现 FnOnce，因此要实现 Fn 就要同时实现 FnMut 和 FnOnce，这段源码从侧面印证了之前规则的正确性。
> 从源码中还能看出一点：Fn 获取 &self，FnMut 获取 &mut self，而 FnOnce 获取 self。 在实际项目中，建议先使用 Fn 特征，然后编译器会告诉你正误以及该如何选择。

## 闭包作为函数返回值

常见错误
```rs
fn factory() -> Fn(i32) -> i32 {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

```rs
fn factory() -> impl Fn(i32) -> i32 {
  //            ^^^^
    let num = 5;

    move |x| x + num
  //^^^^
}
fn main() {
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}
```


```rs
fn factory(b: bool) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if b {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

fn main() {
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}
```