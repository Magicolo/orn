#![cfg(feature = "iter")]

use orn::Or2;

#[test]
fn compiles() {
    or(vec!['a']).iter();
    or(vec!['a']).iter_mut();
    or(vec!['a']).into_iter();
    or(vec!['a']).extend(['a']);
}

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
