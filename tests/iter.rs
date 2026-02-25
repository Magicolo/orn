#![cfg(feature = "iter")]

use orn::Or2;

#[test]
fn compiles() {
    or(vec!['a']).iter();
    or(vec!['a']).iter_mut();
    or(vec!['a']).into_iter();
    or(vec!['a']).extend(['a']);
}

#[test]
fn size_hint_exact_size() {
    let v = vec![1u8, 2u8, 3u8];
    let iter = Or2::<Vec<u8>, Vec<u8>>::T0(v).into_iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
}

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
