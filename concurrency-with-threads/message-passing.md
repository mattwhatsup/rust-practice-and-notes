# 线程同步

## 多发送者，单接收者
标准库提供了通道std::sync::mpsc，其中mpsc是multiple producer, single consumer的缩写

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}
```

- tx,rx对应发送者和接收者，它们的类型由编译器自动推导: tx.send(1)发送了整数，因此它们分别是mpsc::Sender<i32>和mpsc::Receiver<i32>类型，需要注意，由于内部是泛型实现，一旦类型被推导确定，该通道就只能传递对应类型的值, 例如此例中非i32类型的值将导致编译错误
- 接收消息的操作rx.recv()会阻塞当前线程，直到读取到值，或者通道被关闭
- 需要使用move将tx的所有权转移到子线程的闭包中

## 不阻塞的 try_recv 方法
除了上述recv方法，还可以使用try_recv尝试接收一次消息，该方法并不会阻塞线程，当通道中没有消息时，它会立刻返回一个错误：

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
}
```

## 传输具有所有权的数据
使用通道来传输数据，一样要遵循 Rust 的所有权规则：

若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
一起来看看第二种情况:

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s).unwrap();
        println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
以上代码中，String底层的字符串是存储在堆上，并没有实现Copy特征，当它被发送后，会将所有权从发送端的s转移给接收端的received，之后s将无法被使用。

## 使用 for 进行循环接收

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

当子线程运行完成时，发送者tx会随之被drop，此时for循环将被终止，最终main线程成功结束。

## 使用多发送者
由于子线程会拿走发送者的所有权，因此我们必须对发送者进行克隆，然后让每个线程拿走它的一份拷贝:

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```
代码并无太大区别，就多了一个对发送者的克隆let tx1 = tx.clone();，然后一个子线程拿走tx的所有权，另一个子线程拿走tx1的所有权，皆大欢喜。

但是有几点需要注意:

- 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
- 这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
- 由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定

## 同步和异步通道
Rust 标准库的mpsc通道其实分为两种类型：同步和异步。

### 异步通道
之前我们使用的都是异步通道：无论接收者是否正在接收消息，消息发送者在发送消息时都不会阻塞:

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx)= mpsc::channel();

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
```

### 同步通道
与异步通道相反，同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞，例如：

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx)= mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
```

> `let (tx, rx)= mpsc::sync_channel(n);`
> 该值`n`可以用来指定同步通道的消息缓存条数，当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息缓冲队列满了后，新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。

## 关闭通道
之前我们数次提到了通道关闭，并且提到了当通道关闭后，发送消息或接收消息将会报错。那么如何关闭通道呢？ 很简单：所有发送者被drop或者所有接收者被drop后，通道会自动关闭。

## 传输多种类型的数据
之前提到过，一个消息通道只能传输一种类型的数据，如果你想要传输多种类型的数据，可以为每个类型创建一个通道，你也可以使用枚举类型来实现：

```rs
use std::sync::mpsc::{self, Receiver, Sender};

enum Fruit {
    Apple(u8),
    Orange(String)
}

fn main() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}
```
如上所示，枚举类型还能让我们带上想要传输的数据，但是有一点需要注意，Rust 会按照枚举中占用内存最大的那个成员进行内存对齐，这意味着就算你传输的是枚举中占用内存最小的成员，它占用的内存依然和最大的成员相同, 因此会造成内存上的浪费。

## 新手容易遇到的坑
mpsc虽然相当简洁明了，但是在使用起来还是可能存在坑：

```rs
use std::sync::mpsc;
fn main() {

    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
```

通道关闭的两个条件：发送者全部drop或接收者被drop，要结束for循环显然是要求发送者全部drop，但是由于send自身没有被drop，会导致该循环永远无法结束，最终主线程会一直阻塞。

解决办法很简单，drop掉send即可：在代码中的注释下面添加一行drop(send);。

## mpmc 更好的性能
如果你需要 mpmc(多发送者，多接收者)或者需要更高的性能，可以考虑第三方库:

- crossbeam-channel, 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了crossbeam主仓库中
- flume, 官方给出的性能数据某些场景要比 crossbeam 更好些