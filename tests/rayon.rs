#![cfg(feature = "rayon")]

use orn::Or2;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelDrainFull,
    ParallelDrainRange, ParallelExtend, ParallelIterator,
};
use std::collections::HashSet;

#[test]
fn compiles() {
    or(vec!['a']).par_extend(['a']);
    assert_eq!(
        or(HashSet::from(['a']))
            .as_mut()
            .par_drain()
            .map(|value| value.into::<char>())
            .collect::<Vec<_>>(),
        vec!['a']
    );
    assert_eq!(
        or(vec!['a'])
            .as_mut()
            .par_drain(..)
            .map(|value| value.into::<char>())
            .collect::<Vec<_>>(),
        vec!['a']
    );
    assert_eq!(
        or(vec!['a'])
            .par_iter()
            .map(|value| value.copied().into::<char>())
            .collect::<Vec<_>>(),
        vec!['a']
    );
    assert_eq!(
        or(vec!['a'])
            .par_iter_mut()
            .map(|value| value.copied().into::<char>())
            .collect::<Vec<_>>(),
        vec!['a']
    );
    assert_eq!(
        or(vec!['a'])
            .into_par_iter()
            .map(|value| value.into::<char>())
            .collect::<Vec<_>>(),
        vec!['a']
    );
}

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
