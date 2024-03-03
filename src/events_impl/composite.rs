#[allow(unused_imports)]
use crate::events_impl::{
    adapt::action,
    core::{Bhv, Status},
    events::{Event, EventExt, EventKind},
};

/// A selector is a behavior node composed of a list of nodes that are run until one of them succeeds,
/// in which case the node also succeeds. If none of the nodes succeeds, this node fails.
pub struct Sel<C>(pub(crate) Box<[Box<dyn Bhv<Context=C>>]>);

/// A sequence is a behavior node composed of a list of nodes that are run until one of them fails,
/// in which case the node also fails. If none of the nodes fails, this node succeeds.
pub struct Seq<C>(pub(crate) Box<[Box<dyn Bhv<Context=C>>]>);

impl<C> Bhv for Sel<C> {
    type Context = C;
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        let et = event.event_type();

        for node in self
            .0
            .iter_mut()
            .take_while(|n| n.should_react_to(et)) {
            let s = node.react(event, ctx);

            if s != Status::Failure {
                return s;
            }
        }

        Status::Failure
    }
}

impl<C> Bhv for Seq<C> {
    type Context = C;
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        let et = event.event_type();
        let mut count = 0;

        for node in self.0
            .iter_mut()
            .take_while(|n| n.should_react_to(et)) {
            let s = node.react(event, ctx);

            if s != Status::Success {
                return s;
            }

            count += 1;
        }

        if count != self.0.len() {
            Status::Failure
        } else {
            Status::Success
        }
    }
}

/// A macro used to create a selector from a list of behaviors.
/// Selectors run every behavior until one of them succeeds.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let v_1digit = seq! {
///     cond(|v| *v < 10),
///     action(|v| println!("v only has one digit")),
/// };
///
/// let v_other = action(|_| println!("v has more than one digit"));
///
/// let tree = sel! { v_1digit, v_other };
///
/// tree.execute(UnitEventPump, &mut 9); // v only has one digit
/// ```
#[macro_export]
macro_rules! sel {
    () => {
        compile_error!("`sel` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::Sel(
            Box::new([$(Box::new($x)),+]),
        )
    };
}

/// A macro used to create a sequence from a list of behaviors.
/// Sequences run every behavior until one of them fails.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let tree = seq! {
///     cond(|v| *v > 10), // only run if v > 10
///     action(|_| println!("v is greater than 10")),
/// };
///
/// tree.execute(UnitEventPump, &mut 11);
/// ```
#[macro_export]
macro_rules! seq {
    () => {
        compile_error!("`seq` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::Seq(
            Box::new([$(Box::new($x)),+]),
        )
    };
}