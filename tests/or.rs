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
    assert_eq!(tuple, Ok((42u8, 100u16)));
}

#[test]
fn try_into_tuple_success_or3() {
    let array = orn::Or3::<u8, u16, u32>::from_tuple((1u8, 2u16, 3u32));
    let tuple = orn::Or3::<u8, u16, u32>::try_into_tuple(array);
    assert_eq!(tuple, Ok((1u8, 2u16, 3u32)));
}

#[test]
fn sort_by_variant_or2() {
    let mut array: [orn::Or2<u8, u16>; 2] = [orn::Or2::T1(100u16), orn::Or2::T0(42u8)];
    orn::Or2::<u8, u16>::sort_by_variant(&mut array);
    assert_eq!(array[0], orn::Or2::T0(42u8));
    assert_eq!(array[1], orn::Or2::T1(100u16));
}

#[test]
fn sort_by_variant_or3() {
    let mut array: [orn::Or3<u8, u16, u32>; 3] =
        [orn::Or3::T2(3u32), orn::Or3::T0(1u8), orn::Or3::T1(2u16)];
    orn::Or3::<u8, u16, u32>::sort_by_variant(&mut array);
    assert_eq!(array[0], orn::Or3::T0(1u8));
    assert_eq!(array[1], orn::Or3::T1(2u16));
    assert_eq!(array[2], orn::Or3::T2(3u32));
}

#[test]
fn try_into_tuple_out_of_order_or2() {
    // sort first, then convert
    let mut array: [orn::Or2<u8, u16>; 2] = [orn::Or2::T1(100u16), orn::Or2::T0(42u8)];
    orn::Or2::<u8, u16>::sort_by_variant(&mut array);
    assert_eq!(orn::Or2::<u8, u16>::try_into_tuple(array), Ok((42u8, 100u16)));
}

#[test]
fn try_into_tuple_out_of_order_or3() {
    // sort first, then convert
    let mut array: [orn::Or3<u8, u16, u32>; 3] =
        [orn::Or3::T2(3u32), orn::Or3::T0(1u8), orn::Or3::T1(2u16)];
    orn::Or3::<u8, u16, u32>::sort_by_variant(&mut array);
    assert_eq!(orn::Or3::<u8, u16, u32>::try_into_tuple(array), Ok((1u8, 2u16, 3u32)));
}

#[test]
fn try_into_tuple_failure_duplicate_or2() {
    // Duplicate T1, missing T0 — must be Err
    let array: [orn::Or2<u8, u16>; 2] = [orn::Or2::T1(100u16), orn::Or2::T1(200u16)];
    assert!(orn::Or2::<u8, u16>::try_into_tuple(array).is_err());
}

#[test]
fn try_into_tuple_failure_duplicate_or3() {
    // Duplicate T0, missing T2 — must be Err
    let array: [orn::Or3<u8, u16, u32>; 3] =
        [orn::Or3::T0(1u8), orn::Or3::T1(2u16), orn::Or3::T0(3u8)];
    assert!(orn::Or3::<u8, u16, u32>::try_into_tuple(array).is_err());
}

#[test]
fn from_tuple_try_into_tuple_roundtrip() {
    let original = (10u8, 20u16, 30u32);
    let array = orn::Or3::<u8, u16, u32>::from_tuple(original);
    let result = orn::Or3::<u8, u16, u32>::try_into_tuple(array);
    assert_eq!(result, Ok(original));
}

#[test]
fn display_t0() {
    let or: orn::Or2<u8, u16> = orn::Or2::T0(42u8);
    assert_eq!(format!("{}", or), "42");
}

#[test]
fn display_t1() {
    let or: orn::Or2<u8, u16> = orn::Or2::T1(100u16);
    assert_eq!(format!("{}", or), "100");
}

#[test]
fn display_str_variant() {
    let or: orn::Or2<&str, u8> = orn::Or2::T0("hello");
    assert_eq!(format!("{}", or), "hello");
}
