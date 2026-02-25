# Issue 12: `rayon::One` Is an Undocumented Public Type

## Summary

The `rayon` feature module exposes a public struct `One<T, $($t: ?Sized,)* const N: usize>`
and its implementations (`ProducerCallback`, `Producer`, `Folder`) as part of the
public API, but none of them have any documentation. `One` is an implementation detail
for the Rayon parallel iterator plumbing infrastructure and should either be documented
(if intentionally public) or hidden with `#[doc(hidden)]`.

## Location

**File:** `src/lib.rs`

Inside `or!(@main ...)`, within the `#[cfg(feature = "rayon")] pub mod rayon` block:

```rust
/// A parallel iterator that yields the items of an [`Or`] of parallel iterators.
#[derive(Clone, Copy, Debug)]
pub enum Iterator<$($t,)*> { $($t($t)),* }

// ← No doc comment:
pub struct One<T, $($t: ?Sized,)* const N: usize>(pub T, $(PhantomData<$t>,)*);
```

The `One` struct has no `///` doc comment, no `#[doc(hidden)]` attribute, and its field
`pub T` is public. Its associated `impl` blocks for `ProducerCallback`, `Producer`, and
`Folder` also have no documentation.

## Why It Is a Problem

### Confusing public API surface

Users exploring the crate's documentation via `docs.rs` will see `One` in the `rayon`
sub-modules but will have no idea what it is for. It is not something end users are
expected to interact with directly — it is scaffolding for the Rayon plumbing traits.

### `pub` field without documentation

The inner value `pub T` inside `One` can be accessed directly by users, which may
allow misuse of the Rayon plumbing internals.

### Breaks API stability guarantees

Because `One` is `pub`, any change to its signature (fields, const generics, trait
impls) is a semver-breaking change. If it were `#[doc(hidden)]` or `pub(crate)`, the
library would have more flexibility to change the implementation.

### Rayon plumbing traits are not stable

The traits `ProducerCallback`, `Producer`, and `Folder` from `rayon::iter::plumbing`
are considered unstable by the Rayon project itself — they can change between minor
versions. Implementing them publicly and without documentation creates an undocumented
dependency on these unstable internals.

## Proposed Fix

### Option A: Hide with `#[doc(hidden)]`

The simplest fix is to add `#[doc(hidden)]` to the `One` struct and all of its impl
blocks:

```rust
#[doc(hidden)]
pub struct One<T, $($t: ?Sized,)* const N: usize>(pub T, $(PhantomData<$t>,)*);
```

This keeps the type technically public (required for Rayon's trait machinery to work
across crate boundaries if needed), but removes it from the user-facing documentation.

### Option B: Make `pub(super)` or `pub(crate)`

If `One` is only used within the generated `rayon` sub-module (and not by external
crates), it can be changed to `pub(super)`:

```rust
pub(super) struct One<T, $($t: ?Sized,)* const N: usize>(pub(super) T, $(PhantomData<$t>,)*);
```

However, Rayon's plumbing traits (`ProducerCallback`, `Producer`) may require the
impl'ing types to be at least as visible as the trait implementations themselves. This
needs verification.

### Option C: Document `One` as a public implementation detail

If `One` must be public, add clear documentation explaining it is not for end-user use:

```rust
/// An internal helper type for Rayon parallel iterator plumbing.
///
/// This type is an implementation detail of the `Or` parallel iterator
/// infrastructure. It is not intended for direct use by library consumers.
/// Its API may change in future versions.
#[doc(hidden)]
pub struct One<T, $($t: ?Sized,)* const N: usize>(pub T, $(PhantomData<$t>,)*);
```

### Recommendation

**Option A** (add `#[doc(hidden)]`) is the minimum-impact fix. Combine with a comment
in the source explaining the purpose of `One`.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`, find the `rayon` sub-module inside `or!(@main ...)`.
2. Add `#[doc(hidden)]` to the `One` struct definition.
3. Verify that `cargo doc --all-features` no longer shows `One` in the public docs.
4. Run `cargo test --all-features` to ensure no regressions.
5. Optionally add a comment in the source explaining why `One` exists and how it
   interacts with Rayon's plumbing traits.
