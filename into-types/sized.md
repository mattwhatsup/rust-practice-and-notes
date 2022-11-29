## DST(dynamically sized types)

> 总结：只能间接使用的 DST
> Rust 中常见的 DST 类型有: str、[T]、dyn Trait，它们都无法单独被使用，必须要通过引用或者 Box 来间接使用 。

## Sized

几乎所有类型都实现了 Sized 特征

```rs
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

## `Box<str>`

```rs
let s1: Box<str> = "Hello there!".into();
```