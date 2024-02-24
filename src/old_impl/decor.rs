use crate::{Bhv, Status};

/// A decorator that runs the given node until it's done and inverts
/// the result.
///
/// More specifically, it converts [`Status::Success`] to [`Status::Failure`]
/// and [`Status::Failure`] to [`Status::Success`].
#[derive(Clone)]
pub struct Inv<B: Bhv>(pub(crate) B);

/// A decorator that runs the given node until it's done and then returns [`Status::Success`].
#[derive(Clone)]
pub struct Pass<B: Bhv>(pub(crate) B);

/// A decorator that runs the given node until it's done and then returns [`Status::Failure`].
#[derive(Clone)]
pub struct Fail<B: Bhv>(pub(crate) B);

/// A decorator that runs the given node a certain number of times and returns its status.
#[derive(Clone)]
pub struct Repeat<B: Bhv> {
    pub(crate) bhv: B,
    pub(crate) count: u32,
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
    pub(crate) bhv: B,
    pub(crate) cond: C,
    pub(crate) checked_cond: bool,
}

/// A decorator that runs the given node until it returns [`Status::Success`].
///
/// It returns [`Status::Running`] until the node returns [`Status::Success`], in which case it is propagated.
#[derive(Clone)]
pub struct RepeatUntilPass<B: Bhv>(pub(crate) B);

/// A decorator that runs the given node until it returns [`Status::Failure`].
///
/// It returns [`Status::Running`] until the node returns [`Status::Failure`], in which case it is propagated.
#[derive(Clone)]
pub struct RepeatUntilFail<B: Bhv>(pub(crate) B);

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
                s => {
                    self.bhv.reset(s);
                    self.current += 1;
                }
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

impl<B: Bhv> Bhv for RepeatUntilPass<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        match self.0.update(ctx) {
            Status::Failure => {
                self.0.reset(Status::Failure);
                Status::Running
            }
            s => s,
        }
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<B: Bhv> Bhv for RepeatUntilFail<B> {
    type Context = B::Context;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        match self.0.update(ctx) {
            Status::Success => {
                self.0.reset(Status::Success);
                Status::Running
            }
            s => s,
        }
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}
