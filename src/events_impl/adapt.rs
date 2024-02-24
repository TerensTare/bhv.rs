use std::marker::PhantomData;

#[allow(unused_imports)]
use crate::events_impl::{
    core::{Bhv, Status},
    events::{Event, UnitEventPump},
};

/// The type of the result of [`action`].
#[derive(Clone)]
pub struct Action<A, C>(A, PhantomData<C>)
    where
        A: FnMut(&mut C);

/// The type of the result of [`cond`].
#[derive(Clone)]
pub struct Cond<P, C>(P, PhantomData<C>)
    where
        P: Fn(&C) -> bool;

impl<A, C> Bhv for Action<A, C>
    where A: FnMut(&mut C) {
    type Context = C;
    #[inline]
    fn react(&mut self, _event: &dyn Event, ctx: &mut Self::Context) -> Status {
        self.0(ctx);
        Status::Success
    }
}

impl<P, C> Bhv for Cond<P, C>
    where P: Fn(&C) -> bool {
    type Context = C;
    #[inline]
    fn react(&mut self, _event: &dyn Event, ctx: &mut Self::Context) -> Status {
        if self.0(ctx) {
            Status::Success
        } else {
            Status::Failure
        }
    }
}

/// Adapt a function that returns nothing into a behavior, returning [`Status::Success`]
/// on every call to [`Bhv::react`].
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let print = action(|v| println!("Value is {}", *v));
/// print.execute(UnitEventPump, &mut 42);
/// ```
#[inline]
pub fn action<A, C>(a: A) -> Action<A, C>
    where A: FnMut(&mut C) { Action(a, PhantomData) }

/// Adapt a predicate into a behavior, returning [`Status::Success`] if
/// the predicate returns `true` and [`Status::Failure`] otherwise.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let check = sel! {
///     seq! {
///         cond(|v| *v >= 10),
///         action(|_| println!("v >= 10")),
///     },
///     action(|_| println!("v < 10")),
/// };
///
/// check.execute(UnitEventPump, &mut 10);
/// ```
#[inline]
pub fn cond<P, C>(p: P) -> Cond<P, C>
    where P: Fn(&C) -> bool { Cond(p, PhantomData) }