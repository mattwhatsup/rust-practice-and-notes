## 三条消除规则
编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

1. 每一个引用参数都会获得独自的生命周期

  例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。

2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期

  例如函数 fn foo(x: &i32) -> &i32，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32

3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

  拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。

规则其实很好理解，但是，爱思考的读者肯定要发问了，例如第三条规则，若一个方法，它的返回值的生命周期就是跟参数 &self 的不一样怎么办？总不能强迫我返回的值总是和 &self 活得一样久吧？! 问得好，答案很简单：手动标注生命周期，因为这些规则只是编译器发现你没有标注生命周期时默认去使用的，当你标注生命周期后，编译器自然会乖乖听你的话。

## 方法中的生命周期

```rs

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

- impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
- 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则

有一点很容易推理出来：由于 &'a self 是被引用的一方，因此引用它的 &'b str 必须要活得比它短，否则会出现悬垂引用。因此说明生命周期 'b 必须要比 'a 小，只要满足了这一点，编译器就不会再报错：

```rs
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```


就关键点稍微解释下：

- 'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
- 可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系，如下：

```rs
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## 静态生命周期

- 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
- 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹

## 例子

```rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 练习题

### 4

```rs
/* 使用三种方法修复下面的错误  */
fn invalid_output() -> &str {
    "foo"
}

fn main() {
}
```

4.1 方法1
```rs
/* 使用三种方法修复下面的错误  */
fn invalid_output() -> String {
    String::from("foo")
}

fn main() {
}
```

4.2 方法2
```rs
/* 使用三种方法修复下面的错误  */
fn invalid_output() -> &'static str {
    "foo"
}

fn main() {
}
```

4.3 方法3
```rs
/* 使用三种方法修复下面的错误  */
fn invalid_output<'a>() -> &'a str {
    "foo"
}

fn main() {
}
```

### 5

```rs
// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* 让下面的代码工作 */
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` 活得不够久does not live long enough
    let y: &'a i32 = &_x;
    // 改成
    // let y: &i32 = &_x;

    // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
    // 你不能将一个小的生命周期强转成大的
}

fn main() {
    let (four, nine) = (4, 9);


    print_refs(&four, &nine);
    // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

    failed_borrow();
    // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
}
```

### 6

```rs
/* 增加合适的生命周期标准，让代码工作 */

// `i32` 的引用必须比 `Borrowed` 活得更久
#[derive(Debug)]
struct Borrowed(&i32);

// 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
#[derive(Debug)]
struct NamedBorrowed {
    x: &i32,
    y: &i32,
}

#[derive(Debug)]
enum Either {
    Num(i32),
    Ref(&i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

```

```rs
/* 增加合适的生命周期标准，让代码工作 */

// `i32` 的引用必须比 `Borrowed` 活得更久
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

### 8

```rs

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* 修复函数的签名 */
fn fix_me(foo: &Example) -> &NoCopyType
{ foo.b }

fn main()
{
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}

```

```rs
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
{ foo.b }
fn fix_me<'a: 'b, 'b>(foo: &'a Example) -> &'b NoCopyType
{ foo.b }
```

分析

```rs
#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}
```

因为这个struct声明的隐含条件实际上已经包括了`'b`的生命周期应该至少不小于结构体对象的生命周期长度

```rs
fn fix_me(foo: &Example) -> &NoCopyType
{ foo.b }
```

这个调用虽然看起来符合消除的第一个原则
但实际上foo自己有一个隐含的`'a`生命周期（应该和结构体里`&'a u32'`不同）（为了不混淆其实自己起个别的名字更好比如`'z`）
所以foo.b的生命周期其实和结构体的'a并不相同

因此下面这样写之所以能够成功，是因为在告诉编译器返回值的生命周期和结构体的生命周期一样，但像上面所描述的原因，它是不能被省略消除的。

```rs
/* 修复函数的签名 */
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
    foo.b
}
```