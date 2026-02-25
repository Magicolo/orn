# Issue 2: `map_with` Has a Non-Idiomatic Tuple Closure Argument

## Summary

The `map_with` method on `Or<T, T, ..., T>` (the uniform-type variant) accepts a
closure whose argument is a **tuple `(S, T)`** rather than two separate parameters.
This is inconsistent with every other API in the Rust ecosystem and makes the
method awkward to use.

## Location

**File:** `src/lib.rs`

Inside the `or!(@main ...)` macro arm, in the `impl<T> Or<$($same_t,)*>` block:

```rust
/// Maps an `Or<T, T...>` to an `Or<U, U...>` by applying a function with a state to the
/// contained value.
#[inline]
pub fn map_with<S, U, F: FnOnce((S, T)) -> U>(self, state: S, map: F) -> Or<$($same_u,)*> {
    match self {
        $(Self::$t(item) => Or::$t(map((state, item))),)*
    }
}
```

## Why It Is a Problem

### Rust idiom for "state + value" functions

The standard Rust convention for a "function that takes a state and a value" is **two
parameters**, not a tuple:

```rust
F: FnOnce(S, T) -> U
```

Examples from the standard library:
- `Iterator::fold(init, f)` — `f: FnMut(B, Self::Item) -> B`
- `Result::map_or_else(default, f)` — separate parameters
- `Option::map_or_else(default, f)` — separate parameters

The tuple form `FnOnce((S, T)) -> U` is unusual, forces callers to destructure in the
closure argument list, and cannot accept a plain `fn(S, T) -> U` function pointer without
an adapter.

### Closure syntax is cumbersome

With the current tuple API, the caller must write:

```rust
let mapped = or.map_with(10u16, |(state, x)| state + x as u16);
```

The idiomatic two-argument form would be:

```rust
let mapped = or.map_with(10u16, |state, x| state + x as u16);
```

### Named functions cannot be used directly

With the tuple form, an existing function `fn add(state: u16, x: u8) -> u16` cannot be
passed directly — a wrapper closure is required. With the two-argument form it would just
work:

```rust
fn add(state: u16, x: u8) -> u16 { state + x as u16 }

// Current API — does NOT compile:
or.map_with(10u16, add);  // error: type mismatch

// Idiomatic API — compiles directly:
or.map_with(10u16, add);
```

### Doctest shows awkward syntax

The doctest in `src/lib.rs` already shows the destructuring pattern:

```rust
let mapped = or.map_with(10, |(s, x)| s + x as u16);
```

This is a symptom of the design problem.

## Proposed Fix

Change the signature of `map_with` so the closure takes two separate parameters:

```rust
pub fn map_with<S, U, F: FnOnce(S, T) -> U>(self, state: S, map: F) -> Or<$($same_u,)*> {
    match self {
        $(Self::$t(item) => Or::$t(map(state, item)),)*
    }
}
```

Update the doctest accordingly:

```rust
/// ```
/// // ...
/// let mapped = or.map_with(10, |s, x| s + x as u16);
/// assert_eq!(mapped.into_inner(), 52u16);
/// ```
```

### Semver implications

This is a **breaking change** to the public API signature. Since it changes the `F` bound
from `FnOnce((S, T)) -> U` to `FnOnce(S, T) -> U`, any existing callers must update their
closures. It should be released under a major or minor version bump according to the
project's semver policy.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and search for `map_with` (the method definition is inside
   `or!(@main ...)`).
2. Change `F: FnOnce((S, T)) -> U` to `F: FnOnce(S, T) -> U`.
3. Change the body from `map((state, item))` to `map(state, item)`.
4. Update the `#[doc = ...]` doctest line that shows the usage from
   `|(s, x)| s + x as u16` to `|s, x| s + x as u16`.
5. Run `cargo test --all-features` to ensure all doc-tests pass.
6. Bump the crate version according to semver policy.
