# Issue 18: Missing `widen` / `narrow` Conversions Between `OrN` Types

## Summary

There is no way to convert between differently-sized `Or` types. For example, converting
an `Or2<A, B>` into an `Or3<A, B, C>` ("widening" — injecting into a larger sum type)
or extracting an `Or2<A, B>` from an `Or3<A, B, C>` if the third variant is not present
("narrowing"). These conversions are fundamental operations for sum types and are
completely absent from the library.

## Location

**File:** `src/lib.rs`

No widen/narrow operations exist anywhere in the `or!` macro.

## Why It Is a Problem

### A common real-world pattern

When composing functions that return different subsets of errors:

```rust
fn inner_a() -> Result<(), Or2<ParseErr, IoErr>> { ... }
fn inner_b() -> Result<(), Or2<IoErr, NetworkErr>> { ... }

fn outer() -> Result<(), Or3<ParseErr, IoErr, NetworkErr>> {
    inner_a().map_err(/* needs to widen Or2<ParseErr, IoErr> to Or3 */)?;
    inner_b().map_err(/* needs to widen Or2<IoErr, NetworkErr> to Or3 */)?;
    Ok(())
}
```

Currently there is no built-in way to perform this widening. Users must manually match
and re-wrap every variant.

### Sum types require injection/projection

The categorical dual of product types (tuples) is sum types (or-types). Just as tuples
support projection (extracting a field), sum types support:

- **Injection (widen)**: injecting `Or2<A, B>` into `Or3<A, B, C>` where the new
  variant `C` cannot occur
- **Projection (narrow)**: attempting to project `Or3<A, B, C>` onto `Or2<A, B>` (fails
  if the `C` variant is active)

### Comparative note

The `either` crate's `Either<L, R>` is limited to 2 variants, but even it provides
`map_left`/`map_right` to transform variants. Many Haskell/Scala implementations of
union types provide injection and projection operators.

## Proposed Fix

### Widen: inject a smaller `Or` into a larger one

Add `widen` methods or a `Widen<Target>` trait:

```rust
pub trait Widen<Target> {
    fn widen(self) -> Target;
}
```

For example, if `Or2<A, B>` → `Or3<A, B, C>`:

```rust
impl<A, B, C> Widen<Or3<A, B, C>> for Or2<A, B> {
    fn widen(self) -> Or3<A, B, C> {
        match self {
            Or2::T0(a) => Or3::T0(a),
            Or2::T1(b) => Or3::T1(b),
        }
    }
}
```

This requires many impls (all sub-sequences of type lists are valid widening targets),
but the macro can generate them.

A simpler alternative is `map_tN` chaining, but this requires N-1 transformations.

### Narrow: project a larger `Or` onto a smaller one

Add `narrow` / `try_narrow` method:

```rust
pub trait Narrow<Subset>: Sized {
    fn narrow(self) -> Result<Subset, Self>;
}
```

For `Or3<A, B, C>` → `Or2<A, B>` (drops `C`):

```rust
impl<A, B, C> Narrow<Or2<A, B>> for Or3<A, B, C> {
    fn narrow(self) -> Result<Or2<A, B>, Self> {
        match self {
            Or3::T0(a) => Ok(Or2::T0(a)),
            Or3::T1(b) => Ok(Or2::T1(b)),
            other => Err(other),
        }
    }
}
```

### Simpler alternative: per-variant `inject` methods

A more targeted approach: for each `OrN`, generate `inject_into_or_m` methods for all
M > N that include the same type parameters as subsets:

```rust
impl<T0, T1, T2> Or2<T0, T1> {
    pub fn inject_t0_t1_into_or3_t0_t1(self) -> Or3<T0, T1, T2> {
        match self {
            Self::T0(x) => Or3::T0(x),
            Self::T1(x) => Or3::T1(x),
        }
    }
}
```

But this combinatorial explosion makes the approach impractical without macro support.

### Recommendation

For an initial implementation, add `Widen` and `Narrow` as traits with a focused set
of generated impls (e.g., `Or1` → `Or2`, `Or2` → `Or3`, etc. for the "next size up"
case). The full combinatorial set can be added in a follow-up.

## Investigation Steps for the Implementing Agent

1. Design the `Widen<Target>` and `Narrow<Subset>` traits.
2. Add them to `src/lib.rs`.
3. Generate impls using the `or!` macro for the "next size up" case (or use a separate
   macro that generates all sub-sequence pairs).
4. Write tests demonstrating the conversions.
5. Document with examples in `README.md`.

Note: This is a significant feature addition and may warrant its own crate version bump.
