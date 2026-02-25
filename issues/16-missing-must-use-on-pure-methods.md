# Issue 16: Missing `#[must_use]` on Pure Methods

## Summary

Many methods on `Or<T0, T1, ..., TN>` that return values without side effects are
missing the `#[must_use]` attribute. This means the Rust compiler will not warn when
these methods are called but the result is discarded, potentially hiding bugs.

## Location

**File:** `src/lib.rs`

Examples of methods lacking `#[must_use]`:

- `fn t0(self) -> Option<T0>` — pure, result should always be used
- `fn is_t0(&self) -> bool` — pure predicate
- `fn map_t0(self, f) -> Or<...>` — pure transformation
- `fn into<T>(self) -> T` — pure conversion
- `fn map<U, F>(self, f) -> Or<...>` — pure transformation
- `fn into_inner(self) -> T` — pure extraction
- `fn as_ref(&self) -> Or<&T...>` — pure borrow conversion
- `fn as_deref(&self) -> Or<&T::Target, ...>` — pure borrow conversion
- `fn cloned(self) -> Or<T...>` — pure clone operation

## Why It Is a Problem

### Silent discard of important results

Without `#[must_use]`, the compiler does not warn when a programmer accidentally
discards a return value. This can lead to subtle bugs:

```rust
// Bug: result of t0() is discarded; programmer likely forgot to use it
or.t0();  // No warning!

// Bug: map result discarded
or.map_t0(|x| expensive_computation(x));  // No warning!

// Bug: is_t0() result discarded (often indicates a confused check)
or.is_t0();  // No warning!
```

### Standard library precedent

The Rust standard library annotates all pure, side-effect-free methods with
`#[must_use]`. For example:
- `Option::unwrap`, `Option::map`, `Option::is_some` — all `#[must_use]`
- `Result::ok`, `Result::map`, `Result::is_ok` — all `#[must_use]`
- `Iterator::map`, `Iterator::filter` — all `#[must_use]`

Not following this convention is inconsistent with Rust best practices.

### Clippy lint

Clippy has the `clippy::must_use_candidate` lint that flags functions returning values
without `#[must_use]`. Running `cargo clippy` does not currently warn (because the lint
is not enabled by default), but projects that enable stricter linting would get warnings.

## Proposed Fix

Add `#[must_use]` to the following method categories:

### Per-variant accessor methods

```rust
#[must_use]
pub fn $get(self) -> Option<$t> { ... }

#[must_use]
pub fn $is(&self) -> bool { ... }

#[must_use]
pub fn $map<U, F: FnOnce($t) -> U>(self, map: F) -> Or<$($map_t,)*> { ... }
```

### General methods on `Or<T...>` (inherent)

```rust
#[must_use]
pub fn into<T>(self) -> T where ... { ... }

#[must_use]
pub const fn as_ref(&self) -> Or<$(&$t,)*> { ... }

#[must_use]
pub fn as_mut(&mut self) -> Or<$(&mut $t,)*> { ... }

#[must_use]
pub fn as_deref(&self) -> Or<$(&$t::Target,)*> where ... { ... }

#[must_use]
pub fn as_deref_mut(&mut self) -> Or<$(&mut $t::Target,)*> where ... { ... }
```

### Uniform-type methods (`impl<T> Or<T, T, ...>`)

```rust
#[must_use]
pub fn into_inner(self) -> T { ... }

#[must_use]
pub fn map<U, F>(self, map: F) -> Or<U, U, ...> { ... }

#[must_use]
pub fn map_with<S, U, F>(self, state: S, map: F) -> Or<U, U, ...> { ... }
```

### Reference methods

```rust
#[must_use]
pub fn cloned(self) -> Or<$($t,)*> where ... { ... }

#[must_use]
pub fn copied(self) -> Or<$($t,)*> where ... { ... }
```

### Static methods

```rust
#[must_use]
pub fn from_tuple(tuple: ($($t,)*)) -> [Self; $count] { ... }

#[must_use]
pub fn try_into_tuple(array: [Self; $count]) -> Result<($($t,)*), [Self; $count]> { ... }
```

## Customizing the Warning Message

For some methods, a custom `#[must_use = "..."]` message improves the diagnostic:

```rust
#[must_use = "if you don't need the result, consider using `is_t0()` instead"]
pub fn t0(self) -> Option<T0> { ... }

#[must_use = "this returns a new Or value and does not modify the original"]
pub fn map_t0<U, F>(self, map: F) -> Or<...> { ... }
```

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`.
2. Add `#[must_use]` (with optional custom messages) to all methods listed above.
3. The additions occur inside the `or!(@main ...)` and `or!(@inner ...)` macro arms.
4. Run `cargo build --all-features` and confirm there are no new compiler errors.
5. Run `cargo test --all-features` to confirm no regressions.
6. Optionally run `cargo clippy --all-features -- -W clippy::must_use_candidate` to
   check for any remaining un-annotated candidates.
