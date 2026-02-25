# Issue 7: Missing `fmt::Display` Implementation

## Summary

All `Or<T0, T1, ..., TN>` types derive `Debug` but do not implement `fmt::Display`.
This makes it impossible to use `Or` values in format strings with `{}`, in error
messages, or anywhere else a `Display` impl is required. When all variant types
implement `Display`, it is natural for `Or` to delegate to the active variant.

## Location

**File:** `src/lib.rs`

The `#[derive(...)]` line inside `or!(@main ...)`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or<$($t,)*> { ... }
```

No `Display` implementation is generated.

## Why It Is a Problem

### Common usage patterns are broken

```rust
use orn::Or2;
let or: Or2<u8, &str> = Or2::T0(42u8);
println!("{}", or);  // ERROR: Or2 does not implement Display
```

Users must instead write:
```rust
println!("{:?}", or);  // works, but gives Debug output: "T0(42)"
// or
match or {
    Or2::T0(v) => println!("{}", v),
    Or2::T1(v) => println!("{}", v),
}
```

### Inconsistency with `Debug`

It is standard practice in Rust to implement `Display` alongside `Debug` for types
whose contents can be meaningfully shown to end users. Providing `Debug` without
`Display` leaves the API incomplete.

### Cannot be used with `std::fmt::Write` / `format!`

Anywhere that requires `impl Display` (e.g. `thiserror` error messages, log crates,
`anyhow` context messages) cannot use `Or` values directly without manual unwrapping.

### Asymmetry with `serde` feature

The `serde` feature allows serializing/deserializing `Or` values, which implies the
values have a meaningful string representation, yet `Display` is absent.

## Proposed Fix

Add a `Display` implementation inside the `or!(@main ...)` macro arm. The implementation
matches on the active variant and delegates to its `Display` impl:

```rust
impl<$($t: core::fmt::Display,)*> core::fmt::Display for Or<$($t,)*> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            $(Self::$t(item) => core::fmt::Display::fmt(item, f),)*
        }
    }
}
```

This is a purely additive change (new trait impl) and is backwards-compatible.

### Behaviour

`Or2::<u8, &str>::T0(42u8)` will format as `"42"` (just the inner value, no wrapper),
which is the most useful default. The `Debug` impl already provides the variant-tagged
representation `"T0(42)"` for diagnostics.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and locate `or!(@main ...)`.
2. After the `impl<T, $($t: AsMut<T>),*> AsMut<T>` block (approximately line 425–435),
   add the `Display` implementation shown above.
3. Verify `core::fmt` is accessible — it already is via `#![no_std]` and `use core::...`
   patterns in the file.
4. Run `cargo test --all-features`.
5. Add tests:
   ```rust
   #[test]
   fn display_t0() {
       let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
       assert_eq!(format!("{}", or), "42");
   }

   #[test]
   fn display_t1() {
       let or: orn::Or2<u8, u16> = orn::Or2::T1(100u16);
       assert_eq!(format!("{}", or), "100");
   }

   #[test]
   fn display_str_variant() {
       let or: orn::Or2<&str, u8> = orn::Or2::T0("hello");
       assert_eq!(format!("{}", or), "hello");
   }
   ```
6. Add a doc-test in the `Display` impl documentation.
