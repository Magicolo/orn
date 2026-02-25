# Issue 19: Derived `PartialOrd` / `Ord` Ordering Is Surprising and Undocumented

## Summary

All `Or<T0, T1, ..., TN>` types `#[derive(PartialOrd, Ord)]`, which produces a
**lexicographic** ordering: first by variant discriminant (variant index), then by the
inner value. This means `Or2::T0(999) < Or2::T1(0)` for any inner types, regardless of
the inner values. This behaviour is not documented anywhere and can be surprising to
users who expect value-based ordering.

## Location

**File:** `src/lib.rs`

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or<$($t,)*> { ... }
```

## Why It Is a Problem

### The ordering is based on variant index, not inner value

Rust's derived `Ord` for enums orders by variant declaration order first, then by
inner value. For `Or2<u8, u8>`:

```rust
Or2::T0(255) < Or2::T1(0)   // true — T0 always < T1
Or2::T0(0) < Or2::T0(1)     // true — same variant, compare by value
```

This is rarely what users want when they sort a collection of heterogeneous `Or` values.
A user sorting `Vec<Or2<u8, &str>>` by the numeric value would be surprised to find
`T0(1)` sorted before `T1("aardvark")` not because 1 < "aardvark" (which is nonsensical)
but because `T0 < T1` in variant order.

### Not useful for typical use cases

The derived `Ord` is only meaningful when:
1. The user wants to group/sort by variant type (variant-index-first sort), OR
2. All variants hold the same type and inner-value comparison makes sense.

For case 2, `into_inner()` gives access to the comparable inner value anyway.
For case 1, the user probably wanted `sort_by_variant()` which already exists.

### Inconsistency with the existing `sort_by_variant` API

The library already provides `Or::sort_by_variant(slice)` for sorting by variant. This
suggests the library knows that "sort by variant" is a specific operation. But the
derived `Ord` does exactly "sort by variant then by value" — which overlaps with
`sort_by_variant` but adds implicit inner-value ordering that users may not want.

### No documentation

There is no documentation explaining what the derived `Ord` means for `Or` types. Users
who rely on `or.cmp(&other)` or use `Or` in a `BTreeSet` may be surprised.

## Proposed Fix

### Option A: Remove the `Ord` / `PartialOrd` derives

Remove `PartialOrd` and `Ord` from the derive list. Users who need ordering can implement
it manually or use `sort_by_variant` + inner-value comparison.

This is a **breaking change** since `Ord` is currently in the public API surface.

### Option B: Keep derives, add clear documentation

Keep `PartialOrd` and `Ord` but add documentation at the type level explaining the
ordering semantics:

```rust
/// An `enum` of `N` variants.
///
/// # Ordering
///
/// This type derives [`PartialOrd`] and [`Ord`]. The ordering is
/// **by variant index first, then by inner value**. This means:
/// - A value of variant `T0` is always less than a value of variant `T1`,
///   regardless of the inner values.
/// - Two values of the same variant are ordered by their inner values.
///
/// Use [`Or::sort_by_variant`] if you only need to group by variant type.
```

### Option C: Provide a `variant_cmp` / `value_cmp` pair

Provide explicit comparison helpers:

```rust
/// Compares by variant index only (ignoring inner values).
pub fn variant_ord(&self, other: &Self) -> Ordering { ... }

/// Compares by inner value when both are the same variant; returns `None` otherwise.
pub fn value_cmp(&self, other: &Self) -> Option<Ordering> where $($t: Ord),* { ... }
```

### Recommendation

**Option B** is the minimum-impact fix: document the ordering behaviour. **Option A**
is appropriate if the derived ordering is deemed more harmful than helpful.

## Investigation Steps for the Implementing Agent

1. Decide which option to implement (discuss with maintainer).
2. If Option B: add documentation to the `Or` enum in `or!(@main ...)` explaining the
   ordering semantics.
3. If Option A: remove `PartialOrd, Ord` from the `#[derive(...)]` and bump semver.
4. Run `cargo test --all-features`.
5. Update the `README.md` if the change is breaking.
