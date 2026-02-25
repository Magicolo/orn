<div align="center"> <h1> orn 0.9.0 </h1> </div>

<p align="center">
    <i> 
    
A generic implementation of a sum type (or discriminated union). It provides `enum Or<T1, T2, ..., N>` types as a counterpart to tuples.
    </i>
</p>

<div align="right">
    <a href="https://github.com/Magicolo/orn/actions/workflows/test.yml"> <img src="https://github.com/Magicolo/orn/actions/workflows/test.yml/badge.svg"> </a>
    <a href="https://crates.io/crates/orn"> <img src="https://img.shields.io/crates/v/orn.svg"> </a>
</div>

---
### Features
- has `#![no_std]` and `#![forbid(unsafe_code)]`
- supports the applicable core traits
- supports `Widen` (inject into a larger `Or`) and `Narrow` (project onto a smaller `Or`)
- `features = ["iter"]` *(default)*: supports the `Into/Iterator` traits 
- `features = ["future"]` *(default)*: supports the `Into/Future` traits 
- `features = ["serde"]`: supports the `Serialize` and `Deserialize` traits
- `features = ["rayon"]`: supports the `ParallelIterator` family of traits
- `features = ["or16"]`: for up to `Or16`
- `features = ["or32"]`: for up to `Or32`

---
### Cheat Sheet

```rust
use orn::*;

/// Using the `tn` methods, items of an `Or` value can be retrieved.
pub fn retrieve_dynamically(input: Or3<u8, usize, [char; 1]>) {
    let _a: Option<u8> = input.t0();
    let _b: Option<usize> = input.t1();
    let _c: Option<[char; 1]> = input.t2();
}

/// Using the `At<I>` trait, items of an `Or` value can be retrieved
/// generically.
pub fn retrieve_statically(input: Or4<char, bool, isize, u32>) {
    let _a: Option<char> = At::<0>::at(input);
    let _b: Option<bool> = At::<1>::at(input);
    let _c: Option<isize> = At::<2>::at(input);
    let _d: Option<u32> = At::<3>::at(input);
}

/// Often, the type of iterator is conditional to some input value. Typically,
/// to unify the return type, one would need to implement a custom iterator, but
/// here `orn` types are used instead.
#[cfg(feature = "iter")]
pub fn unify_divergent_iterators(array_or_range: bool) -> impl Iterator<Item = u8> {
    if array_or_range {
        Or2::T0([1u8, 2u8])
    } else {
        Or2::T1(0u8..10u8)
    }
    .into_iter()
    // The item of the `Or` iterator is `Or<u8, u8>`. `Or::into` collapses an `Or` value into a
    // specified type.
    .map(Or2::into)
}

#[cfg(feature = "rayon")]
use rayon::prelude::*;
/// With `features = ["rayon"]`, `rayon`'s `ParallelIterator` family of traits
/// can be used similarly to the `Iterator` trait.
#[cfg(feature = "rayon")]
pub fn unify_divergent_parallel_iterators(
    array_or_range: bool,
) -> impl ParallelIterator<Item = u8> {
    if array_or_range {
        Or2::T0([1u8, 2u8])
    } else {
        Or2::T1(0u8..10u8)
    }
    .into_par_iter()
    // The item of the `Or` iterator is `Or<u8, u8>`. `Or::into` collapses an `Or` value into a
    // specified type.
    .map(Or2::into)
}

fn main() {}

```

---
### Widen and Narrow

`Widen` injects a smaller `Or` into a larger one by mapping each variant to the same position
in the target type. `Narrow` projects a larger `Or` onto a smaller one, returning `Err(self)`
when the active variant is not in the subset.

```rust
use orn::{Or1, Or2, Or3, Narrow, Widen};

// inner_a and inner_b return errors from the same prefix subset of the outer type.
fn inner_a() -> Result<(), Or2<String, std::io::Error>> {
    Ok(())
}

fn inner_b() -> Result<(), Or1<String>> {
    Ok(())
}

fn outer() -> Result<(), Or3<String, std::io::Error, std::num::ParseIntError>> {
    // Or2<String, IoError> widens to Or3 — T0→T0, T1→T1
    inner_a().map_err(Widen::widen)?;
    // Or1<String> widens to Or3 — T0→T0
    inner_b().map_err(Widen::widen)?;
    Ok(())
}

fn narrow_example(v: Or3<u8, u16, u32>) -> Result<Or2<u8, u16>, Or3<u8, u16, u32>> {
    v.narrow()
}

fn main() {
    assert!(outer().is_ok());

    // T0 and T1 are within the Or2 subset → Ok
    assert_eq!(narrow_example(Or3::T0(1u8)), Ok(Or2::T0(1u8)));
    assert_eq!(narrow_example(Or3::T1(2u16)), Ok(Or2::T1(2u16)));
    // T2 is outside the Or2 subset → Err
    assert_eq!(narrow_example(Or3::T2(99u32)), Err(Or3::T2(99u32)));
}

```
