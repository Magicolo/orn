# Issue 17: `At<I>` Returns Inconsistent Types for Tuples vs `Or` Types

## Summary

The `At<I>` trait has an associated type `Item` that is `Tk` (the raw type) when used
with tuples, but `Option<Tk>` when used with `Or` types. This means the same trait
name conveys fundamentally different semantics depending on the receiver, making it
impossible to write truly generic code over both tuples and `Or` types using `At<I>`.

## Location

**File:** `src/lib.rs`, inside `or!(@inner ...)`:

```rust
// For tuples: returns T directly (infallible)
impl<$($ts),*> At<$index> for ($($ts,)*) {
    type Item = $t;   // ← just T
    fn at(self) -> Self::Item { self.$index }
}

// For Or: returns Option<T> (fallible)
impl<$($ts),*> At<$index> for Or<$($ts,)*> {
    type Item = Option<$t>;   // ← Option<T>
    fn at(self) -> Self::Item {
        match self { Self::$t(item) => Some(item), _ => None }
    }
}
```

## Why It Is a Problem

### Cannot write generic code over both

Any code parameterized over both tuples and `Or` types via `At<I>` will be confused by
the different `Item` types:

```rust
fn get_first<C: At<0>>(container: C) -> C::Item {
    container.at()
}

// For tuple: Item = u8
let x: u8 = get_first((1u8, 2u16));

// For Or: Item = Option<u8>
let y: Option<u8> = get_first(Or2::<u8, u16>::T0(1u8));
```

While this is valid Rust (the `Item` associated type differs), writing a single
function that works uniformly over both and returns the inner value requires either a
separate trait bound or post-processing.

### Conceptual mismatch

`At<I>` with the name "accessing a type at a specific index" implies:
- For tuples: infallible, because the element always exists
- For `Or`: fallible, because the `Or` might hold a different variant

This distinction is correct but needs to be better documented and possibly addressed
with a better trait design.

### Cannot use `At<I>` in `where` bounds uniformly

A function that says "I accept anything that has a `u8` at index 0" cannot be written
in a uniform way:

```rust
// This works for tuples but not Or (wrong Item type):
fn needs_u8_at_0<T: At<0, Item = u8>>(value: T) -> u8 {
    value.at()
}

// This works for Or but not tuples (wrong Item type):
fn needs_optional_u8_at_0<T: At<0, Item = Option<u8>>>(value: T) -> Option<u8> {
    value.at()
}
```

## Proposed Fixes

### Option A: Split into two traits

Introduce `Get<I>` for infallible access (tuples) and keep `At<I>` for fallible access
(`Or` types), or vice versa:

```rust
/// Infallible indexed access (for tuples).
pub trait Get<const I: usize> {
    type Item;
    fn get(self) -> Self::Item;
}

/// Fallible indexed access (for Or types).
pub trait At<const I: usize> {
    type Item;
    fn at(self) -> Option<Self::Item>;
}
```

This is a **breaking change**.

### Option B: Document the distinction clearly

Keep the current design but add clear documentation explaining that `Item` for tuples
is `T` and for `Or` is `Option<T>`, and why this is intentional. Provide guidance on
writing generic code that handles both.

### Option C: Use a `GetOrAt` design with associated GAT

Use GATs (stabilized in Rust 1.65) to parameterize the return type:

```rust
pub trait At<const I: usize> {
    type Item<'a> where Self: 'a;
    fn at(self) -> Self::Item<'_>;
}
```

But this would require complex lifetime handling.

### Recommendation

**Option B** is the minimum-impact fix for now. Clear documentation should be added to
the `At<I>` trait and its implementations. If the crate is willing to make breaking
changes, **Option A** provides a cleaner API.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`, find the `At<I>` trait definition and its documentation
   (around line 7–16).
2. Add explicit documentation noting the different `Item` types for tuples vs `Or`.
3. Update the examples in `README.md` and the cheat sheet `examples/cheat.rs` to show
   both usage patterns.
4. Consider adding a longer example showing how to write generic code over both:
   ```rust
   // Generic over Or types (where Item = Option<T>)
   fn get_or<C, T>(container: C) -> Option<T>
   where
       C: At<0, Item = Option<T>>,
   {
       container.at()
   }
   ```
5. Run `cargo test --all-features`.
