# Issue 3: Missing `From<Tk>` Trait Implementations for Each Variant

## Summary

`Or<T0, T1, ..., TN>` types have no `From<Tk>` implementation for any individual
variant type. This means users cannot use the standard Rust `From`/`Into` conversion
machinery to construct an `Or` value from an individual variant.

## Location

**File:** `src/lib.rs`

The `or!(@main ...)` macro arm generates all `impl` blocks for each `OrN` type but
does not include any `From<Tk> for Or<T0, T1, ..., TN>` implementations.

## Why It Is a Problem

### Standard Rust conversion idiom

In Rust, the standard way to construct a wrapper type from an inner value is via the
`From` trait:

```rust
impl<T> From<T> for Wrapper<T> { ... }
```

This enables:
- `Wrapper::from(value)` — explicit construction
- `value.into()` — implicit coercion in typed contexts
- Use in `?`-operator chains (for `Result`/`Option` conversions)
- Acceptance in generic bounds: `fn f<T: Into<Wrapper<T>>>(v: T)`

### Current workaround is verbose and non-generic

Without `From` impls, users must explicitly name the variant constructor:

```rust
// Current — must write the variant name explicitly:
let or: Or2<u8, u16> = Or2::T0(42u8);
let or: Or2<u8, u16> = Or2::T1(100u16);
```

With `From` impls, the ergonomics improve dramatically in typed contexts:

```rust
// Desired — leverage type inference:
let or: Or2<u8, u16> = 42u8.into();
let or: Or2<u8, u16> = 100u16.into();
```

### Cannot be used in generic contexts

Code such as the following is currently impossible without `From` impls:

```rust
fn wrap_first<T0, T1>(value: T0) -> Or2<T0, T1>
where
    Or2<T0, T1>: From<T0>,  // This bound cannot be satisfied
{
    value.into()
}
```

### Inconsistency with comparable crates

The popular `either` crate (`Either<L, R>`) does provide `From<L>` and `From<R>`.

## Important Consideration: Orphan Rules and Overlapping Impls

When all type parameters are distinct, adding `From<T0>` through `From<TN>` for
`Or<T0, T1, ..., TN>` runs into a constraint: if two type parameters happen to be the
same type at a call site (e.g. `Or2<u8, u8>`), the compiler cannot resolve which `From`
impl to use, resulting in an ambiguity error.

This is an inherent limitation of the Rust trait system. There are two common approaches:

1. **Add the impls, document the limitation**: The `From` impls can be added unconditionally.
   They will fail to compile only in the case of duplicate type arguments. This is
   acceptable — the user can always fall back to the explicit constructor.

2. **Use a separate `IntoOr` trait**: Introduce a crate-specific conversion trait
   (e.g. `IntoOr<Or<T0, T1, ..., TN>, const N: usize>`) that avoids the ambiguity.

Approach 1 is simpler and consistent with the standard library's behaviour (e.g.
`From<T>` for `Option<T>` has the same limitation when `T = Option<U>`).

## Proposed Fix

Add the following generated impls inside the `or!(@main ...)` arm, within the
`@outer`/`@inner` sub-macro that already generates per-variant methods (`t0`, `is_t0`,
`map_t0`, etc.):

```rust
impl<$($ts),*> From<$t> for Or<$($ts,)*> {
    #[inline]
    fn from(value: $t) -> Self {
        Self::$t(value)
    }
}
```

One such impl is emitted per variant, so `Or2<T0, T1>` gains both `From<T0>` and
`From<T1>`.

### Corresponding `TryFrom` in a separate issue

The reverse direction — extracting a specific variant via `TryFrom` — is covered in
[Issue 4](04-missing-tryfrom-for-each-variant.md).

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and find the `or!(@inner ...)` macro arm (around line 758).
2. After the `impl<$($ts),*> Or<$($ts,)*>` block (which contains `$get`, `$is`, `$map`),
   add:
   ```rust
   impl<$($ts),*> From<$t> for Or<$($ts,)*> {
       #[inline]
       fn from(value: $t) -> Self {
           Self::$t(value)
       }
   }
   ```
3. Run `cargo test --all-features` and verify no regressions.
4. Add a test in `tests/or.rs`:
   ```rust
   #[test]
   fn from_t0() {
       let or: orn::Or2<u8, u16> = u8::from(42u8).into(); // via From<u8>
       // or equivalently:
       let or: orn::Or2<u8, u16> = orn::Or2::from(42u8);
       assert_eq!(or, orn::Or2::T0(42u8));
   }

   #[test]
   fn from_t1() {
       let or: orn::Or2<u8, u16> = orn::Or2::from(100u16);
       assert_eq!(or, orn::Or2::T1(100u16));
   }
   ```
5. Update the README/documentation to show the `From`-based construction style.
