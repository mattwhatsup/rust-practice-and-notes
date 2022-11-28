## for是语法糖

```rs
let arr = [1, 2, 3];
for v in arr {
    println!("{}",v);
}
// 等同于
let arr = [1, 2, 3]; // 数组实现了IntoIterator特征
for v in arr.into_iter() {
    println!("{}", v);
}
```

## Iterator特征和next方法

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 省略其余有默认实现的方法
}
```

- next 方法返回的是 Option 类型，当有值时返回 Some(i32)，无值时返回 None
- 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
- 手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成

```rs
fn main() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}
```

## 模拟实现for循环

```rs
let values = vec![1, 2, 3];

{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{}", x); },
                None => break,
            }
        },
    };
    result
}
```

> IntoIterator::into_iter 是使用完全限定的方式去调用 into_iter 方法，这种调用方式跟 values.into_iter() 是等价的。

## into_iter, iter, iter_mut

- into_iter 会夺走所有权
- iter 是借用
- iter_mut 是可变借用

```rs
fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}
```

- .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
- .iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)，因此在 if let Some(v) = values_iter_mut.next() 中，v 的类型是 &mut i32，最终我们可以通过 *v = 0 的方式修改其值

## Iterator 和 IntoIterator 的区别
- Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next。
- IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器。


## 消费者与适配器

### 消费性适配器

只要迭代器上的某个方法 A 在其内部调用了 next 方法，那么 A 就被称为消费性适配器：因为 next 方法会消耗掉迭代器上的元素，所以方法 A 的调用也会消耗掉迭代器上的元素。

```rs
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}",v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
}
```

### 迭代性适配器

迭代器适配器，顾名思义，会返回一个新的迭代器，迭代器适配器是惰性的，意味着你需要一个消费者适配器来收尾


```rs
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

```rs
use std::{collections::HashMap, hash::Hash};
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks = names.into_iter().zip(ages.into_iter()).collect::<HashMap<_,_>>();

    println!("{:?}",folks);
}
```

## 实现Iterator

```rs
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```