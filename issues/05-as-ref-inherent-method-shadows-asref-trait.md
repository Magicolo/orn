# Issue 5: Inherent `as_ref` Method Shadows the `AsRef<T>` Trait Implementation

## Summary

`Or<T0, T1, ..., TN>` has both an **inherent** method `as_ref(&self) -> Or<&T0, &T1, ..., &TN>`
and a **trait** implementation `AsRef<T> for Or<T0, T1, ..., TN>` (where all `Tk: AsRef<T>`).
Because inherent methods take precedence over trait methods in Rust's method resolution, a
simple `.as_ref()` call on an `Or` value always invokes the inherent method, making the
`AsRef<T>` trait **unreachable via method syntax**.

## Location

**File:** `src/lib.rs`

Inside `or!(@main ...)`:

```rust
// Inherent method — always wins method resolution:
impl<$($t,)*> Or<$($t,)*> {
    pub const fn as_ref(&self) -> Or<$(&$t,)*> {
        match self { ... }
    }
    // ...
}

// Trait implementation — shadowed when called as `.as_ref()`:
impl<T, $($t: AsRef<T>),*> AsRef<T> for Or<$($t,)*> {
    fn as_ref(&self) -> &T {
        match self { ... }
    }
}
```

## Why It Is a Problem

### Rust method resolution rules

In Rust, inherent methods take precedence over trait methods when there is a name conflict
and the trait is not explicitly named at the call site. Concretely:

```rust
use orn::Or2;

let or: Or2<String, String> = Or2::T0("hello".to_string());

// Calls the INHERENT method → returns Or2<&String, &String>
let x = or.as_ref();

// To call AsRef<str>::as_ref, the user must use UFCS:
let y: &str = AsRef::<str>::as_ref(&or);  // works, but very verbose
```

This is surprising and inconsistent with user expectations. The standard library
convention is that `.as_ref()` calls `AsRef::as_ref()`.

### Breaks generic code relying on `AsRef<T>`

Any generic function that expects its argument to implement `AsRef<T>` will call the
*trait* method:

```rust
fn print_as_str<S: AsRef<str>>(s: S) {
    println!("{}", s.as_ref());  // calls AsRef<str>::as_ref
}

print_as_str(Or2::<String, String>::T0("hello".to_string()));
// Works ONLY because AsRef<str> is dispatched through the trait — correct.
```

While trait dispatch (not `.` method syntax) works correctly, the confusion still exists
for users calling `.as_ref()` directly on an `Or` value and expecting `&T`.

### The `AsRef` impl is effectively hidden

Users inspecting the `Or` API will see an `as_ref()` method and naturally use it, never
discovering that `AsRef<T>` is also implemented unless they read the full documentation
carefully.

## Proposed Fix Options

There are two reasonable approaches:

### Option A: Rename the inherent method

Rename the inherent `as_ref` method to something that does not conflict, such as
`as_or_ref` or `each_ref`:

```rust
pub const fn each_ref(&self) -> Or<$(&$t,)*> { ... }
pub fn each_mut(&mut self) -> Or<$(&mut $t,)*> { ... }
```

This mirrors the naming convention in the standard library (`[T; N]::each_ref()`).
This is a **breaking change** to the public API.

### Option B: Keep both, add documentation warning

Keep the current API but add a prominent documentation note explaining the shadowing
and how to call the trait method via UFCS:

```rust
/// # Note on method resolution
///
/// Because this is an inherent method, it takes precedence over the
/// [`AsRef`] trait implementation when called as `.as_ref()`. To call
/// the `AsRef<T>` impl, use UFCS: `AsRef::<T>::as_ref(&value)`.
pub const fn as_ref(&self) -> Or<$(&$t,)*> { ... }
```

### Recommendation

**Option A** is preferred for a clean API. The name `each_ref` / `each_mut` is
self-documenting (it maps *each* variant to a reference), avoids confusion with
`AsRef`, and follows standard library naming. A similar naming conflict appears in
`as_mut` vs `AsMut<T>`.

The same issue applies to `as_mut` (inherent `fn as_mut(&mut self) -> Or<&mut T...>`
vs `AsMut<T>` trait).

## Investigation Steps for the Implementing Agent

1. Open `src/lib.rs`, locate `or!(@main ...)`.
2. Rename the inherent `as_ref` method to `each_ref` and `as_mut` to `each_mut`.
3. Update the `iter` sub-module which calls `self.as_ref().into_iter()` and
   `self.as_mut().into_iter()` inside `Or::iter()` and `Or::iter_mut()` — change
   these to `self.each_ref().into_iter()` and `self.each_mut().into_iter()`.
4. Update all doctests in the method documentation.
5. Run `cargo test --all-features` to confirm no regressions.
6. Bump the crate version (breaking change).
7. Add a note to `CHANGELOG.md` / release notes.

### Alternatively (non-breaking path)

Deprecate `as_ref` (inherent) and `as_mut` (inherent) with `#[deprecated]` pointing
users to `each_ref` and `each_mut`, and keep them for one release cycle before removal.
