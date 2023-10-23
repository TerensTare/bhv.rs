use crate::bhv::{Bhv, Status};

// TODO:
// Repeat should be ran every time the node is finished, not every time `update` is called
// same thing for RepeatUntil

/// A decorator that runs the given node until it's done and inverts
/// the result.
///
/// More specifically, it converts [`Status::Success`] to [`Status::Failure`]
/// and [`Status::Failure`] to [`Status::Success`].
#[derive(Clone)]
pub struct Inv<B: Bhv>(pub B);

/// A decorator that runs the given node until it's done and then returns [`Status::Success`].
#[derive(Clone)]
pub struct Pass<B: Bhv>(pub B);

/// A decorator that runs the given node until it's done and then returns [`Status::Failure`].
#[derive(Clone)]
pub struct Fail<B: Bhv>(pub B);

/// A decorator that runs the given node a certain number of times and returns its status.
#[derive(Clone)]
pub struct Repeat<B: Bhv> {
    pub bhv: B,
    pub count: u32,
    pub(crate) current: u32,
}

/// A decorator that runs the given node as long as it's predicate returns `true`
/// and returns the status of the node.
#[derive(Clone)]
pub struct RepeatUntil<B, C>
where
    B: Bhv,
    C: Fn(&B::Context) -> bool,
{
    pub bhv: B,
    pub cond: C,
    pub(crate) checked_cond: bool,
}

/// Repeat a behavior the given number of times.
#[inline]
pub fn repeat<B: Bhv>(bhv: B, count: u32) -> Repeat<B> {
    Repeat {
        bhv,
        count,
        current: 1,
    }
}

/// Repeat a behavior as long as the given predicate is true.
#[inline]
pub fn repeat_until<B, C>(bhv: B, cond: C) -> RepeatUntil<B, C>
where
    B: Bhv,
    C: Fn(&B::Context) -> bool,
{
    RepeatUntil {
        bhv,
        cond,
        checked_cond: false,
    }
}

impl<B: Bhv> Bhv for Inv<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        match self.0.update(ctx) {
            Status::Running => Status::Running,
            Status::Failure => Status::Success,
            Status::Success => Status::Failure,
        }
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<B: Bhv> Bhv for Pass<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        match self.0.update(ctx) {
            Status::Failure => Status::Success,
            s => s,
        }
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<B: Bhv> Bhv for Fail<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        match self.0.update(ctx) {
            Status::Success => Status::Failure,
            s => s,
        }
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<B: Bhv> Bhv for Repeat<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        if self.current >= self.count {
            self.bhv.update(ctx)
        } else {
            match self.bhv.update(ctx) {
                Status::Running => {}
                _ => self.current += 1,
            };

            Status::Running
        }
    }

    fn reset(&mut self, _status: Status) {
        self.bhv.reset(_status);
        self.current = 1;
    }
}

impl<B, C> Bhv for RepeatUntil<B, C>
where
    B: Bhv,
    C: Fn(&B::Context) -> bool,
{
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        let s = self.bhv.update(ctx);
        if s != Status::Running {
            self.checked_cond = false;
            self.bhv.reset(s);
        }

        if !self.checked_cond {
            if (self.cond)(ctx) {
                return Status::Success;
            } else {
                self.checked_cond = true;
            }
        }

        Status::Running
    }

    fn reset(&mut self, _status: Status) {
        self.bhv.reset(_status)
    }
}
