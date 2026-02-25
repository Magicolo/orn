# Issue 10: Missing `Error` Trait Integration

## Summary

There is no implementation of `core::error::Error` (or `std::error::Error`) for `Or`
types, even when all variant types implement `Error`. This prevents `Or<E0, E1, ..., EN>`
from being used as an error type in functions that return `Result<T, Or<E0, E1, ..., EN>>`.

## Location

**File:** `src/lib.rs`

No `Error` implementations are generated anywhere in the `or!` macro.

## Why It Is a Problem

### A primary use-case for sum types is error handling

One of the most natural uses of a union type is to represent "one of several error kinds":

```rust
fn parse_or_read(input: &str) -> Result<Config, Or2<ParseError, IoError>> {
    if condition {
        parse(input).map_err(Or2::T0)?
    } else {
        read_file(input).map_err(Or2::T1)?
    }
}
```

For this to be ergonomic, `Or2<ParseError, IoError>` should implement `Error`. Without
`Error`, the result type cannot be used with crates like `anyhow`, `thiserror`, or any
function that expects `Box<dyn Error>`.

### Cannot box as `dyn Error`

```rust
let err: Or2<ParseError, IoError> = ...;
let boxed: Box<dyn Error> = Box::new(err);  // ERROR: Or2 does not implement Error
```

### Inconsistency with wrapping convention

The `Display` impl (Issue 7) is a prerequisite for `Error`. Once `Display` is
implemented, adding `Error` is a small additional step.

## Important Consideration: `no_std` Compatibility

`core::error::Error` was stabilized in Rust 1.81 (released September 2024). Since this
crate uses `#![no_std]`, the `Error` impl should use `core::error::Error`:

```rust
#[cfg(feature = "std")]
impl<$($t: std::error::Error),*> std::error::Error for Or<$($t,)*> {}
```

or, for Rust 1.81+:

```rust
impl<$($t: core::error::Error),*> core::error::Error for Or<$($t,)*> {}
```

This crate already requires Rust 2021 edition. If the MSRV is at least 1.81, `core::error::Error`
can be used directly. Otherwise, a `std` feature gate or MSRV bump is needed.

## Proposed Fix

### 1. Add `Display` first (prerequisite)

See [Issue 7](07-missing-display-implementation.md). `Error` requires `Display`.

### 2. Add `Error` implementation

Inside `or!(@main ...)`, after the `Display` impl:

```rust
#[cfg(feature = "std")]
impl<$($t: std::error::Error),*> std::error::Error for Or<$($t,)*> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            $(Self::$t(item) => item.source(),)*
        }
    }
}
```

Or, if targeting Rust 1.81+:

```rust
impl<$($t: core::error::Error),*> core::error::Error for Or<$($t,)*> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            $(Self::$t(item) => item.source(),)*
        }
    }
}
```

### 3. Optionally add a `std` feature flag

If the MSRV is below 1.81, gate the impl behind `feature = "std"` and add a `std`
feature to `Cargo.toml`:

```toml
[features]
std = []
```

```rust
#[cfg(feature = "std")]
impl<$($t: std::error::Error),*> std::error::Error for Or<$($t,)*> { ... }
```

## Investigation Steps for the Implementing Agent

1. Check the current MSRV of the crate (look for `rust-version` in `Cargo.toml` or CI
   configuration).
2. Decide whether to use `core::error::Error` (Rust 1.81+) or `std::error::Error` with
   a `std` feature gate.
3. Implement `Display` first (Issue 7 must be completed or done together).
4. Add the `Error` impl inside `or!(@main ...)`.
5. Run `cargo test --all-features`.
6. Add a test:
   ```rust
   #[test]
   fn or2_is_error() {
       use std::error::Error;
       use std::fmt;

       #[derive(Debug)]
       struct E1;
       impl fmt::Display for E1 { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "E1") } }
       impl Error for E1 {}

       #[derive(Debug)]
       struct E2;
       impl fmt::Display for E2 { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "E2") } }
       impl Error for E2 {}

       let err: orn::Or2<E1, E2> = orn::Or2::T0(E1);
       let _: &dyn Error = &err;  // must compile
       assert_eq!(format!("{}", err), "E1");
   }
   ```
