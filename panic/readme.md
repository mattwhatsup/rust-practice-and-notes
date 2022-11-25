## 使用panic!

```rust
fn main() {
    panic!("crash and burn");
}
```

若要看错误栈
- linux: `RUST_BACKTRACE=1 cargo run`
- windows: `$env:RUST_BACKTRACE=1 ; cargo run`

release版本减小发布尺寸
```
[profile.release]
panic = 'abort'
```

何时使用panic：你确切的知道你的程序是正确时，可以使用 panic