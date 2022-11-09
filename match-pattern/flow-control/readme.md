## if

- if 语句块是表达式
- 用 if 来赋值时，如果返回类型不一致就会报错

```rust
let mut v = 0;
for i in 1..10 {
    v = if i == 9 {
        continue // 没有返回值
    } else {
        i
    }
}
println!("{}", v);
```

## for

注意，使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）。如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了)：

```rust
for item in &container {
  // ...
}
```

> 对于实现了 copy 特征的数组(例如 [i32; 10] )而言， for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr 。

如果想在循环中，修改该元素，可以使用 mut 关键字：

```rust
for item in &mut collection {
  // ...
}
```


|使用方法	| 等价使用方式 |	所有权|
|--|--|--|
|for item in collection	| for item in IntoIterator::into_iter(collection)	|转移所有|权
|for item in &collection	|for item in collection.iter()	|不可变借用|
|for item in &mut collection|	for item in collection.iter_mut()	|可变借用|

如果想在循环中获取元素的索引使用`iter().enumerate()`：

```rust
fn main() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
```

两种循环方式优劣对比

以下代码，使用了两种循环方式：


```rust
// 第一种
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
  let item = collection[i];
  // ...
}

// 第二种
for item in collection {

}
```

- 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
- 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险（这里是因为所有权吗？是的话可能要强调一下）

## while

while和for对比

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

for 并不会使用索引去访问数组，因此更安全也更简洁，同时避免 运行时的边界检查，性能更高。

## loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

- break 可以单独使用，也可以带一个返回值，有些类似 return
- loop 是一个表达式，因此可以返回一个值