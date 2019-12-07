derive_asref
===

Gives you a variety way to derive `AsRef` trait.

## How to use

```rust
use derive_asref::AsRef;

#[derive(AsRef)]
struct Hoge {
  #[as_ref(target = "i64")]
  value: i64,
}
```
