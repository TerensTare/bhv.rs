use std::marker::PhantomData;

use crate::{Bhv, Status};

/// The type of the result of [`cond`].
#[derive(Clone)]
pub struct Cond<Ctx, C>(C, PhantomData<Ctx>)
    where
        C: Fn(&Ctx) -> bool;

/// The type of the result of [`action`].
#[derive(Clone)]
pub struct Action<Ctx, A>(A, PhantomData<Ctx>)
    where
        A: FnMut(&mut Ctx);

/// Adapt a predicate into a behavior, returning [`Status::Success`] if
/// the predicate returns `true` and [`Status::Failure`] otherwise.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let check = cond(|v| *v >= 10);
///
/// assert_eq!(check.clone().execute(&mut 10), true);
/// assert_eq!(check.execute(&mut 5), false);
/// ```
#[inline]
pub fn cond<Ctx, C>(c: C) -> Cond<Ctx, C>
    where
        C: Fn(&Ctx) -> bool,
{
    Cond(c, PhantomData)
}

/// Adapt a function that returns nothing into a behavior, returning [`Status::Success`]
/// on every call to [`Bhv::update`].
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let print = action(|v| println!("Value is {}", *v));
/// assert_eq!(print.execute(&mut 42), true);
/// ```
#[inline]
pub fn action<Ctx, A>(a: A) -> Action<Ctx, A>
    where
        A: FnMut(&mut Ctx),
{
    Action(a, PhantomData)
}

impl<Ctx, C> Bhv for Cond<Ctx, C>
    where
        C: Fn(&Ctx) -> bool,
{
    type Context = Ctx;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        if self.0(ctx) {
            Status::Success
        } else {
            Status::Failure
        }
    }
}

impl<Ctx, A> Bhv for Action<Ctx, A>
    where
        A: FnMut(&mut Ctx),
{
    type Context = Ctx;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        self.0(ctx);
        Status::Success
    }
}
