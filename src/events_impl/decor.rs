use std::marker::PhantomData;

use crate::events_impl::{
    core::{Bhv, Status},
    events::{Event, EventKind, EventType},
};

/// A decorator that runs the given node until it's done and inverts
/// the result.
///
/// More specifically, it converts [`Status::Success`] to [`Status::Failure`]
/// and [`Status::Failure`] to [`Status::Success`].
#[derive(Debug, Clone)]
pub struct Inv<B>(pub(crate) B)
    where B: Bhv;

/// A decorator that runs the given node until it's done and then returns [`Status::Success`].
#[derive(Debug, Clone)]
pub struct Pass<B>(pub(crate) B)
    where B: Bhv;

/// A decorator that runs the given node until it's done and then returns [`Status::Failure`].
#[derive(Debug, Clone)]
pub struct Fail<B>(pub(crate) B)
    where B: Bhv;

/// A decorator that runs the given node a certain number of times and returns its status.
#[derive(Clone)]
pub struct Repeat<B: Bhv> {
    pub(crate) bhv: B,
    pub(crate) count: u32,
    pub(crate) current: u32,
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

/// A decorator that runs a node only when a certain event type is triggered.
#[derive(Clone)]
pub struct WaitFor<B: Bhv, E: EventType> {
    pub(crate) bhv: B,
    pub(crate) kind: EventKind,
    pub(crate) _tag: PhantomData<E>,
}

impl<B: Bhv> Bhv for Inv<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.0.react(event, ctx) {
            Status::Running => Status::Running,
            Status::Success => Status::Failure,
            Status::Failure => Status::Success,
        }
    }
}

impl<B: Bhv> Bhv for Pass<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.0.react(event, ctx) {
            Status::Running => Status::Running,
            _ => Status::Success,
        }
    }
}

impl<B: Bhv> Bhv for Fail<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.0.react(event, ctx) {
            Status::Running => Status::Running,
            _ => Status::Failure,
        }
    }
}

impl<B: Bhv> Bhv for Repeat<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.current < self.count && self.bhv.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.bhv.react(event, ctx) {
            Status::Running => Status::Running,
            _ => {
                self.current += 1;
                Status::Running
            }
        }
    }
}

impl<B: Bhv> Bhv for RepeatUntilPass<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.0.react(event, ctx) {
            Status::Success => Status::Success,
            _ => Status::Running,
        }
    }
}

impl<B: Bhv> Bhv for RepeatUntilFail<B> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        match self.0.react(event, ctx) {
            Status::Failure => Status::Failure,
            _ => Status::Running,
        }
    }
}

impl<B: Bhv, E: EventType> Bhv for WaitFor<B, E> {
    type Context = B::Context;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.kind == kind
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        self.bhv.react(event, ctx)
    }
}

impl<B: Bhv> Repeat<B> {
    #[inline]
    pub fn reset(&mut self) {
        self.current = 0;
    }
}