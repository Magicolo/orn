# Issue 23: Suggestion — Proc-Macro Crate for Ergonomic `Or` Construction

## Summary

This is a feature suggestion: provide a companion proc-macro crate (e.g. `orn-macros`)
that offers a derive macro `#[derive(IntoOr)]` and/or attribute macros to make working
with `Or` types more ergonomic, particularly for error handling and function return types.

## Background and Motivation

Currently, constructing `Or` values requires explicitly naming the variant:

```rust
fn process() -> Result<Data, Or3<ParseError, IoError, NetworkError>> {
    let data = parse(input).map_err(Or3::T0)?;
    let file = read_file(path).map_err(Or3::T1)?;
    let response = fetch(url).map_err(Or3::T2)?;
    Ok(combine(data, file, response))
}
```

Naming `T0`, `T1`, `T2` is fragile: if the error types in `Or3<...>` change order, all
the `.map_err(Or3::Tk)` calls must be updated. This is a significant maintenance burden.

## Proposed Macro: `#[into_or]` Attribute

An attribute macro on the function signature could automatically infer which variant to
use based on type:

```rust
#[into_or]
fn process() -> Result<Data, Or3<ParseError, IoError, NetworkError>> {
    let data = parse(input)?;       // auto-wrapped as T0 (type inference)
    let file = read_file(path)?;    // auto-wrapped as T1 (type inference)
    let response = fetch(url)?;     // auto-wrapped as T2 (type inference)
    Ok(combine(data, file, response))
}
```

The macro would insert the appropriate `map_err(OrN::Tk)` based on the error types.

## Proposed Macro: `or!` Constructor Macro

A simple `or!` constructor macro that selects the correct variant by type:

```rust
// Selects T0 because 42u8 is u8 and u8 corresponds to the first variant
let or: Or2<u8, u16> = or!(42u8);

// Selects T1 because 100u16 is u16
let or: Or2<u8, u16> = or!(100u16);
```

## Proposed Crate: `orn-macros`

Create a companion crate `orn-macros` that provides:

1. `#[derive(IntoOr)]` on enums — converts a user-defined enum into an equivalent `Or`
   type
2. `into_or!` procedural macro for ergonomic `Or` construction
3. `#[into_or]` function attribute for automatic error wrapping

## Relationship to Existing Code

The `orn` crate is `#![no_std]` and `#![forbid(unsafe_code)]`. A proc-macro companion
crate would be a separate library (proc-macros require `proc-macro = true` in
`Cargo.toml` and run at compile time on the host machine). The main `orn` crate would
not depend on `orn-macros`; users would opt in.

## Investigation Steps for the Implementing Agent

1. Create a new crate `orn-macros` in the workspace (or as a separate repository).
2. Implement `#[derive(IntoOr)]` that generates `From` impls between a user enum and
   the corresponding `Or` type.
3. Publish `orn-macros` as a companion crate on crates.io.
4. Document the relationship in the `orn` README.

Note: This is a long-term feature suggestion requiring a new crate. It is out of scope
for a single PR but is recorded here for future consideration.
