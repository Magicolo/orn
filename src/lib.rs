#![no_std]

pub trait At<const N: usize> {
    type Item;
    fn at(self) -> Self::Item;
}

pub trait Is<const N: usize> {
    fn is(&self) -> bool;
}

macro_rules! or {
    ($count: expr, $or: ident, $module: ident $(, $index: tt, $upper: ident, $lower: ident)*) => {
        pub type $or<$($upper),*> = $module::Or<$($upper),*>;

        pub mod $module {
            #[allow(unused_imports)]
            use core::{iter, ops::{Deref, DerefMut}};
            #[allow(unused_imports)]
            use super::{At, Is};

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Or<$($upper),*> { $($upper($upper)),* }
            #[derive(Clone, Copy, Debug)]
            pub enum Iterator<$($upper),*> { $($upper($upper)),* }

            impl<$($upper),*> Or<$($upper),*> {
                #[inline]
                pub fn into<T>(self) -> T where $($upper: Into<T>),* {
                    match self {
                        $(Self::$upper(item) => item.into(),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                pub const fn as_ref(&self) -> Or<$(&$upper,)*> {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                pub fn as_mut(&mut self) -> Or<$(&mut $upper,)*> {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()

                    }
                }

                #[inline]
                pub fn as_deref(&self) -> Or<$(&$upper::Target,)*> where $($upper: Deref),* {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item.deref()),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                pub fn as_deref_mut(&mut self) -> Or<$(&mut $upper::Target,)*> where $($upper: DerefMut),* {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item.deref_mut()),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()

                    }
                }
            }

            impl<$($upper: IntoIterator),*> IntoIterator for Or<$($upper),*> {
                type IntoIter = Iterator<$($upper::IntoIter,)*>;
                type Item = Or<$($upper::Item,)*>;

                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    match self {
                        $(Self::$upper(item) => Iterator::$upper(item.into_iter()),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            impl<$($upper: iter::Iterator),*> iter::Iterator for Iterator<$($upper),*> {
                type Item = Or<$($upper::Item,)*>;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        $(Self::$upper(item) => Some(Or::$upper(item.next()?)),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            impl<$($upper: iter::DoubleEndedIterator),*> iter::DoubleEndedIterator for Iterator<$($upper),*> {
                #[inline]
                fn next_back(&mut self) -> Option<Self::Item> {
                    match self {
                        $(Self::$upper(item) => Some(Or::$upper(item.next_back()?)),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            impl<$($upper: iter::ExactSizeIterator),*> iter::ExactSizeIterator for Iterator<$($upper),*> {
                #[inline]
                fn len(&self) -> usize {
                    match self {
                        $(Self::$upper(item) => item.len(),)*
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            impl<$($upper: iter::FusedIterator),*> iter::FusedIterator for Iterator<$($upper),*> { }

            or!(@outer $($upper, $index, $lower),* @ ($($upper),*));
        }
    };
    (@outer $($upper: ident, $index: tt, $lower: ident),* @ $uppers: tt) => {
        $(or!(@inner $index, $upper, $lower $uppers);)*
    };
    (@inner $index: tt, $upper: ident, $lower: ident ($($uppers: ident),*)) => {
        impl<$($uppers),*> Or<$($uppers,)*> {
            #[inline]
            pub fn $lower(self) -> Option<$upper> {
                #[allow(irrefutable_let_patterns)]
                if let Self::$upper(item) = self {
                    Some(item)
                } else {
                    None
                }
            }
        }

        impl<$($uppers),*> At<$index> for ($($uppers,)*) {
            type Item = $upper;
            #[inline]
            fn at(self) -> Self::Item {
                self.$index
            }
        }

        impl<$($uppers),*> At<$index> for Or<$($uppers,)*> {
            type Item = Option<$upper>;
            #[inline]
            fn at(self) -> Self::Item {
                match self {
                    Self::$upper($lower) => Some($lower),
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        }

        impl<$($uppers),*> Is<$index> for Or<$($uppers,)*> {
            #[inline]
            fn is(&self) -> bool {
                match self {
                    Self::$upper(_) => true,
                    #[allow(unreachable_patterns)]
                    _ => false,
                }
            }
        }
    };
}

or!(0, Or0, or0);
or!(1, Or1, or1, 0, T0, t0);
or!(2, Or2, or2, 0, T0, t0, 1, T1, t1);
or!(3, Or3, or3, 0, T0, t0, 1, T1, t1, 2, T2, t2);
or!(4, Or4, or4, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3);
or!(5, Or5, or5, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4);
or!(6, Or6, or6, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5);
or!(7, Or7, or7, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6);
or!(
    8, Or8, or8, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7
);
or!(
    9, Or9, or9, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8
);
or!(
    10, Or10, or10, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9
);
or!(
    11, Or11, or11, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10
);
or!(
    12, Or12, or12, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11
);
or!(
    13, Or13, or13, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12
);
or!(
    14, Or14, or14, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13
);
or!(
    15, Or15, or15, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14
);
or!(
    16, Or16, or16, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15
);
or!(
    17, Or17, or17, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16
);
or!(
    18, Or18, or18, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17
);
or!(
    19, Or19, or19, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18
);
or!(
    20, Or20, or20, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19
);
or!(
    21, Or21, or21, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20
);
or!(
    22, Or22, or22, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21
);
or!(
    23, Or23, or23, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22
);
or!(
    24, Or24, or24, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23
);
or!(
    25, Or25, or25, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24
);
or!(
    26, Or26, or26, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25
);
or!(
    27, Or27, or27, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26
);
or!(
    28, Or28, or28, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26, 27, T27, t27
);
or!(
    29, Or29, or29, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26, 27, T27, t27,
    28, T28, t28
);
or!(
    30, Or30, or30, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26, 27, T27, t27,
    28, T28, t28, 29, T29, t29
);
or!(
    31, Or31, or31, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26, 27, T27, t27,
    28, T28, t28, 29, T29, t29, 30, T30, t30
);
or!(
    32, Or32, or32, 0, T0, t0, 1, T1, t1, 2, T2, t2, 3, T3, t3, 4, T4, t4, 5, T5, t5, 6, T6, t6, 7,
    T7, t7, 8, T8, t8, 9, T9, t9, 10, T10, t10, 11, T11, t11, 12, T12, t12, 13, T13, t13, 14, T14,
    t14, 15, T15, t15, 16, T16, t16, 17, T17, t17, 18, T18, t18, 19, T19, t19, 20, T20, t20, 21,
    T21, t21, 22, T22, t22, 23, T23, t23, 24, T24, t24, 25, T25, t25, 26, T26, t26, 27, T27, t27,
    28, T28, t28, 29, T29, t29, 30, T30, t30, 31, T31, t31
);
