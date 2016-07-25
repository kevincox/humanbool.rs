# Human Bool (for Rust)

A simple library to parse human entered booleans, especially those from environment variables.

Currently the following strings are supported:
- 1/0
- yes/no/y/n
- true/false/t/f
- on/off

## From a string
```rust
use humanbool::parse;
assert!(parse("y") == Ok(true));
```

## From the environment.

```rust
use humanbool::*;
assert_eq!(env("ENABLE_KITTENS", "f"), Ok(false));
std::env::set_var("ENABLE_KITTENS", "1");
assert!(env("ENABLE_KITTENS", "f") == Ok(true));

assert!(env("ENABLE_TURBO", "") == Err(Error::Empty));
```
