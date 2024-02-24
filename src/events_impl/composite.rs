use std::marker::PhantomData;

#[allow(unused_imports)]
use crate::events_impl::{
    adapt::action,
    core::{Bhv, Status},
    events::{Event, EventExt, EventKind},
};

// TODO:
// BhvExt::then specialization for Seq
// same for Sel

pub(crate) trait StatusPolicy {
    const STATUS: Status;
}

pub(crate) struct SelPolicy;

pub(crate) struct SeqPolicy;

pub(crate) struct List<C, Policy>
    where
        Policy: StatusPolicy,
{
    nodes: Vec<Box<dyn Bhv<Context=C>>>,
    _tag: PhantomData<Policy>,
}

/// A selector is a behavior node composed of a list of nodes that are run until one of them succeeds,
/// in which case the node also succeeds. If none of the nodes succeeds, this node fails.
pub struct Sel<C>(pub(crate) List<C, SelPolicy>);

/// A sequence is a behavior node composed of a list of nodes that are run until one of them fails,
/// in which case the node also fails. If none of the nodes fails, this node succeeds.
pub struct Seq<C>(pub(crate) List<C, SeqPolicy>);

impl<C> Sel<C> {
    #[inline]
    pub fn with_nodes(nodes: Vec<Box<dyn Bhv<Context=C>>>) -> Self {
        Self(List {
            nodes,
            _tag: PhantomData,
        })
    }
}

impl<C> Seq<C> {
    #[inline]
    pub fn with_nodes(nodes: Vec<Box<dyn Bhv<Context=C>>>) -> Self {
        Self(List {
            nodes,
            _tag: PhantomData,
        })
    }
}

impl<C, Policy> Bhv for List<C, Policy>
    where
        Policy: StatusPolicy,
{
    type Context = C;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.nodes[0].should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        for node in self
            .nodes
            .iter_mut()
            .take_while(|n| n.should_react_to(event.event_type())) {
            let s = node.react(event, ctx);

            if s == Policy::STATUS {
                continue;
            } else {
                return s;
            }
        }

        Policy::STATUS
    }
}

impl<C> Bhv for Sel<C> {
    type Context = C;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        self.0.react(event, ctx)
    }
}

impl<C> Bhv for Seq<C> {
    type Context = C;
    #[inline]
    fn should_react_to(&self, kind: EventKind) -> bool {
        self.0.should_react_to(kind)
    }
    #[inline]
    fn react(&mut self, event: &dyn Event, ctx: &mut Self::Context) -> Status {
        self.0.react(event, ctx)
    }
}

impl StatusPolicy for SelPolicy {
    const STATUS: Status = Status::Failure;
}

impl StatusPolicy for SeqPolicy {
    const STATUS: Status = Status::Success;
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
        $crate::Sel::with_nodes(
            vec![$(Box::new($x)),+],
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
        $crate::Seq::with_nodes(
            vec![$(Box::new($x)),+],
        )
    };
}