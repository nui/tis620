# TIS-620

[![Latest Version](https://img.shields.io/crates/v/tis620.svg)](https://crates.io/crates/tis620)
[![Rust Documentation](https://docs.rs/tis620/badge.svg)](https://docs.rs/tis620)
![Crates.io](https://img.shields.io/crates/l/tis620)
![Crates.io](https://img.shields.io/crates/d/tis620)

A library for handling Thai Industrial Standard 620 (TIS-620) characters.

Example usage.

```rust
let message = "แมว";
let encoded = tis620::encode(&message).expect("TIS-620 encoded");
let decoded = tis620::decode(&encoded).expect("Original message");
assert_eq!(decoded, message);
```

[more examples](https://github.com/nui/tis620/tree/main/examples)

---
This crate is inspired by [varokas/tis620](https://github.com/varokas/tis620).
