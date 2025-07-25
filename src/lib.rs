#![no_std]
#![forbid(unsafe_code)]

use core::ops::{Deref, DerefMut};

pub trait At<const I: usize> {
    type Item;
    fn at(self) -> Self::Item;
}

pub trait Is {
    fn is(&self, index: usize) -> bool;
}

pub trait Count {
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

pub type Or0 = or0::Or;
pub mod or0 {
    use super::*;

    pub enum Or {}

    impl Count for () {
        const COUNT: usize = 0;
    }

    impl Count for Or {
        const COUNT: usize = 0;
    }
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
        pub type $alias<$($t,)*> = $module::Or<$($t,)*>;

        pub mod $module {
            use super::*;

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum Or<$($t,)*> { $($t($t)),* }

            impl<$($t,)*> Or<$($t,)*> {
                #[inline]
                pub fn into<T>(self) -> T where $($t: Into<T>),* {
                    match self {
                        $(Self::$t(item) => item.into(),)*
                    }
                }

                #[inline]
                pub const fn as_ref(&self) -> Or<$(&$t,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(item),)*
                    }
                }

                #[inline]
                pub fn as_mut(&mut self) -> Or<$(&mut $t,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(item),)*

                    }
                }

                #[inline]
                pub fn as_deref(&self) -> Or<$(&$t::Target,)*> where $($t: Deref),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.deref()),)*
                    }
                }

                #[inline]
                pub fn as_deref_mut(&mut self) -> Or<$(&mut $t::Target,)*> where $($t: DerefMut),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.deref_mut()),)*

                    }
                }
            }

            impl<$($t,)*> Or<$(&$t,)*> {
                #[inline]
                pub fn cloned(self) -> Or<$($t,)*> where $($t: Clone),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.clone()),)*
                    }
                }

                #[inline]
                pub fn copied(self) -> Or<$($t,)*> where $($t: Copy),* {
                    match self {
                        $(Self::$t(item) => Or::$t(*item),)*
                    }
                }
            }

            impl<$($t,)*> Or<$(&mut $t,)*> {
                #[inline]
                pub fn cloned(self) -> Or<$($t,)*> where $($t: Clone),* {
                    match self {
                        $(Self::$t(item) => Or::$t(item.clone()),)*
                    }
                }

                #[inline]
                pub fn copied(self) -> Or<$($t,)*> where $($t: Copy),* {
                    match self {
                        $(Self::$t(item) => Or::$t(*item),)*
                    }
                }
            }

            impl<T> Or<$($same_t,)*> {
                #[inline]
                pub fn into_inner(self) -> T {
                    match self {
                        $(Self::$t(item) => item,)*
                    }
                }

                #[inline]
                pub fn map<U, F: FnOnce(T) -> U>(self, map: F) -> Or<$($same_u,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(map(item)),)*
                    }
                }

                #[inline]
                pub fn map_with<S, U, F: FnOnce(S, T) -> U>(self, state:S, map: F) -> Or<$($same_u,)*> {
                    match self {
                        $(Self::$t(item) => Or::$t(map(state, item)),)*
                    }
                }
            }

            impl<T, $($t: AsRef<T>),*> AsRef<T> for Or<$($t,)*> {
                #[inline]
                fn as_ref(&self) -> &T {
                    match self {
                        $(Self::$t(item) => item.as_ref(),)*
                    }
                }
            }

            impl<T, $($t: AsMut<T>),*> AsMut<T> for Or<$($t,)*> {
                #[inline]
                fn as_mut(&mut self) -> &mut T {
                    match self {
                        $(Self::$t(item) => item.as_mut(),)*
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

                #[derive(Clone, Copy, Debug)]
                pub enum Iterator<$($t,)*> { $($t($t)),* }

                impl<$($t,)*> Or<$($t,)*> {
                    #[inline]
                    pub fn iter(&self) -> Iterator<$(<&$t as IntoIterator>::IntoIter,)*> where $(for<'a> &'a $t: IntoIterator,)* {
                        self.as_ref().into_iter()
                    }

                    #[inline]
                    pub fn iter_mut(&mut self) -> Iterator<$(<&mut $t as IntoIterator>::IntoIter,)*> where $(for<'a> &'a mut $t: IntoIterator,)* {
                        self.as_mut().into_iter()
                    }
                }

                impl<$($t: IntoIterator),*> IntoIterator for Or<$($t,)*> {
                    type IntoIter = Iterator<$($t::IntoIter,)*>;
                    type Item = Or<$($t::Item,)*>;

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
                }

                impl<$($t: DoubleEndedIterator),*> DoubleEndedIterator for Iterator<$($t,)*> {
                    #[inline]
                    fn next_back(&mut self) -> Option<Self::Item> {
                        match self {
                            $(Self::$t(item) => Some(Or::$t(item.next_back()?)),)*
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
                    #[inline]
                    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                        match self {
                            $(Self::$t(item) => item.extend(iter),)*
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

            or!(@outer [$($index, $t, $get, $is, $map),*] []);
        }
    };
    (@outer [] $old: tt) => {};
    (@outer
        [$index: tt, $t: ident, $get: ident, $is: ident, $map: ident $(, $new_index: tt, $new_t: ident, $new_get: ident, $new_is: ident, $new_map: ident)*]
        [$($old_t: ident),*]
    ) => {
        or!(@inner $index, $t, $get, $is, $map [$($old_t, $old_t, same,)* $t, U, with $(, $new_t, $new_t, same)*]);
        or!(@outer [$($new_index, $new_t, $new_get, $new_is, $new_map),*] [$($old_t,)* $t]);
    };
    (@inner $index: tt, $t: ident, $get: ident, $is: ident, $map: ident [$($ts: ident, $map_t: ident, $map_f: ident),*]) => {
        impl<$($ts),*> Or<$($ts,)*> {
            #[inline]
            pub fn $get(self) -> Option<$t> {
                match self {
                    Self::$t(item) => Some(item),
                    #[allow(unreachable_patterns)]
                    _ => None
                }
            }

            #[inline]
            pub fn $is(&self) -> bool {
                match self {
                    Self::$t(_) => true,
                    #[allow(unreachable_patterns)]
                    _ => false
                }
            }

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

            fn complete(self) -> Self::Result {
                self.0.complete()
            }

            fn consume(self, item: $type) -> Self {
                Self::new(self.0.consume(Or::$type(item)))
            }

            fn consume_iter<I>(self, iter: I) -> Self
            where
                I: IntoIterator<Item = $type>,
            {
                Self::new(self.0.consume_iter(iter.into_iter().map(Or::$type)))
            }

            fn full(&self) -> bool {
                self.0.full()
            }
        }
    };
}

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
