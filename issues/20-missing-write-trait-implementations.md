# Issue 20: Missing `Write` Trait Implementations (`fmt::Write` and `io::Write`)

## Summary

When all variant types in `Or<T0, T1, ..., TN>` implement `core::fmt::Write` (or
`std::io::Write`), it is natural for `Or` to implement that trait by delegating to
the active variant. Currently neither `fmt::Write` nor `io::Write` are implemented,
making it impossible to use an `Or` value where a writer is expected.

## Location

**File:** `src/lib.rs`

No `fmt::Write` or `io::Write` implementations are generated in the `or!` macro.

## Why It Is a Problem

### A natural use case for sum types

A common use case for sum types is representing "one of several output targets":

```rust
fn write_output(output: Or2<File, Vec<u8>>, data: &str) -> io::Result<()> {
    write!(output, "{}", data)?;  // ERROR: Or2 does not implement io::Write
    Ok(())
}
```

Without `Write` impls, users must match every time they want to write:

```rust
match output {
    Or2::T0(ref mut f) => write!(f, "{}", data),
    Or2::T1(ref mut v) => write!(v, "{}", data),
}
```

This defeats the purpose of using `Or` to unify types.

### `fmt::Write` is `core`-compatible (no-std)

`core::fmt::Write` is available without `std`, so it can be implemented in `no_std`
builds. `std::io::Write` requires `std` and could be gated behind a `std` feature.

## Proposed Fix

### `core::fmt::Write`

Add inside `or!(@main ...)`:

```rust
impl<$($t: core::fmt::Write),*> core::fmt::Write for Or<$($t,)*> {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self {
            $(Self::$t(item) => item.write_str(s),)*
        }
    }

    #[inline]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        match self {
            $(Self::$t(item) => item.write_char(c),)*
        }
    }

    #[inline]
    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        match self {
            $(Self::$t(item) => item.write_fmt(args),)*
        }
    }
}
```

### `std::io::Write` (gated behind a `std` feature)

```rust
#[cfg(feature = "std")]
impl<$($t: std::io::Write),*> std::io::Write for Or<$($t,)*> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            $(Self::$t(item) => item.write(buf),)*
        }
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            $(Self::$t(item) => item.flush(),)*
        }
    }
}
```

This requires adding a `std` feature to `Cargo.toml`.

### `std::io::Read` (similar)

Similarly, `std::io::Read` and `std::io::Seek` could be implemented with the same
pattern, behind the `std` feature.

## Investigation Steps for the Implementing Agent

1. Add `core::fmt::Write` impl inside `or!(@main ...)` — no feature gate needed.
2. Add a `std` feature to `Cargo.toml` if `io::Write` support is desired.
3. Add `std::io::Write` and `std::io::Read` impls gated on `feature = "std"`.
4. Run `cargo test --all-features`.
5. Add tests:
   ```rust
   #[test]
   fn fmt_write_t0() {
       use core::fmt::Write;
       let mut buf = String::new();
       let mut or: orn::Or2<String, String> = orn::Or2::T0(String::new());
       write!(or, "hello {}", 42).unwrap();
       assert_eq!(or.t0().unwrap(), "hello 42");
   }
   ```
