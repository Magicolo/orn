#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{
    fmt,
    ops::{Deref, DerefMut},
};

/// A trait for accessing a type at a specific index.
///
/// This trait is implemented for `Or` types and tuples, allowing generic
/// access to their elements.
pub trait At<const I: usize> {
    /// The type of the item at the given index.
    type Item;
    /// Returns the item at the given index.
    fn at(self) -> Self::Item;
}

/// A trait for checking if an `Or` value is of a certain variant by index.
pub trait Is {
    /// Returns `true` if the `Or` value corresponds to the given variant
    /// index.
    ///
    /// # Examples
    ///
    /// ```
    /// use orn::{Is, Or2};
    ///
    /// let value: Or2<u8, &str> = Or2::T0(42);
    /// assert!(value.is(0));
    /// assert!(!value.is(1));
    ///
    /// let value: Or2<u8, &str> = Or2::T1("hello");
    /// assert!(!value.is(0));
    /// assert!(value.is(1));
    /// ```
    fn is(&self, index: usize) -> bool;
}

/// A trait for getting the number of type arguments in a type.
///
/// This is implemented for `Or` types and tuples.
///
/// # Examples
///
/// ```
/// use orn::{Count, Or3};
///
/// assert_eq!(Or3::<u8, u16, u32>::COUNT, 3);
/// assert_eq!(<(u8, u16, u32)>::COUNT, 3);
/// ```
pub trait Count {
    /// The number of type arguments.
    const COUNT: usize;
}

#[inline]
fn same<T, F>(item: T, _: F) -> T {
    item
}

#[inline]
fn with<T, U, F: FnOnce(T) -> U>(item: T, map: F) -> U {
    map(item)
}

/// A type alias for a union of 0 types. This type is uninhabited.
pub type Or0 = or0::Or;

pub mod or0 {
    use super::*;

    /// A union of 0 types. This type is uninhabited, meaning it cannot be
    /// instantiated.
    pub enum Or {}

    impl Count for () {
        const COUNT: usize = 0;
    }

    impl Count for Or {
        const COUNT: usize = 0;
    }
}

macro_rules! ignore_and_stringify {
    ($to_ignore:tt, $to_stringify:ty) => {
        stringify!($to_stringify)
    };
}

macro_rules! type_list {
    ($type:ty, $T1:tt $(, $T:tt)*) => {
        concat!(stringify!($type) $(, ", ", ignore_and_stringify!($T, $type))*)
    };
}

