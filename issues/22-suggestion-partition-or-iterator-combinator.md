# Issue 22: Suggestion — `hash_map` / `BTreeMap` Integration via `Entry`-Like API

## Summary

This is a feature suggestion for enabling `Or` to serve as a unified key/value type in
maps and sets. Additionally, a suggestion for a higher-level `fold_or` / `collect_or`
combinator that distributes items from an `Or` iterator into separate collections by
variant.

## Background

A common pattern enabled by sum types is distributing a stream of heterogeneous values
into separate typed collections:

```rust
let results: Vec<Or2<u8, String>> = vec![
    Or2::T0(1), Or2::T1("error".to_string()), Or2::T0(2), Or2::T1("oops".to_string()),
];

// Want to partition into (Vec<u8>, Vec<String>):
let (values, errors): (Vec<u8>, Vec<String>) = results.into_iter().partition_or();
```

Currently this requires writing a manual fold.

## Proposed `partition_or` / `unzip_or` for Iterator

Add an extension trait or inherent method on `Or` iterators:

```rust
pub trait PartitionOr<T0, T1, ..., TN>: Iterator<Item = Or<T0, T1, ..., TN>> {
    fn partition_or(self) -> (Vec<T0>, Vec<T1>, ..., Vec<TN>);
}
```

Example implementation for `Or2`:

```rust
impl<T0, T1, I: Iterator<Item = Or2<T0, T1>>> PartitionOr2<T0, T1> for I {
    fn partition_or(self) -> (Vec<T0>, Vec<T1>) {
        let mut t0 = Vec::new();
        let mut t1 = Vec::new();
        for item in self {
            match item {
                Or2::T0(v) => t0.push(v),
                Or2::T1(v) => t1.push(v),
            }
        }
        (t0, t1)
    }
}
```

## Relationship to `FromIterator`

A more composable approach uses `FromIterator`. If `Or2<A, B>` implements `Extend<A>`
and `Extend<B>`, and if a "dual collector" can consume an iterator of `Or2<A, B>` and
split it into two collections, then `unzip()` from the standard library could be used:

```rust
let results: Vec<Or2<u8, String>> = ...;
let (values, errors): (Vec<u8>, Vec<String>) = results
    .into_iter()
    .map(|item| match item {
        Or2::T0(v) => (Some(v), None),
        Or2::T1(e) => (None, Some(e)),
    })
    .unzip(); // This doesn't directly work...
```

A cleaner API would be:

```rust
let (values, errors): (Vec<u8>, Vec<String>) = 
    orn::iter::partition(results.into_iter());
```

## `collect_into_tuple` Extension

An alternative to `partition_or`:

```rust
impl<T0, T1> Or2<T0, T1> {
    pub fn collect_or<I, C0: Default + Extend<T0>, C1: Default + Extend<T1>>(
        iter: I,
    ) -> (C0, C1)
    where
        I: Iterator<Item = Self>,
    {
        iter.fold((C0::default(), C1::default()), |(mut c0, mut c1), item| {
            match item {
                Self::T0(v) => c0.extend(core::iter::once(v)),
                Self::T1(v) => c1.extend(core::iter::once(v)),
            }
            (c0, c1)
        })
    }
}
```

## Investigation Steps for the Implementing Agent

1. Design the `partition_or` / `collect_or` API.
2. Decide between:
   - Free function `orn::iter::partition(iter) -> (Vec<T0>, Vec<T1>)`
   - Extension trait `PartitionOr` on iterators
   - Method on `Or` types: `Or::collect_or(iter)`
3. Add the implementation using the `or!` macro (generating separate impls per `OrN`).
4. Add to the `#[cfg(feature = "iter")]` module.
5. Write tests:
   ```rust
   use orn::Or2;
   let items = vec![Or2::<u8, u16>::T0(1), Or2::T1(2), Or2::T0(3)];
   let (t0s, t1s): (Vec<u8>, Vec<u16>) = Or2::collect_or(items.into_iter());
   assert_eq!(t0s, vec![1, 3]);
   assert_eq!(t1s, vec![2]);
   ```
