#![no_std]

macro_rules! or {
    ($n: expr, $or: ident, $module: ident $(, $types: ident, $upper: ident, $lower: ident)*) => {
        pub type $or<$($types),*> = $module::Or<$($types),*>;

        pub mod $module {
            #[allow(unused_imports)]
            use core::{iter, hint::unreachable_unchecked, ops::{Deref, DerefMut}};

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Or<$($types),*> { $($upper($types)),* }
            #[derive(Clone, Copy, Debug)]
            pub enum Iterator<$($types),*> { $($upper($types)),* }

            impl<$($types),*> Or<$($types),*> {
                #[inline]
                pub fn into<T>(self) -> T where $($types: Into<T>),* {
                    match self {
                        $(Self::$upper(item) => item.into(),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }

                #[inline]
                pub const fn as_ref(&self) -> Or<$(&$types,)*> {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }

                #[inline]
                pub fn as_mut(&mut self) -> Or<$(&mut $types,)*> {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }

                    }
                }

                #[inline]
                pub fn as_deref(&self) -> Or<$(&$types::Target,)*> where $($types: Deref),* {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item.deref()),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }

                #[inline]
                pub fn as_deref_mut(&mut self) -> Or<$(&mut $types::Target,)*> where $($types: DerefMut),* {
                    match self {
                        $(Self::$upper(item) => Or::$upper(item.deref_mut()),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }

                    }
                }

                $(
                    #[inline]
                    pub fn $lower(self) -> Option<$types> {
                        #[allow(irrefutable_let_patterns)]
                        if let Self::$upper(item) = self {
                            Some(item)
                        } else {
                            None
                        }
                    }
                )*
            }

            impl<$($types: IntoIterator),*> IntoIterator for Or<$($types),*> {
                type IntoIter = Iterator<$($types::IntoIter,)*>;
                type Item = Or<$($types::Item,)*>;

                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    match self {
                        $(Self::$types(item) => Iterator::$types(item.into_iter()),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }
            }

            impl<$($types: iter::Iterator),*> iter::Iterator for Iterator<$($types),*> {
                type Item = Or<$($types::Item,)*>;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        $(Self::$types(item) => Some(Or::$types(item.next()?)),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }
            }

            impl<$($types: iter::DoubleEndedIterator),*> iter::DoubleEndedIterator for Iterator<$($types),*> {
                #[inline]
                fn next_back(&mut self) -> Option<Self::Item> {
                    match self {
                        $(Self::$types(item) => Some(Or::$types(item.next_back()?)),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }
            }

            impl<$($types: iter::ExactSizeIterator),*> iter::ExactSizeIterator for Iterator<$($types),*> {
                #[inline]
                fn len(&self) -> usize {
                    match self {
                        $(Self::$types(item) => item.len(),)*
                        #[allow(unreachable_patterns)]
                        _ => unsafe { unreachable_unchecked() }
                    }
                }
            }

            impl<$($types: iter::FusedIterator),*> iter::FusedIterator for Iterator<$($types),*> { }
        }
    };
}

or!(0, Or0, or0);
or!(1, Or1, or1, T0, T0, t0);
or!(2, Or2, or2, T0, T0, t0, T1, T1, t1);
or!(3, Or3, or3, T0, T0, t0, T1, T1, t1, T2, T2, t2);
or!(4, Or4, or4, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3);
or!(5, Or5, or5, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4);
or!(6, Or6, or6, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5);
or!(
    7, Or7, or7, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6, t6
);
or!(
    8, Or8, or8, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7
);
or!(
    9, Or9, or9, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8
);
or!(
    10, Or10, or10, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9
);
or!(
    11, Or11, or11, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10
);
or!(
    12, Or12, or12, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10, T11, T11, t11
);
or!(
    13, Or13, or13, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10, T11, T11, t11, T12, T12, t12
);
or!(
    14, Or14, or14, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10, T11, T11, t11, T12, T12, t12, T13, T13,
    t13
);
or!(
    15, Or15, or15, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10, T11, T11, t11, T12, T12, t12, T13, T13,
    t13, T14, T14, t14
);
or!(
    16, Or16, or16, T0, T0, t0, T1, T1, t1, T2, T2, t2, T3, T3, t3, T4, T4, t4, T5, T5, t5, T6, T6,
    t6, T7, T7, t7, T8, T8, t8, T9, T9, t9, T10, T10, t10, T11, T11, t11, T12, T12, t12, T13, T13,
    t13, T14, T14, t14, T15, T15, t15
);
