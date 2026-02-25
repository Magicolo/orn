# Issue 4: Missing `TryFrom<OrN<...>>` for Each Variant Type

## Summary

There is no `TryFrom<Or<T0, T1, ..., TN>>` implementation for any of the individual
variant types `T0`, `T1`, …, `TN`. Users who receive an `Or` value and wish to extract
a specific variant via the standard `TryFrom`/`TryInto` trait cannot do so, forcing
them to call the bespoke `.t0()`, `.t1()`, … methods instead.

## Location

**File:** `src/lib.rs`

No `TryFrom` implementations are generated anywhere in the `or!` macro.

## Why It Is a Problem

### Standard Rust fallible conversion idiom

`TryFrom<Source>` is the standard mechanism for fallible conversion in Rust:

```rust
let value: Result<u8, _> = some_or_value.try_into();
```

Without `TryFrom` impls:
- The `?` operator cannot be used to propagate variant-extraction failures.
- Generic code with `T: TryFrom<Or<...>>` bounds cannot be written.
- Interoperability with APIs that expect `TryFrom`/`TryInto` is impossible.

### Current workaround

Currently users must call `.t0()` and handle the `Option`:

```rust
let t0: Option<u8> = or_value.t0();
let value = t0.ok_or(MyError)?;
```

With `TryFrom`:

```rust
let value: u8 = or_value.try_into().map_err(|_| MyError)?;
```

### Enables generic programming

Many error-handling and data-transformation patterns benefit from `TryFrom`:

```rust
fn extract<T, Source>(src: Source) -> Result<T, Source>
where
    T: TryFrom<Source, Error = Source>,
{
    src.try_into()
}
```

## Important Consideration: Ambiguity with Duplicate Types

As with `From<Tk>` (see Issue 3), `TryFrom<Or<T0, T1, ..., TN>>` for `Tk` will create
ambiguity when multiple type parameters are the same type. For example,
`u8::try_from(or2_u8_u8_value)` cannot compile because both `TryFrom<Or<u8, u8>>` for
`u8` via the `T0` impl and via the `T1` impl conflict.

This is an inherent limitation of the Rust type system. Document this clearly in the
method and trait impl docs.

## Proposed Fix

Inside the `or!(@inner ...)` macro arm (alongside the `From<Tk>` impl from Issue 3),
add:

```rust
impl<$($ts),*> TryFrom<Or<$($ts,)*>> for $t {
    /// The original `Or` value is returned on failure, so the caller
    /// can inspect which variant was actually present.
    type Error = Or<$($ts,)*>;

    #[inline]
    fn try_from(value: Or<$($ts,)*>) -> Result<Self, Self::Error> {
        match value {
            Or::$t(item) => Ok(item),
            other => Err(other),
        }
    }
}
```

The `Error` type is set to the original `Or` value, preserving the original data for
the caller to handle. This is consistent with how the standard library models fallible
conversions (e.g. `String::try_from(Vec<u8>)` returns `Err(Vec<u8>)` on failure).

### Import required

`core::convert::TryFrom` must be in scope. Since the file already uses `use core::ops::{Deref, DerefMut};`, add:

```rust
use core::convert::TryFrom;
```

or reference it as `core::convert::TryFrom` directly in the macro expansion.

Note: In Rust 2021 edition (which this crate uses), `TryFrom` and `TryInto` are in the
prelude, so no explicit import is needed in user code — but an explicit `use` may be
needed inside the macro to avoid ambiguity.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and locate the `or!(@inner ...)` macro arm.
2. After the `From<$t> for Or<$($ts,)*>` impl (Issue 3), add the `TryFrom` impl shown
   above.
3. Ensure `core::convert::TryFrom` is accessible within the macro expansion context.
   In Rust 2021 edition it is in the prelude, so no extra `use` is needed.
4. Run `cargo test --all-features`.
5. Add tests in `tests/or.rs`:

```rust
#[test]
fn try_from_ok() {
    let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
    let result: Result<u8, _> = u8::try_from(or);
    assert_eq!(result, Ok(42u8));
}

#[test]
fn try_from_err() {
    let or: orn::Or2<u8, u16> = orn::Or2::T1(100u16);
    let result: Result<u8, _> = u8::try_from(or);
    assert!(result.is_err());
    // The original Or value is returned:
    assert_eq!(result.unwrap_err(), orn::Or2::T1(100u16));
}
```

6. Document the duplicate-type limitation in the trait impl docs.
