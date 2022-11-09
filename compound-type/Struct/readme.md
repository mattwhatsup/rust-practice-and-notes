## 结构体

- 初始化实例时，每个字段都需要进行初始化
- 初始化时的字段顺序不需要和结构体定义时的顺序一致

```rust
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 访问
println!("{}", user1.username);

// 更新
user1.email = String::from("anotheremail@example.com");

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};

// user1 的部分字段所有权被转移到 user2 中：
//    username 字段发生了所有权转移，作为结果，user1 无法再被使用。
// println!("{:?}", user1); // 错误
println!("{:?}", user1.email); // 但user1.email仍然可以访问
println!("{:?}", user2); // 注意与js的扩展语法不同，email未被user1的值覆盖
```

🌟 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.

> 当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。

> 在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。

## 元组结构体(Tuple Struct)

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```

## 单元结构体(Unit-like Struct)

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {

}
```

## 结构体数据所有权

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
```

##  结构体数据生命周期

报错，因结构体的字段不知道数据的生命周期

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

修复
```rust
#[derive(Debug)]
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1); // 打印到stdout
    dbg!(user1); // 打印到stderr
}
```