use orn::Or2;
use std::borrow::Cow;

#[test]
fn or0_standard_trait_bounds_compile() {
    fn assert_clone<T: Clone>() {}
    fn assert_copy<T: Copy>() {}
    fn assert_debug<T: std::fmt::Debug>() {}
    fn assert_eq<T: Eq>() {}
    fn assert_ord<T: Ord>() {}
    fn assert_hash<T: std::hash::Hash>() {}
    fn assert_is<T: orn::Is>() {}
    fn assert_count<T: orn::Count>() {}
    assert_clone::<orn::Or0>();
    assert_copy::<orn::Or0>();
    assert_debug::<orn::Or0>();
    assert_eq::<orn::Or0>();
    assert_ord::<orn::Or0>();
    assert_hash::<orn::Or0>();
    assert_is::<orn::Or0>();
    assert_count::<orn::Or0>();
}

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

#[cfg(feature = "serde")]
#[test]
fn serde_untagged_serialize_or2() {
    let t0: orn::Or2<u8, &str> = orn::Or2::T0(42);
    let t1: orn::Or2<u8, &str> = orn::Or2::T1("hello");
    assert_eq!(serde_json::to_string(&t0).unwrap(), "42");
    assert_eq!(serde_json::to_string(&t1).unwrap(), r#""hello""#);
}

#[cfg(feature = "serde")]
#[test]
fn serde_untagged_deserialize_or2() {
    let t0: orn::Or2<u8, String> = serde_json::from_str("42").unwrap();
    let t1: orn::Or2<u8, String> = serde_json::from_str(r#""hello""#).unwrap();
    assert_eq!(t0, orn::Or2::T0(42));
    assert_eq!(t1, orn::Or2::T1("hello".to_string()));
}

#[cfg(feature = "serde")]
#[test]
fn serde_untagged_roundtrip_or3() {
    let t0: orn::Or3<u8, String, bool> = orn::Or3::T0(1);
    let t1: orn::Or3<u8, String, bool> = orn::Or3::T1("hello".to_string());
    let t2: orn::Or3<u8, String, bool> = orn::Or3::T2(true);
    for (val, json) in [(t0, "1"), (t1, r#""hello""#), (t2, "true")] {
        assert_eq!(serde_json::to_string(&val).unwrap(), json);
        let back: orn::Or3<u8, String, bool> = serde_json::from_str(json).unwrap();
        assert_eq!(back, val);
    }
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

#[test]
fn or1_from_t0() {
    let or1: orn::Or1<u8> = orn::Or1::from(42u8);
    assert_eq!(or1, orn::Or1::T0(42u8));
}

#[test]
fn or1_into_inner() {
    let or1: orn::Or1<u8> = orn::Or1::T0(42u8);
    let back: u8 = or1.into_inner();
    assert_eq!(back, 42u8);
}

#[test]
fn or1_into_syntax() {
    let or1: orn::Or1<u8> = orn::Or1::from(42u8);
    assert_eq!(or1, orn::Or1::T0(42u8));
    let back: u8 = or1.into();
    assert_eq!(back, 42u8);
}
  
fn or2_is_error() {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct E1;
    impl fmt::Display for E1 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "E1")
        }
    }
    impl Error for E1 {}

    #[derive(Debug)]
    struct E2;
    impl fmt::Display for E2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "E2")
        }
    }
    impl Error for E2 {}

    let err: orn::Or2<E1, E2> = orn::Or2::T0(E1);
    let _: &dyn Error = &err;
    assert_eq!(format!("{}", err), "E1");
}

#[test]
fn fmt_write_t0() {
    use core::fmt::Write;
    let mut or: orn::Or2<String, String> = orn::Or2::T0(String::new());
    write!(or, "hello {}", 42).unwrap();
    assert_eq!(or.t0().unwrap(), "hello 42");
}

#[test]
fn fmt_write_t1() {
    use core::fmt::Write;
    let mut or: orn::Or2<String, String> = orn::Or2::T1(String::new());
    write!(or, "world {}", 7).unwrap();
    assert_eq!(or.t1().unwrap(), "world 7");
}

#[cfg(feature = "std")]
#[test]
fn io_write_t0() {
    use std::io::Write;
    let mut or: orn::Or2<Vec<u8>, Vec<u8>> = orn::Or2::T0(Vec::new());
    or.write_all(b"hello").unwrap();
    assert_eq!(or.t0().unwrap(), b"hello");
}

#[cfg(feature = "std")]
#[test]
fn io_read_t0() {
    use std::io::Read;
    let data: &[u8] = b"hello";
    let mut or: orn::Or2<&[u8], &[u8]> = orn::Or2::T0(data);
    let mut buf = [0u8; 5];
    or.read_exact(&mut buf).unwrap();
    assert_eq!(&buf, b"hello");
}
