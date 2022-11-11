## 类型转换


> 使用类型转换需要小心，因为如果执行以下操作 300_i32 as i8，你将获得 44 这个值，而不是 300，因为 i8 类型能表达的的最大值为 2^7 - 1，使用以下代码可以查看 i8 的最大值：


```rust
let a = i8::MAX;
println!("{}",a);
```

```rust
fn main() {
   let a = 3.1 as i8;
   let b = 100_i8 as i32;
   let c = 'a' as u8; // 将字符'a'转换为整数，97

   println!("{},{},{}",a,b,c)
}
```

```rust
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as u8 as char;

    assert_eq!(integer, 'a' as u8);

    println!("Success!")
}
```

默认情况下, 数值溢出会导致编译错误，但是我们可以通过添加一行全局注解的方式来避免编译错误(溢出还是会发生)

```rust
#![allow(overflowing_literals)]

fn main() {
    assert_eq!(u8::MAX, 255);
    let v = 1000 as u8;
}
```


当将任何数值转换成无符号整型 T 时，如果当前的数值不在新类型的范围内，我们可以对当前数值进行加值或减值操作( 增加或减少 T::MAX + 1 )，直到最新的值在新类型的范围内，假设我们要将 300 转成 u8 类型，由于u8 最大值是 255，因此 300 不在新类型的范围内并且大于新类型的最大值，因此我们需要减去 T::MAX + 1，也就是 300 - 256 = 44。
```rust
#![allow(overflowing_literals)]
fn main() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);


    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);


    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```


内存地址转换为指针

```rust
let mut values: [i32; 2] = [1, 2];
let p1: *mut i32 = values.as_mut_ptr();
let first_address = p1 as usize; // 将p1内存地址转换为一个整数
let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
unsafe {
    *p2 += 1;
}
assert_eq!(values[1], 3);
```

```rust
fn main() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13)
    }
}
```

> 转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）。

## TryInto 转换

在一些场景中，使用 as 关键字会有比较大的限制。如果你想要在类型转换上拥有完全的控制而不依赖内置的转换，例如处理转换错误，那么可以使用 TryInto ：

```rust
use std::convert::TryInto;

fn main() {
   let a: u8 = 10;
   let b: u16 = 1500;

   let b_: u8 = b.try_into().unwrap();

   if a < b_ {
     println!("Ten is less than one hundred.");
   }
}
```

> 上面代码中引入了 std::convert::TryInto 特征，但是却没有使用它，可能有些同学会为此困惑，主要原因在于如果你要使用一个特征的方法，那么你需要引入该特征到当前的作用域中，我们在上面用到了 try_into 方法，因此需要引入对应的特征。但是 Rust 又提供了一个非常便利的办法，把最常用的标准库中的特征通过std::prelude模块提前引入到当前作用域中，其中包括了 std::convert::TryInto，你可以尝试删除第一行的代码 use ...，看看是否会报错。

```rust
fn main() {
    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}
```

From 特征允许让一个类型定义如何基于另一个类型来创建自己，因此它提供了一个很方便的类型转换的方式。

From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：只要实现了 impl From<T> for U， 就可以使用以下两个方法: let u: U = U::from(T) 和 let u:U = T.into()，前者由 From 特征提供，而后者由自动实现的 Into 特征提供。

需要注意的是，当使用 into 方法时，你需要进行显式地类型标注，因为编译器很可能无法帮我们推导出所需的类型。

```rust
fn main() {
    let my_str = "hello";

    // 以下三个转换都依赖于一个事实：String 实现了 From<&str> 特征
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // 这里需要显式地类型标注
    let string3: String = my_str.into();
}
```

```rust
// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self{value: n}
    }
}

// 填空
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!")
}
```

当执行错误处理时，为我们自定义的错误类型实现 From 特征是非常有用。这样就可以通过 ? 自动将某个错误类型转换成我们自定义的错误类型

```rust
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!")
}
```

```rust
// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!")
}
```

