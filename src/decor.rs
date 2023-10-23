use crate::core::{Bhv, Status};

/// A decorator that runs the given node until it's done and inverts
/// the result.
///
/// More specifically, it converts [`Status::Success`] to [`Status::Failure`]
/// and [`Status::Failure`] to [`Status::Success`].
/// 
/// # Example
/// 
/// ```
/// use bhv::*;
/// 
/// let always_true = action(|_| {});
/// assert!(always_true.clone().execute(&mut 100) == true);
/// 
/// let inverse = always_true.inv();
/// assert!(inverse.execute(&mut 100) == false);
/// ```
#[derive(Clone)]
pub struct Inv<B: Bhv>(pub B);

/// A decorator that runs the given node until it's done and then returns [`Status::Success`].
/// 
/// # Example
/// 
/// ```
/// use bhv::*;
/// 
/// let always_false = cond(|_| false);
/// assert!(always_false.clone().execute(&mut 100) == false);
/// 
/// let always_true = always_false.pass();
/// assert!(always_true.execute(&mut 100) == true);
/// ```
#[derive(Clone)]
pub struct Pass<B: Bhv>(pub B);

/// A decorator that runs the given node until it's done and then returns [`Status::Failure`].
/// 
/// # Example
/// 
/// ```
/// use bhv::*;
/// 
/// let always_true = cond(|_| true);
/// assert!(always_true.clone().execute(&mut 100) == true);
/// 
/// let always_false = always_true.fail();
/// assert!(always_false.execute(&mut 100) == false);
/// ```
#[derive(Clone)]
pub struct Fail<B: Bhv>(pub B);

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

/// Repeat a behavior the given number of times.
/// 
/// # Example
/// 
/// ```
/// use bhv::*;
/// 
/// let mut v = 10;
/// 
/// let dec = action(|v| *v -= 1);
/// let tree = dec.repeat(3);
/// 
/// tree.execute(&mut v);
/// 
/// assert_eq!(v, 7);
/// ```
#[inline]
pub fn repeat<B: Bhv>(bhv: B, count: u32) -> Repeat<B> {
    Repeat {
        bhv,
        count,
        current: 1,
    }
}

/// Repeat a behavior as long as the given predicate is true.
/// 
/// # Example
/// 
/// ```
/// use bhv::*;
/// 
/// let mut v = 10;
/// 
/// let dec = action(|v| *v -= 1);
/// let tree = dec.repeat_until(|v| *v < 8);
/// 
/// tree.execute(&mut v);
/// 
/// assert_eq!(v, 7);
/// ```
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
