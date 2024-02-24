use std::marker::PhantomData;

#[allow(unused_imports)]
use crate::events_impl::{
    core::{Bhv, Status},
    decor::*,
    events::{Event, EventExt, EventType, EventTypeExt},
};

/// Helper methods to build a tree from given nodes.
/// See nodes returned by the functions for specific examples and usage.
pub trait BhvExt: Bhv + Sized {
    /// Return a node that inverts the result of this node.
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let always_true = action(|_| {});
    /// always_true.clone().execute(UnitEventPump, &mut 100);
    ///
    /// let inverse = always_true.inv();
    /// inverse.execute(UnitEventPump, &mut 100);
    /// ```
    #[inline]
    fn inv(self) -> Inv<Self> {
        Inv(self)
    }

    /// Return a node that runs this node and returns [`Status::Success`] when done.
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let always_false = cond(|_| false);
    /// always_false.clone().execute(UnitEventPump, &mut ());
    ///
    /// let always_true = always_false.pass();
    /// always_true.execute(UnitEventPump, &mut ());
    /// ```
    #[inline]
    fn pass(self) -> Pass<Self> {
        Pass(self)
    }

    /// Return a node that runs this node and returns [`Status::Failure`] when done.
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let always_true = cond(|_| true);
    /// always_true.clone().execute(UnitEventPump, &mut ());
    ///
    /// let always_false = always_true.fail();
    /// always_false.execute(UnitEventPump, &mut ());
    /// ```
    #[inline]
    fn fail(self) -> Fail<Self> {
        Fail(self)
    }

    /// Return a node that runs this node the given number of times
    /// and returns the last exit status when done.
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
    /// tree.execute(UnitEventPump, &mut v);
    ///
    /// assert_eq!(v, 7);
    /// ```
    #[inline]
    fn repeat(self, count: u32) -> Repeat<Self> {
        Repeat {
            bhv: self,
            count,
            current: 0,
        }
    }

    /// Return a node that runs this node until it returns [`Status::Success`].
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let print = action(|v| println!("Value is {}", *v));
    /// let inc = action(|v| *v += 1);
    /// let cond = cond(|v| *v == 5);
    ///
    /// let tree = seq! { print, inc, cond }.repeat_until_pass();
    ///
    /// let mut ctx = 0;
    /// tree.execute(UnitEventPump, &mut ctx);
    ///
    /// assert_eq!(ctx, 5);
    /// ```
    #[inline]
    fn repeat_until_pass(self) -> RepeatUntilPass<Self> {
        RepeatUntilPass(self)
    }

    /// Return a node that runs this node until it returns [`Status::Failure`].
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let print = action(|v| println!("Value is {}", *v));
    /// let inc = action(|v| *v += 1);
    /// let cond = cond(|v| *v < 5);
    ///
    /// let tree = seq! { print, inc, cond }.repeat_until_fail();
    ///
    /// let mut ctx = 0;
    /// tree.execute(UnitEventPump, &mut ctx);
    ///
    /// assert_eq!(ctx, 5);
    /// ```
    #[inline]
    fn repeat_until_fail(self) -> RepeatUntilFail<Self> {
        RepeatUntilFail(self)
    }

    /// Return a node that runs this node only after the given event type is triggered.
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// struct Step;
    /// struct Exit;
    ///
    /// impl EventType for Step {}
    /// impl EventType for Exit {}
    ///
    /// let step = action(|_| println!("Running"));
    /// let on_exit = action(|_| println!("Exiting")).wait_for::<Exit>();
    ///
    /// let events: [&dyn Event; 3] = [
    ///     &Step,
    ///     &Step,
    ///     &Exit,
    /// ];
    ///
    /// let tree = seq! {
    ///     step,
    ///     on_exit,
    /// };
    ///
    /// tree.execute(events, &mut ());
    /// ```
    #[inline]
    fn wait_for<E: EventType>(self) -> WaitFor<Self, E> {
        WaitFor {
            bhv: self,
            kind: E::static_event_type(),
            _tag: PhantomData,
        }
    }

    /// Execute the node until it does not return [`Status::Running`] anymore. Events are consumed from `events`.
    fn execute<'a>(
        mut self,
        events: impl IntoIterator<Item=&'a dyn Event>,
        ctx: &mut Self::Context,
    ) {
        for event in events {
            if !self.should_react_to(event.event_type()) {
                break;
            } else {
                match self.react(event, ctx) {
                    Status::Running => continue,
                    _ => return,
                }
            }
        }
    }
}

impl<B> BhvExt for B where B: Bhv + Sized {}
