use crate::events_impl::events::{Event, EventKind};

/// An enum type representing the outcome of calling [`Bhv::update`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    /// The behavior is still running.
    Running,
    /// The behavior was completed successfully.
    Success,
    /// The behavior failed to complete.
    Failure,
}

/// A trait used to denote that the implementing type can be used as
/// a behavior tree node.
pub trait Bhv {
    /// Node-specific data passed every time it is run.
    type Context;
    /// Check whether the node should run in response to events of the given kind.
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool { true }
    /// Run this node in response to an outer event.
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status;
}