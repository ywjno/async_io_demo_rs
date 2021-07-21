本代码来自b站up主 [原子之音](https://space.bilibili.com/437860379) 的 [Rust异步io实战](https://www.bilibili.com/video/BV1Xv411L7Jx)

### How to use

异步的方式统计
```
cargo build --release --bin async_io
.\target\release\async_io.exe <PATH>
# or
# ./target/release/async_io <PATH>
```

同步的方式统计
```
cargo build --release --bin sync_io
.\target\release\sync_io.exe <PATH>
# or
# ./target/release/sync_io <PATH>
```
