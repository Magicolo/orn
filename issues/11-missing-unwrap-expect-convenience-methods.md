# Issue 11: Missing `unwrap` / `expect` Convenience Methods

## Summary

The per-variant accessor methods `.t0()`, `.t1()`, etc. return `Option<Tk>`, which is
correct but requires calling `.unwrap()` or `.expect()` separately when the caller
knows the variant. There are no convenience methods like `.unwrap_t0()` or
`.expect_t0("message")` that panic with a useful message if the wrong variant is
present. This is a minor ergonomics gap that is easy to address.

## Location

**File:** `src/lib.rs`

Inside `or!(@inner ...)`, for each variant:

```rust
pub fn $get(self) -> Option<$t> {
    match self {
        Self::$t(item) => Some(item),
        #[allow(unreachable_patterns)]
        _ => None
    }
}
```

There is no corresponding `unwrap_$get` or `expect_$get`.

## Why It Is a Problem

### Verbose unwrapping pattern

A very common pattern when interacting with `Or` values in tests or guaranteed-variant
contexts is:

```rust
let value = or.t0().expect("expected T0 variant");
```

This could be simplified to:

```rust
let value = or.expect_t0("expected T0 variant");
// or
let value = or.unwrap_t0();
```

### Panic message quality

Without a dedicated `unwrap_t0()`, a simple `.unwrap()` after `.t0()` gives the generic
panic message "called `Option::unwrap()` on a `None` value", which does not tell the
developer which variant was expected or what was actually found. A dedicated method can
provide a much more informative panic:

```
called `unwrap_t0()` on Or2::T1(...)
```

### Alignment with `Option` and `Result` conventions

The `Option` type has `unwrap()` and `expect("msg")`. For a type that wraps `Option`-
returning accessors, providing similar named variants is consistent with Rust conventions.

## Proposed Fix

Add two methods per variant inside `or!(@inner ...)`:

```rust
#[doc = concat!("Unwraps the `", stringify!($t), "` variant, panicking if it is not present.")]
///
/// # Panics
///
/// Panics if the value is not the expected variant.
#[inline]
#[track_caller]
pub fn $unwrap(self) -> $t {
    match self {
        Self::$t(item) => item,
        #[allow(unreachable_patterns)]
        other => panic!(
            concat!("called `", stringify!($unwrap), "()` on a `{}` value"),
            other.variant_name()
        ),
    }
}

#[doc = concat!("Unwraps the `", stringify!($t), "` variant, panicking with `msg` if it is not present.")]
///
/// # Panics
///
/// Panics with the provided message if the value is not the expected variant.
#[inline]
#[track_caller]
pub fn $expect(self, msg: &str) -> $t {
    match self {
        Self::$t(item) => item,
        #[allow(unreachable_patterns)]
        _ => panic!("{}", msg),
    }
}
```

Where `$unwrap` = `unwrap_t0`, `unwrap_t1`, … and `$expect` = `expect_t0`, `expect_t1`, ….

### Variant name helper

For the panic message in `unwrap_tN()`, a `variant_name()` helper method can be added:

```rust
pub fn variant_name(&self) -> &'static str {
    match self {
        $(Self::$t(_) => stringify!($t),)*
    }
}
```

Alternatively, the `Debug` impl already includes the variant name, so the panic could use:

```rust
panic!("called `unwrap_t0()` on {:?}", self)
```

(though this requires `Debug` to be implemented, which it is).

### Macro changes required

The `or!(@inner ...)` call site and the macro arm signature need two new identifier
parameters per variant: `$unwrap` and `$expect`. The invocation at the bottom of the
file would expand to:

```
0, T0, U0, F0, t0, is_t0, map_t0, unwrap_t0, expect_t0,
1, T1, U1, F1, t1, is_t1, map_t1, unwrap_t1, expect_t1,
...
```

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and find the `or!(@inner ...)` macro arm signature:
   ```
   (@inner $count: tt, $alias: ident, $module: ident, $index: tt, $t: ident, $get: ident, $is: ident, $map: ident [$(...)])
   ```
2. Add `$unwrap: ident` and `$expect: ident` parameters.
3. Update all call sites of `@inner` (inside `@outer`) to pass the new identifiers.
4. Update the large `or!(...)` invocations at the bottom of the file to include the
   new identifiers.
5. Add the method bodies as described above.
6. Run `cargo test --all-features`.
7. Add tests:
   ```rust
   #[test]
   fn unwrap_t0_ok() {
       let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
       assert_eq!(or.unwrap_t0(), 42u8);
   }

   #[test]
   #[should_panic]
   fn unwrap_t0_panics_on_wrong_variant() {
       let or: orn::Or2<u8, u16> = orn::Or2::T1(100u16);
       or.unwrap_t0();
   }

   #[test]
   fn expect_t0_ok() {
       let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
       assert_eq!(or.expect_t0("should be t0"), 42u8);
   }

   #[test]
   #[should_panic(expected = "should be t0")]
   fn expect_t0_panics_with_message() {
       let or: orn::Or2<u8, u16> = orn::Or2::T1(100u16);
       or.expect_t0("should be t0");
   }
   ```
