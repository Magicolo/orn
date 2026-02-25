# Issue 6: Missing `Default` Implementation

## Summary

None of the `Or<T0, T1, ..., TN>` types implement the `Default` trait. A natural
implementation would return `Or::T0(T0::default())` when `T0: Default`. This omission
makes `Or` types harder to use in structs and generic contexts that require `Default`.

## Location

**File:** `src/lib.rs`

The `or!(@main ...)` macro arm does not generate any `Default` implementation.

## Why It Is a Problem

### Common expectation for container-like types

Users familiar with Rust expect enum-like types to optionally implement `Default` when
a sensible default exists. For `Or<T0, T1, ..., TN>`, a natural default is the first
variant `T0` wrapping `T0::default()`.

### Cannot be used in `#[derive(Default)]` structs

When a struct contains an `Or` field, `#[derive(Default)]` requires all fields to
implement `Default`. Without this impl, users must write the impl manually:

```rust
// Currently does not compile with #[derive(Default)]:
#[derive(Default)]
struct Config {
    mode: Or2<Mode1, Mode2>,  // error: Or2 does not implement Default
}
```

### Missing from common trait set

The `Or` types derive `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash` but
omit `Default`, which is typically included in this group for types where it makes sense.

## Proposed Fix

Add a `Default` implementation in the `or!(@main ...)` macro arm. The implementation
returns `Or::T0(T0::default())`:

```rust
impl<$first_t: Default, $($rest_t,)*> Default for Or<$first_t, $($rest_t,)*> {
    #[inline]
    fn default() -> Self {
        Self::T0($first_t::default())
    }
}
```

### Macro implementation note

The macro already tracks the first type parameter as the first entry in its iteration.
The `or!(@main ...)` arm receives all type parameter names (`$t`). The first one can be
extracted and used in the `Default` bound.

Looking at the macro structure, the `@main` arm receives a flat list:
```
[$($index: tt, $same_t: ident, $same_u: ident, $t: ident, $u: ident, $f: ident, $get: ident, $is: ident, $map: ident),*]
```

This can be split into `[first_index, first_same_t, ..., $t: ident, ...]` by using a
nested macro pattern that pops the first element.

Alternatively, a simpler approach is to emit the `Default` impl only in the `@inner`
sub-macro for index `0`:

In `or!(@inner ...)` when `$index == 0`, add:
```rust
impl<$($ts),*> Default for Or<$($ts,)*>
where
    $t: Default,
{
    #[inline]
    fn default() -> Self {
        Self::$t($t::default())
    }
}
```

Since `@inner` is called once per variant index (0, 1, 2, …), we need to guard this
impl so it only emits for index `0`. One way is to use a dedicated `@default` arm:

```rust
or!(@default $count, $alias, $module, $t, $get [$($ts),*]);
```

emitted at the end of `@main` after all `@inner` calls.

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs` and locate `or!(@main ...)`.
2. After all existing `impl` blocks inside `@main`, add a `Default` impl that uses the
   first type parameter (index 0). The macro receives `[$index, ...]` — identify the
   entry with `$index = 0` (it is always the first in the list).
3. A practical approach: at the end of `or!(@main ...)`, emit:
   ```rust
   or!(@default [$($index, $same_t, $same_u, $t, $u, $f, $get, $is, $map),*]);
   ```
   and define `@default` as:
   ```rust
   (@default [0, $same_t:ident, $same_u:ident, $t:ident, ... , $($rest:tt)*]) => {
       impl<$t: Default, ...> Default for Or<...> {
           fn default() -> Self { Self::$t($t::default()) }
       }
   };
   ```
   Adjust to fit the actual macro parameter layout.
4. Run `cargo test --all-features`.
5. Add a test:
   ```rust
   #[test]
   fn default_or2() {
       let or: orn::Or2<u8, u16> = Default::default();
       assert_eq!(or, orn::Or2::T0(0u8));
   }
   ```
