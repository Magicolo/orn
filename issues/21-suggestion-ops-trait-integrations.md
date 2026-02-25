# Issue 21: Suggestion — Integration with `std::ops` Traits

## Summary

When all variant types in `Or<T0, T1, ..., TN>` implement standard operator traits
(`Add`, `Sub`, `Mul`, `Div`, `Neg`, `Not`, `BitAnd`, `BitOr`, `BitXor`, etc.), it
would be natural for `Or` to implement these too by delegating to the active variant.
This is a **feature suggestion** for improving the library's utility as a general-purpose
sum type.

## Location

**File:** `src/lib.rs`

No `std::ops` trait implementations are generated in the `or!` macro.

## Why It Is a Useful Addition

### Uniform types operating on `Or`

When all variants hold the same or compatible types, users often want to perform arithmetic:

```rust
let a: Or2<i32, f64> = Or2::T0(5);
let b: Or2<i32, f64> = Or2::T0(3);
// Want: let c = a + b  // Or2::T0(8)
```

Without `Add` impl, users must unwrap, add, and rewrap manually.

### The homogeneous case is already partially supported

The `map(f)` method on `Or<T, T, ..., T>` allows applying any function. So `a + b`
could be expressed as `a.map(|x| x + b.into_inner())`, but this is verbose.

### Practical example: numeric processing pipeline

```rust
fn scale(input: Or2<Vec<f32>, Vec<f64>>, factor: Or2<f32, f64>) -> Or2<Vec<f32>, Vec<f64>> {
    // Currently requires match; with Mul impl it could be cleaner
}
```

## Proposed Implementations

### Unary operators (for uniform types)

```rust
impl<T: core::ops::Neg<Output = T>> core::ops::Neg for Or<T, T, ..., T> {
    type Output = Self;
    fn neg(self) -> Self {
        self.map(|x| -x)
    }
}
```

### Binary operators (for uniform types)

```rust
impl<T: core::ops::Add<Output = T>> core::ops::Add for Or<T, T, ..., T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // Both must be the same variant; if not, behavior must be defined.
        // Option 1: panic on variant mismatch
        // Option 2: always use the left operand's variant
    }
}
```

The challenge with binary operators on heterogeneous `Or` types: `Or2::T0(5) + Or2::T1(3)`
— what should this return? The variants differ. Possible answers:
- **Panic**: if variants differ, panic.
- **Left-biased**: use the left operand's variant, map the right through `into_inner()` (requires uniform types).
- **Not supported for heterogeneous types**: only implement for uniform types.

For `Or<T, T, ..., T>` (uniform type), binary operators are straightforward:

```rust
impl<T: core::ops::Add<Output = T>> core::ops::Add for Or<T, T, ..., T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        // Note: Or<T, T> variants T0 and T1 hold the same type T
        // so we can unwrap both and re-wrap
        self.map(|lhs| lhs + rhs.into_inner())
    }
}
```

### Index / IndexMut

```rust
impl<Idx, $($t: core::ops::Index<Idx, Output = Output>,)* Output: ?Sized> 
    core::ops::Index<Idx> for Or<$($t,)*>
{
    type Output = Output;
    fn index(&self, index: Idx) -> &Output {
        match self {
            $(Self::$t(item) => &item[index],)*
        }
    }
}
```

This allows `or_vec_or_array[i]` to work uniformly.

## Investigation Steps for the Implementing Agent

1. Determine which `ops` traits are most useful (priority order):
   - `Index` / `IndexMut` (very useful for slice/vec/array unifcation)
   - `Neg` / `Not` (unary, straightforward for uniform types)
   - `Add`, `Sub`, `Mul`, `Div` (binary, useful for uniform types)
   - `AddAssign`, `SubAssign`, etc.
2. Implement `Index<Idx>` first as a prototype:
   ```rust
   impl<Idx, Output: ?Sized, $($t: core::ops::Index<Idx, Output = Output>),*> 
       core::ops::Index<Idx> for Or<$($t,)*> { ... }
   ```
3. Add tests:
   ```rust
   let v: orn::Or2<Vec<i32>, [i32; 3]> = orn::Or2::T0(vec![1, 2, 3]);
   assert_eq!(v[0], 1);
   ```
4. Open follow-up issues for the arithmetic operators once `Index` is confirmed.
