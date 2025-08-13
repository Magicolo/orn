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

const fn or<T>(value: T) -> Or2<T, T> {
    Or2::T0(value)
}
