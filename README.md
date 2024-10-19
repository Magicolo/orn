<div align="center"> <h1> orn 0.5.1 </h1> </div>

<p align="center">
    <em> 

A general implementation of the sum type. Meant to be a generic counterpart to tuples.
    </em>
</p>

<div align="right">
    <a href="https://github.com/Magicolo/orn/actions/workflows/test.yml"> <img src="https://github.com/Magicolo/orn/actions/workflows/test.yml/badge.svg"> </a>
    <a href="https://crates.io/crates/orn"> <img src="https://img.shields.io/crates/v/orn.svg"> </a>
</div>

---
### Cheat Sheet

```rust
use orn::*;

/// Often, the type of iterator is conditional to some input value. Typically,
/// to unify the return type, one would need to implement a custom iterator, but
/// here `orn` types are used instead.
pub fn unify_divergent(array_or_range: bool) -> impl Iterator<Item = u8> {
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

fn main() {}
```
