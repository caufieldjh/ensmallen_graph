//! # Method Caller
//! Aka Map with benefits
//!
//! This is a way to be able to call different functions that acts on the same state
//! during a parallel iteration. It's conceptually similar to a parallel scan.
//!
//! ```rust
//!
//!     // implemnet the sturct with context usize
//!    impl_struct_func!(Funzioni usize);
//!
//!    // Implement the methods we want to be abel to call
//!    // these must use the following primitives to access the context:
//!    // self.get_mutable_write, self.get_mutable_read, self.get_immutable
//!    // get_mutable_read and get_immutable are conceptually identical but
//!    // get_immutable is ~40 times faster, but can be called IFF you have a
//!    // guarantee that no-one is ever writing to the context.
//!    impl Funzioni {
//!        pub fn parse(&mut self, value: usize) -> u8 {
//!            let c = self.get_mutable_write();
//!
//!            *c = c.wrapping_add(1);
//!            (value.wrapping_add(*c)) as _
//!        }
//!
//!        pub fn check(&mut self, value: usize) -> u8 {
//!            let c = *self.get_immutable();
//!            (value.wrapping_add(c)) as _
//!        }
//!    }
//!
//!    fn main() {
//!        for x in vec![true, false] {
//!
//!            let mut f = Funzioni::new(100);
//!
//!            let method_f = if x {
//!                Funzioni::parse
//!            } else {
//!                Funzioni::check
//!            };
//!
//!
//!            let res: u8 = (0..100_000_000)
//!                .into_par_iter()
//!                .map(|x| x as usize)
//!                .method_caller(method_f, &mut f)
//!                .sum();
//!
//!            println!("{:?}", res);
//!            println!("{:?}", f.context);
//!            println!("");
//!        }
//!    }
//! ```

use super::*;
use rayon::iter::plumbing::*;
use rayon::iter::Map;
use rayon::prelude::*;
use std::iter::Iterator;

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct MethodCaller<T, R, S, I: ParallelIterator<Item = T>> {
    base: I,
    method: fn(&mut S, T) -> R,
    context: usize,
}

impl<T, R, S, I: ParallelIterator<Item = T>> MethodCaller<T, R, S, I> {
    fn new(base: I, method: fn(&mut S, T) -> R, context: usize) -> MethodCaller<T, R, S, I> {
        MethodCaller {
            base,
            method,
            context,
        }
    }
}

impl<T, R: Send, S, I: ParallelIterator<Item = T>> ParallelIterator for MethodCaller<T, R, S, I> {
    type Item = R;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let consumer1 = MethodCallerConsumer::new(consumer, self.method, self.context);
        self.base.drive_unindexed(consumer1)
    }

    fn opt_len(&self) -> Option<usize> {
        self.base.opt_len()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct MethodCallerConsumer<T, R, S, C> {
    base: C,
    method: fn(&mut S, T) -> R,
    context: usize,
}

impl<T, R, S, C> MethodCallerConsumer<T, R, S, C> {
    fn new(
        base: C,
        method: fn(&mut S, T) -> R,
        context: usize,
    ) -> MethodCallerConsumer<T, R, S, C> {
        MethodCallerConsumer {
            base,
            method,
            context,
        }
    }
}

impl<T, R, S, C> Consumer<T> for MethodCallerConsumer<T, R, S, C>
where
    C: Consumer<R>,
{
    type Folder = MethodCallerFolder<T, R, S, C::Folder>;
    type Reducer = C::Reducer;
    type Result = C::Result;

    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer) {
        let (left, right, reducer) = self.base.split_at(index);
        (
            MethodCallerConsumer::new(left, self.method.clone(), self.context.clone()),
            MethodCallerConsumer::new(right, self.method.clone(), self.context.clone()),
            reducer,
        )
    }

    fn into_folder(self) -> Self::Folder {
        MethodCallerFolder {
            base: self.base.into_folder(),
            method: self.method.clone(),
            context: self.context.clone(),
        }
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

impl<T, R, S, C> UnindexedConsumer<T> for MethodCallerConsumer<T, R, S, C>
where
    C: UnindexedConsumer<R>,
{
    fn split_off_left(&self) -> Self {
        MethodCallerConsumer::new(self.base.split_off_left(), self.method, self.context)
    }

    fn to_reducer(&self) -> Self::Reducer {
        self.base.to_reducer()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct MethodCallerFolder<T, R, S, C> {
    base: C,
    method: fn(&mut S, T) -> R,
    context: usize,
}

impl<T, R, S, C> Folder<T> for MethodCallerFolder<T, R, S, C>
where
    C: Folder<R>,
{
    type Result = C::Result;

    fn consume(self, item: T) -> Self {
        let context = unsafe { &mut *(self.context as *mut S) };
        let mapped_item = (self.method)(context, item);
        MethodCallerFolder {
            base: self.base.consume(mapped_item),
            method: self.method,
            context: self.context,
        }
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let context = self.context;
        let method = self.method;

        self.base = self.base.consume_iter(iter.into_iter().map(|item| {
            let call_self = unsafe { &mut *(context as *mut S) };
            (method)(call_self, item)
        }));
        self
    }

    fn complete(self) -> C::Result {
        self.base.complete()
    }

    fn full(&self) -> bool {
        self.base.full()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait OrOps<T, R, S> {
    fn method_caller(
        self,
        method: fn(&mut S, T) -> R,
        context: &mut S,
    ) -> MethodCaller<T, R, S, Self>
    where
        Self: ParallelIterator<Item = T>,
    {
        MethodCaller::new(self, method, context as *const S as usize)
    }
}

impl<T, R, S, J: ?Sized> OrOps<T, R, S> for J where J: ParallelIterator<Item = T> {}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! impl_struct_func {
    ($struct_name:ident $context_type:ty) => {
        use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

        pub(crate) struct $struct_name {
            context: $context_type,
            lock: RwLock<()>,
        }

        impl $struct_name {
            pub fn new(context: $context_type) -> $struct_name {
                $struct_name {
                    context: context,
                    lock: RwLock::new(()),
                }
            }

            pub fn into_inner(self) -> $context_type {
                self.context
            }

            #[inline]
            fn get_immutable(&self) -> &$context_type {
                &self.context
            }

            #[inline]
            fn get_mutable_read(&mut self) -> (&mut $context_type, RwLockReadGuard<'_, ()>) {
                (&mut self.context, self.lock.read().unwrap())
            }

            #[inline]
            fn get_mutable_write(&mut self) -> (&mut $context_type, RwLockWriteGuard<'_, ()>) {
                (&mut self.context, self.lock.write().unwrap())
            }
        }
    };
}
