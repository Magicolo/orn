# Issue 14: Inherent `into()` Is Not a Proper `Into<T>` Trait Implementation

## Summary

`Or<T0, T1, ..., TN>` provides an **inherent method** `fn into<T>(self) -> T` that
converts an `Or` value to `T` when all variant types implement `Into<T>`. However,
this is not a `From<Or<...>>` or `Into<T>` **trait** implementation. As a result, `Or`
values cannot be used with trait-bound APIs that require `Into<T>` or `From<Or<...>>`,
and the method is easy to confuse with the standard `Into::into` from the Rust prelude.

## Location

**File:** `src/lib.rs`

Inside `or!(@main ...)`:

```rust
impl<$($t,)*> Or<$($t,)*> {
    /// Converts the [`Or`] into a single value of type `T`.
    ///
    /// This method is available when all types inside the [`Or`] can be converted into `T`.
    #[inline]
    pub fn into<T>(self) -> T where $($t: Into<T>),* {
        match self {
            $(Self::$t(item) => item.into(),)*
        }
    }
    // ...
}
```

## Why It Is a Problem

### Cannot be used in generic `Into<T>` bounds

The following code does **not** compile, even though `Or2<u8, u16>` has an `into()`
method that converts to `u32`:

```rust
fn convert<S: Into<u32>>(s: S) -> u32 { s.into() }

let or: Or2<u8, u16> = Or2::T0(42u8);
let n = convert(or);  // ERROR: Or2<u8, u16> does not implement Into<u32>
```

### Naming conflict with `Into::into`

The inherent `into<T>(self)` method has the same name as `Into::into`. When a user
writes `or_value.into()`, Rust resolves this to the inherent method (because inherent
methods have priority). The result is correct, but if the user is not aware that an
inherent method exists (rather than a trait impl), they may be confused about why type
inference works differently from a standard `Into` impl.

### Cannot use `or_value.into()` in `From` / coercion contexts

In some contexts, Rust uses `From`/`Into` implicitly (e.g. `?` operator, certain
coercion rules). An inherent method never participates in these implicit conversions.

### Inconsistency with the rest of the Rust ecosystem

Virtually all conversion types in Rust implement `From`/`Into` as a trait, not as an
inherent method. The inherent `into()` is a non-standard pattern that breaks the
principle of least surprise.

## Proposed Fix

### Add a blanket `Into<T>` impl (or equivalently `From<Or<...>>` for `T`)

However, implementing `From<Or<T0, T1, ..., TN>>` for an arbitrary `T` is not possible
without specialization, because there may be conflicting impls (e.g. `T = Or<T0, T1>`
itself would conflict with `From<X> for X`).

Instead, consider a newtype approach: provide a trait `Collapse<T>` that the user can
implement, or use a free-standing function.

### Alternative: keep the inherent method, rename it to avoid confusion

Rename the inherent `into` method to something unambiguous, such as `collapse` or
`into_unified`:

```rust
pub fn collapse<T>(self) -> T where $($t: Into<T>),* {
    match self {
        $(Self::$t(item) => item.into(),)*
    }
}
```

This removes the naming confusion while keeping the functionality. The standard
`.into()` (trait method) would then fail to compile, making the limitation explicit
and clear to users.

### Implement `Into<T>` via a sealed trait

A more advanced approach: introduce a `Unify<T>` sealed trait that `Or` types implement
when all variants implement `Into<T>`. This allows trait-bound usage:

```rust
pub trait Unify<T>: Sized {
    fn unify(self) -> T;
}
impl<T, $($t: Into<T>),*> Unify<T> for Or<$($t,)*> {
    fn unify(self) -> T { ... }
}
```

## Semver Note

Renaming `into` to `collapse` is a **breaking change**. Keeping `into` and adding
`collapse` as an alias (deprecated or not) is a non-breaking transition path.

## Investigation Steps for the Implementing Agent

1. Decide on the preferred approach (rename vs. alias vs. trait).
2. If renaming: change `into<T>` to `collapse<T>` in `or!(@main ...)` and update all
   doctests and `README.md`.
3. Run `cargo test --all-features` to confirm no regressions.
4. Bump the crate version for the breaking change.
5. Add a test demonstrating the limitation (and the workaround):
   ```rust
   // After renaming to collapse:
   let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
   let n: u32 = or.collapse();
   assert_eq!(n, 42u32);
   ```
