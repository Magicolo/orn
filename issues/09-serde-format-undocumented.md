# Issue 9: Serde Serialization Format Is Undocumented

## Summary

The `serde` feature flag enables `#[derive(serde::Serialize, serde::Deserialize)]` on
all `Or` enum types, but the resulting serialization format is nowhere documented. Users
enabling `features = ["serde"]` have no way to know what wire format to expect without
running the code themselves, making interoperability fragile and surprising.

## Location

**File:** `src/lib.rs`

Inside `or!(@main ...)`:

```rust
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Or<$($t,)*> { ... }
```

**File:** `README.md` / `Cargo.toml` — the `serde` feature is listed but its behaviour
is not explained.

## Why It Is a Problem

### The format is non-obvious

Serde's default representation for Rust enums with tuple variants is the
**externally tagged** format. For example:

```rust
Or2::<u8, &str>::T0(42)
```

serializes (in JSON) to:

```json
{"T0": 42}
```

and:

```rust
Or2::<u8, &str>::T1("hello")
```

serializes to:

```json
{"T1": "hello"}
```

This is not documented anywhere in the crate. A user who wants to interchange `Or` values
with an API expecting a specific format (e.g. a plain integer `42` rather than `{"T0":42}`)
will be surprised and have no guidance.

### No information on customizing the format

Serde's `#[serde(...)]` attributes can change the representation (e.g. `untagged`,
`adjacently tagged`, etc.), but since the derive is applied via a macro, users cannot add
custom attributes to the generated types.

### Versioning / stability

If the variant names change (e.g. if a future version of the library renames `T0`→`V0`),
all serialized data becomes invalid. There is no documentation warning about this.

### Potential interoperability issues with `untagged` deserialization

If all variant types are the same (e.g. `Or2<u8, u8>`), serde may have trouble
distinguishing variants during deserialization with the default tagged format, since both
have identical JSON representations after the outer `{"Tk": ...}` wrapper — actually
in that case they are distinguishable by tag. But `untagged` would fail. This corner
case is undocumented.

## Proposed Fix

### 1. Document the serialization format

Add a documentation section to the crate-level docs (either `README.md` or via
`#[doc = ...]` in `lib.rs`) explaining:

- The default serde representation for `Or` enum types (externally tagged)
- Example JSON/TOML/etc. for at least `Or2` and `Or3`
- The limitation that variant names (`T0`, `T1`, …) appear in the serialized form
- A warning that variant names are part of the serialized format and may change across
  major versions

### 2. Example in README

Add a section to `README.md` such as:

```markdown
### Serde

When `features = ["serde"]` is enabled, `Or` types implement `Serialize` and
`Deserialize` using serde's **externally tagged** enum representation.

For example, `Or2::<u8, &str>::T0(42)` serializes to JSON as:
```json
{"T0": 42}
```

and `Or2::<u8, &str>::T1("hello")` serializes to:
```json
{"T1": "hello"}
```

The variant tag names (`T0`, `T1`, …) are part of the stable serialization format.
```

### 3. Add a doc-test in `src/lib.rs`

```rust
/// # Serde support
///
/// ```
/// #[cfg(feature = "serde")]
/// {
///     use orn::Or2;
///     use serde_json;
///     let or: Or2<u8, &str> = Or2::T0(42);
///     let json = serde_json::to_string(&or).unwrap();
///     assert_eq!(json, r#"{"T0":42}"#);
///     let back: Or2<u8, &str> = serde_json::from_str(&json).unwrap();
///     assert_eq!(back, Or2::T0(42));
/// }
/// ```
```

Note: adding `serde_json` as a `dev-dependency` would be required for the doc-test.

## Investigation Steps for the Implementing Agent

1. Enable `features = ["serde"]` and add `serde_json` as a `dev-dependency`:
   ```toml
   [dev-dependencies]
   serde_json = "1"
   ```
2. Write a small experiment to confirm the actual serialization format:
   ```rust
   use orn::Or2;
   let json = serde_json::to_string(&Or2::<u8, &str>::T0(42)).unwrap();
   println!("{}", json);  // Expected: {"T0":42}
   ```
3. Write documentation as described above.
4. Verify round-trip deserialization with tests.
5. Remove `dev-dependency` if doc-tests are gated behind `cfg(feature = "serde")` and
   do not require `serde_json` to compile.
