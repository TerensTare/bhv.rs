use crate::{
    core::Bhv,
    decor::{Fail, Inv, Pass, Repeat, RepeatUntil},
};

#[allow(unused_imports)]
use crate::core::Status;

/// Helper methods to build a tree from given nodes.
/// See nodes returned by the functions for specific examples and usage.
pub trait BhvExt: Bhv + Sized {
    /// Return a node that inverts the result of this node.
    #[inline]
    fn inv(self) -> Inv<Self> {
        Inv(self)
    }

    /// Return a node that runs this node and returns [`Status::Success`] when done.
    #[inline]
    fn pass(self) -> Pass<Self> {
        Pass(self)
    }

    /// Return a node that runs this node and returns [`Status::Failure`] when done.
    #[inline]
    fn fail(self) -> Fail<Self> {
        Fail(self)
    }

    /// Return a node that runs this node the given number of times
    /// and returns the last exit status when done.
    #[inline]
    fn repeat(self, count: u32) -> Repeat<Self> {
        Repeat {
            bhv: self,
            count,
            current: 1,
        }
    }

    /// Return a node that runs this node then checks the passed condition
    /// until the condition returns true.
    /// The node then returns the last exit status when done.
    #[inline]
    fn repeat_until<C>(self, cond: C) -> RepeatUntil<Self, C>
    where
        C: Fn(&Self::Context) -> bool,
    {
        RepeatUntil {
            bhv: self,
            cond,
            checked_cond: false,
        }
    }
}

impl<B> BhvExt for B where B: Bhv + Sized {}
