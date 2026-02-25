# Issue 8: Missing `fold`, `for_each`, and `nth` Optimizations in `iter::Iterator`

## Summary

The `iter::Iterator<T...>` type (the `IntoIterator` adapter generated for `Or`) only
overrides `next()` and `next_back()`. It does not override `fold`, `for_each`, `nth`,
or `nth_back`. This misses significant performance opportunities because many inner
iterator types (e.g. `std::vec::IntoIter`, `std::ops::Range`, slice iterators) have
highly optimized implementations of these methods that the `Or` wrapper currently
bypasses.

## Location

**File:** `src/lib.rs`

Inside `or!(@main ...)`, within the `#[cfg(feature = "iter")] pub mod iter` block:

```rust
impl<$($t: core::iter::Iterator),*> core::iter::Iterator for Iterator<$($t,)*> {
    type Item = Or<$($t::Item,)*>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self { ... }
    }
    // ← No fold, for_each, nth, size_hint overrides
}

impl<$($t: DoubleEndedIterator),*> DoubleEndedIterator for Iterator<$($t,)*> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self { ... }
    }
    // ← No nth_back override
}
```

## Why It Is a Problem

### `fold` — the core of iterator chains

`fold` is the backbone of many iterator operations: `sum`, `product`, `collect`, `max`,
`min`, `any`, `all`, etc. all delegate to `fold` when the inner iterator overrides it.

For example, `std::vec::IntoIter::fold` uses a specialized loop that the compiler can
auto-vectorize. When `Or::iter::Iterator` doesn't override `fold`, these optimizations
are lost: the chain falls back to calling `next()` repeatedly.

Missing override:
```rust
fn fold<B, F>(self, init: B, mut f: F) -> B
where
    F: FnMut(B, Self::Item) -> B,
{
    match self {
        $(Self::$t(item) => item.fold(init, |acc, x| f(acc, Or::$t(x))),)*
    }
}
```

### `for_each` — side-effecting consumption

`for_each` is commonly optimized similarly to `fold`. Without an override, it falls back
to the default `for_each` which calls `fold`. Even if `fold` is fixed, adding `for_each`
directly is beneficial:

```rust
fn for_each<F>(self, mut f: F)
where
    F: FnMut(Self::Item),
{
    match self {
        $(Self::$t(item) => item.for_each(|x| f(Or::$t(x))),)*
    }
}
```

### `nth` — random access into iterators

Slice iterators and other exact-size iterators optimize `nth` to advance without
allocating intermediate values. Without overriding `nth`, every `take(n).last()` or
`skip(n).next()` call must call `next()` n+1 times:

```rust
fn nth(&mut self, n: usize) -> Option<Self::Item> {
    match self {
        $(Self::$t(item) => item.nth(n).map(Or::$t),)*
    }
}
```

### `nth_back` for `DoubleEndedIterator`

Same rationale as `nth` but for the reverse direction:

```rust
fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    match self {
        $(Self::$t(item) => item.nth_back(n).map(Or::$t),)*
    }
}
```

### Quantified impact

Consider iterating over `Or2::<Vec<i32>, Vec<i32>>::T0(large_vec).into_iter().sum::<i32>()`.
With `fold` overridden, this delegates to `vec::IntoIter::fold` which the compiler can
SIMD-vectorize. Without the override, each element goes through `next()` one at a time.

## Proposed Fix

Add the following method overrides to the `core::iter::Iterator` impl for
`iter::Iterator<T...>` in `or!(@main ...)`:

```rust
#[inline]
fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
        $(Self::$t(item) => item.size_hint(),)*
    }
}

#[inline]
fn count(self) -> usize {
    match self {
        $(Self::$t(item) => item.count(),)*
    }
}

#[inline]
fn nth(&mut self, n: usize) -> Option<Self::Item> {
    match self {
        $(Self::$t(item) => item.nth(n).map(Or::$t),)*
    }
}

#[inline]
fn fold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, mut f: F) -> B {
    match self {
        $(Self::$t(item) => item.fold(init, |acc, x| f(acc, Or::$t(x))),)*
    }
}

#[inline]
fn for_each<F: FnMut(Self::Item)>(self, mut f: F) {
    match self {
        $(Self::$t(item) => item.for_each(|x| f(Or::$t(x))),)*
    }
}
```

And to `DoubleEndedIterator`:

```rust
#[inline]
fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    match self {
        $(Self::$t(item) => item.nth_back(n).map(Or::$t),)*
    }
}

#[inline]
fn rfold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, mut f: F) -> B {
    match self {
        $(Self::$t(item) => item.rfold(init, |acc, x| f(acc, Or::$t(x))),)*
    }
}

#[inline]
fn rfore_each<F: FnMut(Self::Item)>(self, mut f: F) {
    match self {
        $(Self::$t(item) => item.rfore_each(|x| f(Or::$t(x))),)*  // rfore_each does not exist, use rfold
    }
}
```

Note: `rfore_each` does not exist in the standard library; only `rfold` is needed.

Note also that `size_hint` is already covered as a separate correctness fix in
[Issue 1](01-exactsizeiterator-size-hint-contract-violation.md).

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`, locate `or!(@main ...)` → `pub mod iter` → `impl core::iter::Iterator`.
2. Add `size_hint`, `count`, `nth`, `fold`, `for_each` overrides as shown above.
3. In `DoubleEndedIterator`, add `nth_back` and `rfold` overrides.
4. Run `cargo test --all-features`.
5. Consider adding a benchmark (using `criterion`) that shows the performance
   improvement for large vectors; this can be left for a follow-up PR.
