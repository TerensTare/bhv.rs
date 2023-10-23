use crate::bhv::{Bhv, Status};
use std::marker::PhantomData;

/// Adapt a predicate into a behavior, returning [`Status::Success`] if
/// the predicate returns `true` and [`Status::Failure`] otherwise.
#[derive(Clone)]
pub struct Cond<Ctx, C>(C, PhantomData<Ctx>)
where
    C: Fn(&Ctx) -> bool;

/// Adapt a function that returns nothing into a behavior, returning [`Status::Success`]
/// on every call to [`Bhv::update`].
#[derive(Clone)]
pub struct Action<Ctx, A>(A, PhantomData<Ctx>)
where
    A: FnMut(&mut Ctx);

/// Wrap a predicate into a behavior node.
#[inline]
pub fn cond<Ctx, C>(c: C) -> Cond<Ctx, C>
where
    C: Fn(&Ctx) -> bool,
{
    Cond(c, PhantomData)
}

/// Wrap a function into a behavior node.
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
        if (self.0)(ctx) {
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
        (self.0)(ctx);
        Status::Success
    }
}