```rust
#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // 实现 `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!")
}
```

## 通用类型转换

### 强制类型转换

首先，在匹配特征时，不会做任何强制转换(除了方法)。一个类型 T 可以强制转换为 U，不代表 impl T 可以强制转换为 impl U，例如下面的代码就无法通过编译检查：
```rust
trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() {
    let t: &mut i32 = &mut 0;
    foo(t); // 报错
}
```
&i32 实现了特征 Trait， &mut i32 可以转换为 &i32，但是 &mut i32 依然无法作为 Trait 来使用。

### 点操作符

- 首先，编译器检查它是否可以直接调用 T::foo(value)，称之为值方法调用
- 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现，上文提到过特征不能进行强制转换)，那么编译器会尝试增加自动引用，例如会尝试以下调用： <&T>::foo(value) 和 <&mut T>::foo(value)，称之为引用方法调用
- 若上面两个方法依然不工作，编译器会试着解引用 T ，然后再进行尝试。这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U)，那么编译器会使用 U 类型进行尝试，称之为解引用方法调用
- 若 T 不能被解引用，且 T 是一个定长类型(在编译器类型长度是已知的)，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
- 若还是不行，那...没有那了，最后编译器大喊一声：汝欺我甚，不干了！

#### 例1
```rust
let array: Rc<Box<[T; 3]>> = ...;
let first_entry = array[0];
```

array 数组的底层数据隐藏在了重重封锁之后，那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁，准确的访问到数组中的第一个元素？

- 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
- 接着，编译器检查 Rc<Box<[T; 3]>> 是否有实现 Index 特征，结果是否，不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
- 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
- 此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]> 都没有实现 Index 特征，所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
- [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，它也不能再进行解引用，因此编译器只能祭出最后的大杀器：将定长转为不定长，因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 方法访问到对应的元素。

#### 例2

```rust
fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
```

上面例子中 cloned 的类型是什么？首先编译器检查能不能进行值方法调用， value 的类型是 &T，同时 clone 方法的签名也是 &T ： fn clone(&T) -> T，因此可以进行值方法调用，再加上编译器知道了 T 实现了 Clone，因此 cloned 的类型是 T。

如果 T: Clone 的特征约束被移除呢？


```rust
fn do_stuff<T>(value: &T) {
    let cloned = value.clone();
}
```

首先，从直觉上来说，该方法会报错，因为 T 没有实现 Clone 特征，但是真实情况是什么呢？

我们先来推导一番。 首先通过值方法调用就不再可行，因为 T 没有实现 Clone 特征，也就无法调用 T 的 clone 方法。接着编译器尝试引用方法调用，此时 T 变成 &T，在这种情况下， clone 方法的签名如下： fn clone(&&T) -> &T，接着我们现在对 value 进行了引用。 编译器发现 &T 实现了 Clone 类型(所有的引用类型都可以被复制，因为其实就是复制一份地址)，因此可以推出 cloned 也是 &T 类型。

最终，我们复制出一份引用指针，这很合理，因为值类型 T 没有实现 Clone，只能去复制一个指针了。

#### 例3
下面的例子也是自动引用生效的地方：
```rust
#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}
```

推断下上面的 foo_cloned 和 bar_cloned 是什么类型？提示: 关键在 Container 的泛型参数，一个是 i32 的具体类型，一个是泛型类型，其中 i32 实现了 Clone，但是 T 并没有。

首先要复习一下复杂类型派生 Clone 的规则：一个复杂类型能否派生 Clone，需要它内部的所有子类型都能进行 Clone。因此 Container<T>(Arc<T>) 是否实现 Clone 的关键在于 T 类型是否实现了 Clone 特征。

上面代码中，Container<i32> 实现了 Clone 特征，因此编译器可以直接进行值方法调用，此时相当于直接调用 foo.clone，其中 clone 的函数签名是 fn clone(&T) -> T，由此可以看出 foo_cloned 的类型是 Container<i32>。

然而，bar_cloned 的类型却是 &Container<T>，这个不合理啊，明明我们为 Container<T> 派生了 Clone 特征，因此它也应该是 Container<T> 类型才对。万事皆有因，我们先来看下 derive 宏最终生成的代码大概是啥样的：


```rust
impl<T> Clone for Container<T> where T: Clone {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
```
从上面代码可以看出，派生 Clone 能实现的根本是 T 实现了Clone特征：where T: Clone， 因此 Container<T> 就没有实现 Clone 特征。

编译器接着会去尝试引用方法调用，此时 &Container<T> 引用实现了 Clone，最终可以得出 bar_cloned 的类型是 &Container<T>。

当然，也可以为 Container<T> 手动实现 Clone 特征：


```rust
impl<T> Clone for Container<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
```
此时，编译器首次尝试值方法调用即可通过，因此 bar_cloned 的类型变成 Container<T>。



### 将任何类型转换成 String

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}
```

### 使用 parse 方法可以将一个 String 转换成 i32 数字，这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32

```rust
// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed:i32 = "10".parse().unwrap();
    let from_str:i32 = "20".parse().unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}
```

### 还可以为自定义类型实现 FromStr 特征

```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}
fn main() {
    // 使用两种方式填空
    // 不要修改其它地方的代码
    let p = "(3,4)";
    // let p = Point::from_str("(3,4)");
    assert_eq!(p.parse::<Point>().unwrap(), Point{ x: 3, y: 4} );

    println!("Success!")
}



### 变形记(Transmutes)

 mem::transmute<T, U> 将类型 T 直接转成类型 U，唯一的要求就是，这两个类型占用同样大小的字节数

- 首先也是最重要的，转换后创建一个任意类型的实例会造成无法想象的混乱，而且根本无法预测。不要把 3 转换成 bool 类型，就算你根本不会去使用该 bool 类型，也不要去这样转换
- 变形后会有一个重载的返回类型，即使你没有指定返回类型，为了满足类型推导的需求，依然会产生千奇百怪的类型
- 将 & 变形为 &mut 是未定义的行为
  - 这种转换永远都是未定义的
  - 不，你不能这么做
  - 不要多想，你没有那种幸运
- 变形为一个未指定生命周期的引用会导致无界生命周期
- 在复合类型之间互相变换时，你需要保证它们的排列布局是一模一样的！一旦不一样，那么字段就会得到不可预期的值，这也是未定义的行为，至于你会不会因此愤怒， WHO CARES ，你都用了变形了，老兄！

将裸指针变成函数指针：

```rust
fn foo() -> i32 {
    0
}

let pointer = foo as *const ();
let function = unsafe {
    // 将裸指针转换为函数指针
    std::mem::transmute::<*const (), fn() -> i32>(pointer)
};
assert_eq!(function(), 0);
```

延长生命周期，或者缩短一个静态生命周期寿命：

```rust
struct R<'a>(&'a i32);

