use orn::Or2;
use std::borrow::Cow;

#[test]
fn into_compiles() {
    let value = Or2::<&'static str, Cow<'static, str>>::T0("a").into::<String>();
    assert_eq!(value, "a".to_string());
}

#[test]
fn from_tuple_or2() {
    let array = orn::Or2::<u8, u16>::from_tuple((42u8, 100u16));
    assert_eq!(array[0], orn::Or2::T0(42u8));
    assert_eq!(array[1], orn::Or2::T1(100u16));
}

#[test]
fn from_tuple_or3() {
    let array = orn::Or3::<u8, u16, u32>::from_tuple((1u8, 2u16, 3u32));
    assert_eq!(array[0], orn::Or3::T0(1u8));
    assert_eq!(array[1], orn::Or3::T1(2u16));
    assert_eq!(array[2], orn::Or3::T2(3u32));
}

#[test]
fn try_into_tuple_success_or2() {
    let array = orn::Or2::<u8, u16>::from_tuple((42u8, 100u16));
    let tuple = orn::Or2::<u8, u16>::try_into_tuple(array);
    assert_eq!(tuple, Some((42u8, 100u16)));
}

#[test]
fn try_into_tuple_success_or3() {
    let array = orn::Or3::<u8, u16, u32>::from_tuple((1u8, 2u16, 3u32));
    let tuple = orn::Or3::<u8, u16, u32>::try_into_tuple(array);
    assert_eq!(tuple, Some((1u8, 2u16, 3u32)));
}

#[test]
fn try_into_tuple_failure_wrong_variant() {
    // Element at index 0 has wrong variant (T1 instead of T0)
    let array: [orn::Or2<u8, u16>; 2] = [orn::Or2::T1(100u16), orn::Or2::T1(200u16)];
    let tuple = orn::Or2::<u8, u16>::try_into_tuple(array);
    assert_eq!(tuple, None);
}

#[test]
fn from_tuple_try_into_tuple_roundtrip() {
    let original = (10u8, 20u16, 30u32);
    let array = orn::Or3::<u8, u16, u32>::from_tuple(original);
    let result = orn::Or3::<u8, u16, u32>::try_into_tuple(array);
    assert_eq!(result, Some(original));
}