macro_rules! or {
    (
        [$($count: tt, $alias: ident, $module: ident),* $(,)?]
        [$($index: tt, $t: ident, $u: ident, $f: ident, $get: ident, $is: ident, $map: ident),* $(,)?]
    ) => {
        or!(@next [$($count, $alias, $module),*] [$($index, $t, $u, $f, $get, $is, $map),*] []);
    };
    (@next [] [] $old: tt) => {};
    (@next
        [$count: tt, $alias: ident, $module: ident $(, $counts: tt, $aliases: ident, $modules: ident)*]
        [$index: tt, $t: ident, $u: ident, $f: ident, $get: ident, $is: ident, $map: ident $(, $new_index: tt, $new_t: ident, $new_u: ident, $new_f: ident, $new_get: ident, $new_is: ident, $new_map: ident)*]
        [$($old_index: tt, $old_t: ident, $old_u: ident, $old_f: ident, $old_get: ident, $old_is: ident, $old_map: ident),*]
    ) => {
        or!(@main
            $count, $alias, $module
            [$($old_index, T, U, $old_t, $old_u, $old_f, $old_get, $old_is, $old_map,)* $index, T, U, $t, $u, $f, $get, $is, $map]
        );
        or!(@next
            [$($counts, $aliases, $modules),*]
            [$($new_index, $new_t, $new_u, $new_f, $new_get, $new_is, $new_map),*]
            [$($old_index, $old_t, $old_u, $old_f, $old_get, $old_is, $old_map,)* $index, $t, $u, $f, $get, $is, $map]
        );
    };
    (@main $count: tt, $alias: ident, $module: ident [$($index: tt, $same_t: ident, $same_u: ident, $t: ident, $u: ident, $f: ident, $get: ident, $is: ident, $map: ident),*]) => {
        #[doc = concat!("An `enum` of `", stringify!($count), "` variants.")]
        pub type $alias<$($t,)*> = $module::Or<$($t,)*>;

        pub mod $module {
            use super::*;

            #[doc = concat!("An `enum` of `", stringify!($count), "` variants.")]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Or<$($t,)*> {
                $(
                    #[doc = concat!("A variant of the [`Or`] enum that contains a `", stringify!($t), "` value.")]
                    $t($t)
                ),*
            }

            impl<$($t,)*> Or<$($t,)*> {
                /// Converts the [`Or`] into a single value of type `T`.
                ///
                /// This method is available when all types inside the [`Or`] can be converted into `T`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($t),*), "> = ", stringify!($module), "::Or::T0(42);")]
                /// let value: u16 = or.into();
                /// assert_eq!(value, 42u16);
                /// ```
                #[inline]
                pub fn into<T>(self) -> T where $($t: Into<T>),* {
                    match self {
                        $(Self::$t(item) => item.into(),)*
                    }
                }

                /// Converts from `&Or<T...>` to `Or<&T...>`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($t),*), "> = ", stringify!($module), "::Or::T0(42);")]
                #[doc = concat!("let or_ref: ", stringify!($module), "::Or<", type_list!(&u8, $($t),*), "> = or.as_ref();")]
                /// assert!(or_ref.is_t0());
                /// ```
                #[inline]
                pub const fn as_ref(&self) -> Or<$(&$t,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(item),)*
                    }
                }

                /// Converts from `&mut Or<T...>` to `Or<&mut T...>`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                #[doc = concat!("let mut or: ", stringify!($alias), "<", type_list!(u8, $($t),*), "> = ", stringify!($module), "::Or::T0(42);")]
                /// let or_mut = or.as_mut();
                /// *or_mut.t0().unwrap() = 100;
                /// assert_eq!(or.t0(), Some(100));
                /// ```
                #[inline]
                pub fn as_mut(&mut self) -> Or<$(&mut $t,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(item),)*

                    }
                }

                /// Converts from `&Or<T...>` to `Or<&T::Target...>` where `T` implements [`Deref`].
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// use core::ops::Deref;
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(String, $($t),*), "> = ", stringify!($module), "::Or::T0(\"hello\".to_string());")]
                #[doc = concat!("let or_deref: ", stringify!($module), "::Or<", type_list!(&str, $($t),*), "> = or.as_deref();")]
                /// assert_eq!(or_deref.t0(), Some("hello"));
                /// ```
                #[inline]
                pub fn as_deref(&self) -> Or<$(&$t::Target,)*> where $($t: Deref),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.deref()),)*
                    }
                }

                /// Converts from `&mut Or<T...>` to `Or<&mut T::Target...>` where `T` implements [`DerefMut`].
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// use core::ops::DerefMut;
                #[doc = concat!("let mut or: ", stringify!($alias), "<", type_list!(String, $($t),*), "> = ", stringify!($module), "::Or::T0(\"hello\".to_string());")]
                #[doc = concat!("let mut or_deref_mut: ", stringify!($module), "::Or<", type_list!(&mut str, $($t),*), "> = or.as_deref_mut();")]
                #[doc = concat!("if let ", stringify!($module), "::Or::T0(s) = or_deref_mut { s.make_ascii_uppercase(); }")]
                /// assert_eq!(or.t0(), Some("HELLO".to_string()));
                /// ```
                #[inline]
                pub fn as_deref_mut(&mut self) -> Or<$(&mut $t::Target,)*> where $($t: DerefMut),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.deref_mut()),)*
                    }
                }

                /// Converts a tuple `(T0, T1, ..., TN)` into an array `[Or<T0, T1, ..., TN>; N]`.
                ///
                /// Each element in the resulting array contains the corresponding tuple element
                /// wrapped in the matching [`Or`] variant.
                #[inline]
                pub fn from_tuple(tuple: ($($t,)*)) -> [Self; $count] {
                    let ($($get,)*) = tuple;
                    [$( Self::$t($get), )*]
                }

                /// Sorts a slice of [`Or`] values in-place by variant index (`T0` first, then
                /// `T1`, …, `TN` last).
                ///
                /// This is useful as a preprocessing step before calling [`Or::try_into_tuple`],
                /// which requires elements to be in order.
                #[inline]
                pub fn sort_by_variant(slice: &mut [Self]) {
                    slice.sort_unstable_by_key(|item| match item {
                        $(Self::$t(_) => $index,)*
                    });
                }

                /// Tries to convert an array `[Or<T0, T1, ..., TN>; N]` into a tuple `(T0, T1, ..., TN)`.
                ///
                /// Each element must contain the matching variant at its position: index `0` must
                /// be `T0`, index `1` must be `T1`, and so on. Returns `Ok` with the assembled
                /// tuple on success, or `Err` with the original array otherwise.
                ///
                /// Call [`Or::sort_by_variant`] first to handle out-of-order arrays.
                #[inline]
                pub fn try_into_tuple(array: [Self; $count]) -> Result<($($t,)*), [Self; $count]> {
                    #[allow(unreachable_patterns)]
                    match array {
                        [$( Self::$t($get) ),*] => Ok(($($get,)*)),
                        other => Err(other),
                    }
                }
            }

            impl<$($t,)*> Or<$(&$t,)*> {
                /// Maps an `Or<&T...>` to an `Or<T...>` by cloning the contents of the `Or`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// let x = 12;
                #[doc = concat!("let or_ref: ", stringify!($alias), "<", type_list!(&i32, $($t),*), "> = ", stringify!($module), "::Or::T0(&x);")]
                #[doc = concat!("let cloned: ", stringify!($alias), "<", type_list!(i32, $($t),*), "> = or_ref.cloned();")]
                #[doc = concat!("assert_eq!(cloned, ", stringify!($module), "::Or::T0(12));")]
                /// ```
                ///
                /// ```compile_fail
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// struct NonClone;
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(&NonClone, $($t),*), "> = ", stringify!($module), "::Or::T0(&NonClone);")]
                /// or.cloned();
                /// ```
                #[inline]
                pub fn cloned(self) -> Or<$($t,)*> where $($t: Clone),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.clone()),)*
                    }
                }

                /// Maps an `Or<&T...>` to an `Or<T...>` by copying the contents of the `Or`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// let x = 12;
                #[doc = concat!("let or_ref: ", stringify!($alias), "<", type_list!(&i32, $($t),*), "> = ", stringify!($module), "::Or::T0(&x);")]
                #[doc = concat!("let copied: ", stringify!($alias), "<", type_list!(i32, $($t),*), "> = or_ref.copied();")]
                #[doc = concat!("assert_eq!(copied, ", stringify!($module), "::Or::T0(12));")]
                /// ```
                ///
                /// ```compile_fail
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// struct NonCopy;
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(&NonCopy, $($t),*), "> = ", stringify!($module), "::Or::T0(&NonCopy);")]
                /// or.copied();
                /// ```
                #[inline]
                pub fn copied(self) -> Or<$($t,)*> where $($t: Copy),* {
                    match self {
                        $(Self::$t(item) => Or::$t(*item),)*
                    }
                }
            }

            impl<$($t,)*> Or<$(&mut $t,)*> {
                /// Maps an `Or<&mut T...>` to an `Or<T...>` by cloning the contents of the `Or`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// let mut x = 12;
                #[doc = concat!("let or_mut_ref: ", stringify!($alias), "<", type_list!(&mut i32, $($t),*), "> = ", stringify!($module), "::Or::T0(&mut x);")]
                #[doc = concat!("let cloned: ", stringify!($alias), "<", type_list!(i32, $($t),*), "> = or_mut_ref.cloned();")]
                #[doc = concat!("assert_eq!(cloned, ", stringify!($module), "::Or::T0(12));")]
                /// ```
                #[inline]
                pub fn cloned(self) -> Or<$($t,)*> where $($t: Clone),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.clone()),)*
                    }
                }

                /// Maps an `Or<&mut T...>` to an `Or<T...>` by copying the contents of the `Or`.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                /// let mut x = 12;
                #[doc = concat!("let or_mut_ref: ", stringify!($alias), "<", type_list!(&mut i32, $($t),*), "> = ", stringify!($module), "::Or::T0(&mut x);")]
                #[doc = concat!("let copied: ", stringify!($alias), "<", type_list!(i32, $($t),*), "> = or_mut_ref.copied();")]
                #[doc = concat!("assert_eq!(copied, ", stringify!($module), "::Or::T0(12));")]
                /// ```
                #[inline]
                pub fn copied(self) -> Or<$($t,)*> where $($t: Copy),* {
                    match self {
                        $(Self::$t(item) => Or::$t(*item),)*
                    }
                }
            }

            impl<T> Or<$($same_t,)*> {
                /// Extracts the inner value from the `Or`.
                ///
                /// This method is available when all types in the [`Or`] are the same.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($t),*), "> = ", stringify!($module), "::Or::T0(42);")]
                /// assert_eq!(or.into_inner(), 42);
                /// ```
                #[inline]
                pub fn into_inner(self) -> T {
                    match self {
                        $(Self::$t(item) => item,)*
                    }
                }

                /// Maps an `Or<T, T...>` to an `Or<U, U...>` by applying a function to the contained value.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($t),*), "> = ", stringify!($module), "::Or::T0(42);")]
                #[doc = concat!("let mapped: ", stringify!($alias), "<", type_list!(u16, $($t),*), "> = or.map(|x| x as u16);")]
                /// assert_eq!(mapped.into_inner(), 42u16);
                /// ```
                #[inline]
                pub fn map<U, F: FnOnce(T) -> U>(self, map: F) -> Or<$($same_u,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(map(item)),)*
                    }
                }
            }

            impl<T, $($t: AsRef<T>),*> AsRef<T> for Or<$($t,)*> {
                /// Returns a reference to the inner value.
                ///
                /// This method is available when all types in the [`Or`] implement [`AsRef<T>`].
                #[inline]
                fn as_ref(&self) -> &T {
                    match self {
                        $(Self::$t(item) => item.as_ref(),)*
                    }
                }
            }

            impl<T, $($t: AsMut<T>),*> AsMut<T> for Or<$($t,)*> {
                /// Returns a mutable reference to the inner value.
                ///
                /// This method is available when all types in the [`Or`] implement [`AsMut<T>`].
                #[inline]
                fn as_mut(&mut self) -> &mut T {
                    match self {
                        $(Self::$t(item) => item.as_mut(),)*
                    }
                }
            }

            impl<$($t: fmt::Display,)*> fmt::Display for Or<$($t,)*> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        $(Self::$t(item) => fmt::Display::fmt(item, f),)*
                    }
                }
            }

            impl<$($t,)*> Count for ($($t,)*) {
                const COUNT: usize = $count;
            }

            impl<$($t,)*> Count for Or<$($t,)*> {
                const COUNT: usize = $count;
            }

            impl<$($t,)*> Is for Or<$($t,)*> {
                #[inline]
                fn is(&self, index: usize) -> bool {
                    match (index, self) {
                        $(($index, Self::$t(_)) => true,)*
                        _ => false,
                    }
                }
            }

            #[cfg(feature = "iter")]
            pub mod iter {
                use super::Or;
                use core::{self, iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator}};

                /// An iterator that yields the items of an [`Or`] of iterators.
                #[derive(Clone, Copy, Debug)]
                pub enum Iterator<$($t,)*> { $($t($t)),* }

                impl<$($t,)*> Or<$($t,)*> {
                    /// Creates an iterator from a reference to an [`Or`].
                    ///
                    /// # Examples
                    ///
                    /// ```
                    #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                    #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(Vec<u8>, $($t),*), "> = ", stringify!($module), "::Or::T0(vec![1, 2, 3]);")]
                    /// let mut iter = or.iter();
                    /// // The iterator's item type is `Or<T...>` where `T` is `&u8`, which has `into_inner`.
                    /// assert_eq!(iter.next().unwrap().into_inner(), &1);
                    /// ```
                    #[inline]
                    pub fn iter(&self) -> Iterator<$(<&$t as IntoIterator>::IntoIter,)*> where $(for<'a> &'a $t: IntoIterator,)* {
                        self.as_ref().into_iter()
                    }

                    /// Creates a mutable iterator from a mutable reference to an [`Or`].
                    ///
                    /// # Examples
                    ///
                    /// ```
                    #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                    #[doc = concat!("let mut or: ", stringify!($alias), "<", type_list!(Vec<u8>, $($t),*), "> = ", stringify!($module), "::Or::T0(vec![1, 2, 3]);")]
                    /// let mut iter = or.iter_mut();
                    #[doc = concat!("if let ", stringify!($module), "::Or::T0(val) = iter.next().unwrap() { *val = 42; }")]
                    /// assert_eq!(or.t0().unwrap()[0], 42);
                    /// ```
                    #[inline]
                    pub fn iter_mut(&mut self) -> Iterator<$(<&mut $t as IntoIterator>::IntoIter,)*> where $(for<'a> &'a mut $t: IntoIterator,)* {
                        self.as_mut().into_iter()
                    }
                }

                impl<$($t: IntoIterator),*> IntoIterator for Or<$($t,)*> {
                    type IntoIter = Iterator<$($t::IntoIter,)*>;
                    type Item = Or<$($t::Item,)*>;

                    /// Creates a consuming iterator.
                    ///
                    /// # Examples
                    ///
                    /// ```
                    #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
                    #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(Vec<u8>, $($t),*), "> = ", stringify!($module), "::Or::T0(vec![1, 2, 3]);")]
                    /// let mut iter = or.into_iter();
                    /// // The iterator's item type is `Or<T...>` where `T` is `u8`, which has `into_inner`.
                    /// assert_eq!(iter.next().unwrap().into_inner(), 1);
                    /// ```
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        match self {
                            $(Self::$t(item) => Iterator::$t(item.into_iter()),)*
                        }
                    }
                }

                impl<$($t: core::iter::Iterator),*> core::iter::Iterator for Iterator<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;

                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        match self {
                            $(Self::$t(item) => Some(Or::$t(item.next()?)),)*
                        }
                    }

                    #[inline]
                    fn size_hint(&self) -> (usize, Option<usize>) {
                        match self {
                            $(Self::$t(item) => item.size_hint(),)*
                        }
                    }

                    #[inline]
                    fn count(self) -> usize {
                        match self {
                            $(Self::$t(item) => item.count(),)*
                        }
                    }

                    #[inline]
                    fn nth(&mut self, n: usize) -> Option<Self::Item> {
                        match self {
                            $(Self::$t(item) => item.nth(n).map(Or::$t),)*
                        }
                    }

                    #[inline]
                    fn fold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, mut f: F) -> B {
                        match self {
                            $(Self::$t(item) => item.fold(init, |acc, x| f(acc, Or::$t(x))),)*
                        }
                    }

                    #[inline]
                    fn for_each<F: FnMut(Self::Item)>(self, mut f: F) {
                        match self {
                            $(Self::$t(item) => item.for_each(|x| f(Or::$t(x))),)*
                        }
                    }
                }

                impl<$($t: DoubleEndedIterator),*> DoubleEndedIterator for Iterator<$($t,)*> {
                    #[inline]
                    fn next_back(&mut self) -> Option<Self::Item> {
                        match self {
                            $(Self::$t(item) => Some(Or::$t(item.next_back()?)),)*
                        }
                    }

                    #[inline]
                    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                        match self {
                            $(Self::$t(item) => item.nth_back(n).map(Or::$t),)*
                        }
                    }

                    #[inline]
                    fn rfold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, mut f: F) -> B {
                        match self {
                            $(Self::$t(item) => item.rfold(init, |acc, x| f(acc, Or::$t(x))),)*
                        }
                    }
                }

                impl<$($t: ExactSizeIterator),*> ExactSizeIterator for Iterator<$($t,)*> {
                    #[inline]
                    fn len(&self) -> usize {
                        match self {
                            $(Self::$t(item) => item.len(),)*
                        }
                    }
                }

                impl<$($t: FusedIterator),*> FusedIterator for Iterator<$($t,)*> { }

                impl<T, $($t: Extend<T>,)*> Extend<T> for Or<$($t,)*> {
                    /// Extends the [`Or`] with the contents of an iterator.
                    #[inline]
                    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                        match self {
                            $(Self::$t(item) => item.extend(iter),)*
                        }
                    }
                }
            }

            #[cfg(feature = "future")]
            pub mod future {
                use super::Or;
                use core::{
                    future::{self, IntoFuture},
                    pin::Pin,
                    task::{Context, Poll},
                };
                use pin_project::pin_project;

                /// A future that resolves to an [`Or`] of the outputs of the inner futures.
                #[pin_project(project = Project)]
                pub enum Future<$($t,)*> {
                    $($t(#[pin] $t),)*
                }

                impl<$($t: IntoFuture),*> IntoFuture for Or<$($t,)*> {
                    type IntoFuture = Future<$($t::IntoFuture,)*>;
                    type Output = Or<$($t::Output,)*>;

                    #[inline]
                    fn into_future(self) -> Self::IntoFuture {
                        match self {
                            $(Self::$t(item) => Future::$t(item.into_future()),)*
                        }
                    }
                }

                impl<$($t: future::Future),*> future::Future for Future<$($t,)*> {
                    type Output = Or<$($t::Output,)*>;

                    #[inline]
                    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                        match self.project() {
                            $(Project::$t(item) => item.poll(cx).map(Or::$t),)*
                        }
                    }
                }
            }

            #[cfg(feature = "rayon")]
            pub mod rayon {
                use super::Or;
                use ::rayon::iter::{
                    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
                    ParallelDrainFull, ParallelDrainRange, ParallelExtend, ParallelIterator,
                    IndexedParallelIterator,
                    plumbing::{UnindexedConsumer, Producer, ProducerCallback, Folder, Consumer},
                };
                use core::{iter::Map, ops::RangeBounds, marker::PhantomData};

                /// A parallel iterator that yields the items of an [`Or`] of parallel iterators.
                #[derive(Clone, Copy, Debug)]
                pub enum Iterator<$($t,)*> { $($t($t)),* }
                pub struct One<T, $($t: ?Sized,)* const N: usize>(pub T, $(PhantomData<$t>,)*);

                impl<T, $($t: ?Sized,)* const N: usize> One<T, $($t,)* N> {
                    #[inline]
                    pub const fn new(value: T) -> Self {
                        Self(value, $(PhantomData::<$t>,)*)
                    }
                }

                impl<$($t: IntoParallelIterator,)*> IntoParallelIterator for Or<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;
                    type Iter = Iterator<$($t::Iter,)*>;

                    #[inline]
                    fn into_par_iter(self) -> Self::Iter {
                        match self {
                            $(Self::$t(item) => Iterator::$t(item.into_par_iter()),)*
                        }
                    }
                }

                impl<'data, $($t: IntoParallelRefIterator<'data>,)*> IntoParallelRefIterator<'data> for Or<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;
                    type Iter = Iterator<$($t::Iter,)*>;

                    #[inline]
                    fn par_iter(&'data self) -> Self::Iter {
                        match self {
                            $(Self::$t(item) => Iterator::$t(item.par_iter()),)*
                        }
                    }
                }

                impl<'data, $($t: IntoParallelRefMutIterator<'data>,)*> IntoParallelRefMutIterator<'data> for Or<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;
                    type Iter = Iterator<$($t::Iter,)*>;

                    #[inline]
                    fn par_iter_mut(&'data mut self) -> Self::Iter {
                        match self {
                            $(Self::$t(item) => Iterator::$t(item.par_iter_mut()),)*
                        }
                    }
                }

                impl<$($t: ParallelIterator,)*> ParallelIterator for Iterator<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;

                    #[inline]
                    fn drive_unindexed<C>(self, consumer: C) -> C::Result
                    where
                        C: UnindexedConsumer<Self::Item>,
                    {
                        match self {
                            $(Self::$t(item) => item.map(Or::$t).drive_unindexed(consumer),)*
                        }
                    }

                    #[inline]
                    fn opt_len(&self) -> Option<usize> {
                        match self {
                            $(Self::$t(item) => item.opt_len(),)*
                        }
                    }
                }

                impl<$($t: ParallelDrainFull,)*> ParallelDrainFull for Or<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;
                    type Iter = Iterator<$($t::Iter,)*>;

                    #[inline]
                    fn par_drain(self) -> Self::Iter {
                        match self {
                            $(Self::$t(value) => Iterator::$t(value.par_drain()),)*
                        }
                    }
                }

                impl<$($t: ParallelDrainRange,)*> ParallelDrainRange for Or<$($t,)*> {
                    type Item = Or<$($t::Item,)*>;
                    type Iter = Iterator<$($t::Iter,)*>;

                    #[inline]
                    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
                        match self {
                            $(Self::$t(value) => Iterator::$t(value.par_drain(range)),)*
                        }
                    }
                }

                impl<T: Send, $($t: ParallelExtend<T>,)*> ParallelExtend<T> for Or<$($t,)*> {
                    #[inline]
                    fn par_extend<I>(&mut self, par_iter: I)
                    where
                        I: IntoParallelIterator<Item = T>,
                    {
                        match self {
                            $(Self::$t(value) => value.par_extend(par_iter),)*
                        }
                    }
                }


                impl<$($t: IndexedParallelIterator,)*> IndexedParallelIterator for Iterator<$($t,)*> {
                    #[inline]
                    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
                        match self {
                            $(Self::$t(item) => item.map(Or::$t).drive(consumer),)*
                        }
                    }

                    #[inline]
                    fn len(&self) -> usize {
                        match self {
                            $(Self::$t(item) => item.len(),)*
                        }
                    }

                    #[inline]
                    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
                        match self {
                            $(Self::$t(item) => item.with_producer(One::new(callback)),)*
                        }
                    }
                }

                or!(@rayon @outer [$($t),*] [$($index, $t),*]);
            }

            or!(@outer $count, $alias, $module [$($index, $t, $get, $is, $map),*] []);
        }
    };
    (@outer $count:tt, $alias:ident, $module:ident [] $old:tt) => {};
    (@outer
        $count:tt, $alias:ident, $module:ident
        [$index: tt, $t: ident, $get: ident, $is: ident, $map: ident $(, $new_index: tt, $new_t: ident, $new_get: ident, $new_is: ident, $new_map: ident)*]
        [$($old_t: ident),*]
    ) => {
        or!(@inner $count, $alias, $module, $index, $t, $get, $is, $map [$($old_t, $old_t, same,)* $t, U, with $(, $new_t, $new_t, same)*]);
        or!(@outer $count, $alias, $module [$($new_index, $new_t, $new_get, $new_is, $new_map),*] [$($old_t,)* $t]);
    };
    (@inner $count: tt, $alias: ident, $module: ident, $index: tt, $t: ident, $get: ident, $is: ident, $map: ident [$($ts: ident, $map_t: ident, $map_f: ident),*]) => {
        impl<$($ts),*> Or<$($ts,)*> {
            #[doc = concat!("Returns `Some(T)` if the [`Or`] contains the `", stringify!($t), "` variant, otherwise `None`.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use orn::{", stringify!($module), "::Or, ", stringify!($alias), "};")]
            #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($ts),*), "> = Or::", stringify!($t), "(42);")]
            #[doc = concat!("assert_eq!(or.", stringify!($get), "(), Some(42));")]
            /// ```
            #[inline]
            pub fn $get(self) -> Option<$t> {
                match self {
                    Self::$t(item) => Some(item),
                    #[allow(unreachable_patterns)]
                    _ => None
                }
            }

            #[doc = concat!("Returns `true` if the [`Or`] contains the `", stringify!($t), "` variant.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
            #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($ts),*), "> = ", stringify!($module), "::Or::", stringify!($t), "(42);")]
            #[doc = concat!("assert!(or.", stringify!($is), "());")]
            /// ```
            #[inline]
            pub fn $is(&self) -> bool {
                match self {
                    Self::$t(_) => true,
                    #[allow(unreachable_patterns)]
                    _ => false
                }
            }

            #[doc = concat!("Maps the `", stringify!($t), "` variant of the [`Or`] with the provided function.")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use orn::{", stringify!($module), ", ", stringify!($alias), "};")]
            #[doc = concat!("let or: ", stringify!($alias), "<", type_list!(u8, $($ts),*), "> = ", stringify!($module), "::Or::", stringify!($t), "(42);")]
            #[doc = concat!("let mapped = or.", stringify!($map), "(|x| x.to_string());")]
            #[doc = concat!("assert_eq!(mapped.", stringify!($get), "(), Some(\"42\".to_string()));")]
            /// ```
            #[inline]
            pub fn $map<U, F: FnOnce($t) -> U>(self, map: F) -> Or<$($map_t,)*> {
                match self {
                    $(Self::$ts(item) => Or::$ts($map_f(item, map)),)*
                }
            }
        }

        impl<$($ts),*> At<$index> for ($($ts,)*) {
            type Item = $t;
            #[inline]
            fn at(self) -> Self::Item {
                self.$index
            }
        }

        impl<'a, $($ts),*> At<$index> for &'a ($($ts,)*) {
            type Item = &'a $t;
            #[inline]
            fn at(self) -> Self::Item {
                &self.$index
            }
        }

        impl<'a, $($ts),*> At<$index> for &'a mut ($($ts,)*) {
            type Item = &'a mut $t;
            #[inline]
            fn at(self) -> Self::Item {
                &mut self.$index
            }
        }

        impl<$($ts),*> At<$index> for Or<$($ts,)*> {
            type Item = Option<$t>;
            #[inline]
            fn at(self) -> Self::Item {
                match self {
                    Self::$t(item) => Some(item),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        }

        impl<'a, $($ts),*> At<$index> for &'a Or<$($ts,)*> {
            type Item = Option<&'a $t>;
            #[inline]
            fn at(self) -> Self::Item {
                match self {
                    Or::$t(item) => Some(item),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        }

        impl<'a, $($ts),*> At<$index> for &'a mut Or<$($ts,)*> {
            type Item = Option<&'a mut $t>;
            #[inline]
            fn at(self) -> Self::Item {
                match self {
                    Or::$t(item) => Some(item),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        }
    };
    (@rayon @outer $types: tt [$($index: tt, $type: ident),*]) => {
        $(or!(@rayon @inner $index, $type $types);)*
    };
    (@rayon @inner $index: tt, $type: ident [$($t: ident),*]) => {
        impl<$($t: Send,)* C: ProducerCallback<Or<$($t,)*>>> ProducerCallback<$type> for One<C, $($t,)* $index> {
            type Output = C::Output;

            #[inline]
            fn callback<P>(self, producer: P) -> Self::Output
            where
                P: Producer<Item = $type>,
            {
                self.0.callback(One::<_, $($t,)* $index>::new(producer))
            }
        }

        impl<$($t: Send,)* P: Producer<Item = $type>> Producer for One<P, $($t,)* $index> {
            type IntoIter = Map<P::IntoIter, fn($type) -> Self::Item>;
            type Item = Or<$($t,)*>;

            #[inline]
            fn fold_with<F>(self, folder: F) -> F
            where
                F: Folder<Self::Item>,
            {
                self.0.fold_with(One::new(folder)).0
            }

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter().map(Or::$type)
            }

            #[inline]
            fn max_len(&self) -> usize {
                self.0.max_len()
            }

            #[inline]
            fn min_len(&self) -> usize {
                self.0.min_len()
            }

            #[inline]
            fn split_at(self, index: usize) -> (Self, Self) {
                let (left, right) = self.0.split_at(index);
                (Self::new(left), Self::new(right))
            }
        }

        impl<$($t,)* F: Folder<Or<$($t,)*>>> Folder<$type> for One<F, $($t,)* $index> {
            type Result = F::Result;

            #[inline]
            fn complete(self) -> Self::Result {
                self.0.complete()
            }

            #[inline]
            fn consume(self, item: $type) -> Self {
                Self::new(self.0.consume(Or::$type(item)))
            }

            #[inline]
            fn consume_iter<I>(self, iter: I) -> Self
            where
                I: IntoIterator<Item = $type>,
            {
                Self::new(self.0.consume_iter(iter.into_iter().map(Or::$type)))
            }

            #[inline]
            fn full(&self) -> bool {
                self.0.full()
            }
        }
    };
}

#[cfg(all(not(feature = "or16"), not(feature = "or32")))]
or!(
    [
        1, Or1, or1,
        2, Or2, or2,
        3, Or3, or3,
        4, Or4, or4,
        5, Or5, or5,
        6, Or6, or6,
        7, Or7, or7,
        8, Or8, or8,
    ]
    [
        0, T0, U0, F0, t0, is_t0, map_t0,
        1, T1, U1, F1, t1, is_t1, map_t1,
        2, T2, U2, F2, t2, is_t2, map_t2,
        3, T3, U3, F3, t3, is_t3, map_t3,
        4, T4, U4, F4, t4, is_t4, map_t4,
        5, T5, U5, F5, t5, is_t5, map_t5,
        6, T6, U6, F6, t6, is_t6, map_t6,
        7, T7, U7, F7, t7, is_t7, map_t7,
    ]
);

#[cfg(all(feature = "or16", not(feature = "or32")))]
or!(
    [
        1, Or1, or1,
        2, Or2, or2,
        3, Or3, or3,
        4, Or4, or4,
        5, Or5, or5,
        6, Or6, or6,
        7, Or7, or7,
        8, Or8, or8,
        9, Or9, or9,
        10, Or10, or10,
        11, Or11, or11,
        12, Or12, or12,
        13, Or13, or13,
        14, Or14, or14,
        15, Or15, or15,
        16, Or16, or16,
    ]
    [
        0, T0, U0, F0, t0, is_t0, map_t0,
        1, T1, U1, F1, t1, is_t1, map_t1,
        2, T2, U2, F2, t2, is_t2, map_t2,
        3, T3, U3, F3, t3, is_t3, map_t3,
        4, T4, U4, F4, t4, is_t4, map_t4,
        5, T5, U5, F5, t5, is_t5, map_t5,
        6, T6, U6, F6, t6, is_t6, map_t6,
        7, T7, U7, F7, t7, is_t7, map_t7,
        8, T8, U8, F8, t8, is_t8, map_t8,
        9, T9, U9, F9, t9, is_t9, map_t9,
        10, T10, U10, F10, t10, is_t10, map_t10,
        11, T11, U11, F11, t11, is_t11, map_t11,
        12, T12, U12, F12, t12, is_t12, map_t12,
        13, T13, U13, F13, t13, is_t13, map_t13,
        14, T14, U14, F14, t14, is_t14, map_t14,
        15, T15, U15, F15, t15, is_t15, map_t15,
    ]
);

#[cfg(feature = "or32")]
or!(
    [
        1, Or1, or1,
        2, Or2, or2,
        3, Or3, or3,
        4, Or4, or4,
        5, Or5, or5,
        6, Or6, or6,
        7, Or7, or7,
        8, Or8, or8,
        9, Or9, or9,
        10, Or10, or10,
        11, Or11, or11,
        12, Or12, or12,
        13, Or13, or13,
        14, Or14, or14,
        15, Or15, or15,
        16, Or16, or16,
        17, Or17, or17,
        18, Or18, or18,
        19, Or19, or19,
        20, Or20, or20,
        21, Or21, or21,
        22, Or22, or22,
        23, Or23, or23,
        24, Or24, or24,
        25, Or25, or25,
        26, Or26, or26,
        27, Or27, or27,
        28, Or28, or28,
        29, Or29, or29,
        30, Or30, or30,
        31, Or31, or31,
        32, Or32, or32,
    ]
    [
        0, T0, U0, F0, t0, is_t0, map_t0,
        1, T1, U1, F1, t1, is_t1, map_t1,
        2, T2, U2, F2, t2, is_t2, map_t2,
        3, T3, U3, F3, t3, is_t3, map_t3,
        4, T4, U4, F4, t4, is_t4, map_t4,
        5, T5, U5, F5, t5, is_t5, map_t5,
        6, T6, U6, F6, t6, is_t6, map_t6,
        7, T7, U7, F7, t7, is_t7, map_t7,
        8, T8, U8, F8, t8, is_t8, map_t8,
        9, T9, U9, F9, t9, is_t9, map_t9,
        10, T10, U10, F10, t10, is_t10, map_t10,
        11, T11, U11, F11, t11, is_t11, map_t11,
        12, T12, U12, F12, t12, is_t12, map_t12,
        13, T13, U13, F13, t13, is_t13, map_t13,
        14, T14, U14, F14, t14, is_t14, map_t14,
        15, T15, U15, F15, t15, is_t15, map_t15,
        16, T16, U16, F16, t16, is_t16, map_t16,
        17, T17, U17, F17, t17, is_t17, map_t17,
        18, T18, U18, F18, t18, is_t18, map_t18,
        19, T19, U19, F19, t19, is_t19, map_t19,
        20, T20, U20, F20, t20, is_t20, map_t20,
        21, T21, U21, F21, t21, is_t21, map_t21,
        22, T22, U22, F22, t22, is_t22, map_t22,
        23, T23, U23, F23, t23, is_t23, map_t23,
        24, T24, U24, F24, t24, is_t24, map_t24,
        25, T25, U25, F25, t25, is_t25, map_t25,
        26, T26, U26, F26, t26, is_t26, map_t26,
        27, T27, U27, F27, t27, is_t27, map_t27,
        28, T28, U28, F28, t28, is_t28, map_t28,
        29, T29, U29, F29, t29, is_t29, map_t29,
        30, T30, U30, F30, t30, is_t30, map_t30,
        31, T31, U31, F31, t31, is_t31, map_t31,
    ]
);

impl<T0> From<T0> for or1::Or<T0> {
    #[inline]
    fn from(value: T0) -> Self {
        Self::T0(value)
    }
}