// 将 'b 生命周期延长至 'static 生命周期
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

// 将 'static 生命周期缩短至 'c 生命周期
unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}
```

### 事实上我们还可以使用一些安全的方法来替代 transmute.

```rust
fn main() {
    /*Turning raw bytes(&[u8]) to u32, f64, etc.: */
    let raw_bytes = [0x78, 0x56, 0x34, 0x12];

    let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };

    // use `u32::from_ne_bytes` instead
    let num = u32::from_ne_bytes(raw_bytes);
    // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    let num = u32::from_le_bytes(raw_bytes);
    assert_eq!(num, 0x12345678);
    let num = u32::from_be_bytes(raw_bytes);
    assert_eq!(num, 0x78563412);

    /*Turning a pointer into a usize: */
    let ptr = &0;
    let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };

    // Use an `as` cast instead
    let ptr_num_cast = ptr as *const i32 as usize;

    /*Turning an &mut T into an &mut U: */
    let ptr = &mut 0;
    let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };

    // Now, put together `as` and reborrowing - note the chaining of `as`
    // `as` is not transitive
    let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };

    /*Turning an &str into a &[u8]: */
    // this is not a good way to do this.
    let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    assert_eq!(slice, &[82, 117, 115, 116]);

    // You could use `str::as_bytes`
    let slice = "Rust".as_bytes();
    assert_eq!(slice, &[82, 117, 115, 116]);

    // Or, just use a byte string, if you have control over the string
    // literal
    assert_eq!(b"Rust", &[82, 117, 115, 116]);
}
```