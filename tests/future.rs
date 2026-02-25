#![cfg(feature = "future")]

use core::{
    future::{IntoFuture, ready},
    pin::pin,
};
use orn::Or2;

#[test]
fn compiles() {
    let future = pin!(or(ready('a')).into_future());
    drop(async { future.await.into_inner() });
}

#[test]
fn or0_into_future_compiles() {
    fn assert_into_future<T: IntoFuture>() {}
    assert_into_future::<orn::Or0>();
}

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
