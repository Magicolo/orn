# Issue 15: `Or0` Is Missing Standard Trait Derives

## Summary

The `or0::Or` type (which represents an uninhabited / never type) is declared as a bare
`pub enum Or {}` with only two manual `Count` implementations. It is missing derives for
`Clone`, `Copy`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, and also
the trait implementations that all the other `Or` types have: `Is`, `Count`, and (with
features enabled) `IntoIterator`, `Future`, etc.

## Location

**File:** `src/lib.rs`, `or0` module (lines 69–83):

```rust
pub mod or0 {
    use super::*;

    /// A union of 0 types. This type is uninhabited, meaning it cannot be
    /// instantiated.
    pub enum Or {}

    impl Count for () {
        const COUNT: usize = 0;
    }

    impl Count for Or {
        const COUNT: usize = 0;
    }
}
```

## Why It Is a Problem

### Derives work trivially for uninhabited types

For an enum with no variants, all of the standard derive macros compile and produce
correct (vacuously true) implementations:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or {}
```

Rust requires matching arms for all variants in `match`, but with an empty enum there
are no variants, so `match` expressions are exhaustive with no arms. All trait methods
are vacuously correct (they can never be called).

### Inconsistency with `Or1` through `Or8`

All other `OrN` types derive these traits. `Or0` is the only exception, creating an
inconsistency that makes generic code difficult:

```rust
fn needs_debug<T: std::fmt::Debug>(v: T) { ... }

needs_debug(Or2::<u8, u16>::T0(1));  // works
needs_debug(Or0::UNINHABITED);        // error: Or0 does not implement Debug
// (Or0 can never be instantiated, so this could never be called — but the bound fails)
```

### Missing `Is` implementation

The `Is` trait is implemented for all `OrN` (N >= 1) types but not for `Or0`. For an
uninhabited type, `is(index)` should always return `false` (or be unreachable):

```rust
impl Is for Or {
    fn is(&self, _index: usize) -> bool {
        match *self {} // exhaustive, vacuously false
    }
}
```

### Missing `IntoIterator` for `#[cfg(feature = "iter")]`

All other `OrN` types implement `IntoIterator`. `Or0` doesn't. Since it can never be
instantiated, an iterator over `Or0` could be implemented as the empty iterator.

### Missing `Future` for `#[cfg(feature = "future")]`

Similarly, all other `OrN` types implement `IntoFuture`. `Or0` doesn't.

## Proposed Fix

### Step 1: Add standard derives

```rust
pub mod or0 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Or {}

    impl Count for () { const COUNT: usize = 0; }
    impl Count for Or { const COUNT: usize = 0; }
}
```

### Step 2: Add `Is` implementation

```rust
impl Is for Or {
    fn is(&self, _index: usize) -> bool {
        match *self {}
    }
}
```

### Step 3: Document that `Or0` is the "never" / "void" type

Add documentation clarifying the relationship between `Or0` and `!`:

```rust
/// A union of 0 types.
///
/// This type is **uninhabited**: it has no variants and cannot be instantiated.
/// It is analogous to Rust's [never type](https://doc.rust-lang.org/std/primitive.never.html)
/// `!` (currently unstable) and to the mathematical concept of the empty sum type.
///
/// A function returning `Or0` can never return normally; a value of type `Or0`
/// can never exist at runtime.
pub enum Or {}
```

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`, find the `or0` module (around line 69).
2. Add `#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]` to the
   `Or` enum.
3. Add `impl Is for Or` with `match *self {}`.
4. Consider adding `impl IntoIterator for Or` that returns an empty iterator (e.g.
   `core::iter::empty()`).
5. Improve the documentation of `Or0` to explain its role as the "never" type.
6. Run `cargo test --all-features`.
7. Add a compile-time test showing the derives work:
   ```rust
   fn assert_debug<T: std::fmt::Debug>() {}
   fn assert_eq<T: Eq>() {}
   // In a test or in lib.rs:
   // assert_debug::<orn::Or0>();  // must compile
   ```
