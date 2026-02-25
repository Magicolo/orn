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
fn size_hint() {
    assert_eq!(or(vec![1, 2, 3]).into_iter().size_hint(), (3, Some(3)));
}

#[test]
fn count() {
    assert_eq!(or(vec![1, 2, 3]).into_iter().count(), 3);
}

#[test]
fn nth() {
    assert_eq!(or(vec![1, 2, 3]).into_iter().nth(1).unwrap().into_inner(), 2);
}

#[test]
fn fold() {
    let sum = or(vec![1, 2, 3]).into_iter().fold(0, |acc, x| acc + x.into_inner());
    assert_eq!(sum, 6);
}

#[test]
fn for_each() {
    let mut sum = 0;
    or(vec![1, 2, 3]).into_iter().for_each(|x| sum += x.into_inner());
    assert_eq!(sum, 6);
}

#[test]
fn nth_back() {
    assert_eq!(or(vec![1, 2, 3]).into_iter().nth_back(0).unwrap().into_inner(), 3);
}

#[test]
fn rfold() {
    let sum = or(vec![1, 2, 3]).into_iter().rfold(0, |acc, x| acc + x.into_inner());
    assert_eq!(sum, 6);
}

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
