use std::marker::PhantomData;

use crate::{Bhv, Status};

// TODO:
// BhvExt::then specialization for Seq
// same for Sel

pub(crate) trait StatusPolicy {
    const STATUS: Status;
}

pub(crate) struct SelPolicy;

pub(crate) struct SeqPolicy;

pub(crate) struct List<Ctx, Policy>
    where
        Policy: StatusPolicy,
{
    nodes: Vec<Box<dyn Bhv<Context=Ctx>>>,
    current: usize,
    _tag: PhantomData<Policy>,
}

/// A selector is a behavior node composed of a list of nodes that are run until one of them succeeds,
/// in which case the node also succeeds. If none of the nodes succeeds, this node fails.
pub struct Sel<Ctx>(pub(crate) List<Ctx, SelPolicy>);

/// A sequence is a behavior node composed of a list of nodes that are run until one of them fails,
/// in which case the node also fails. If none of the nodes fails, this node succeeds.
pub struct Seq<Ctx>(pub(crate) List<Ctx, SeqPolicy>);

impl<Ctx> Sel<Ctx> {
    #[inline]
    pub fn with_nodes(nodes: Vec<Box<dyn Bhv<Context=Ctx>>>) -> Self {
        Self(List {
            nodes,
            current: 0,
            _tag: PhantomData,
        })
    }
}

impl<Ctx> Seq<Ctx> {
    #[inline]
    pub fn with_nodes(nodes: Vec<Box<dyn Bhv<Context=Ctx>>>) -> Self {
        Self(List {
            nodes,
            current: 0,
            _tag: PhantomData,
        })
    }
}

impl<Ctx, Policy> Bhv for List<Ctx, Policy>
    where
        Policy: StatusPolicy,
{
    type Context = Ctx;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        loop {
            if self.current >= self.nodes.len() {
                // TODO: is the reset called twice or is it necessary here?
                self.reset(Policy::STATUS);
                return Policy::STATUS;
            } else {
                let s = self.nodes[self.current].update(ctx);

                if s == Policy::STATUS {
                    self.current += 1;
                    continue;
                } else if s == Status::Running {
                    return Status::Running;
                } else {
                    self.reset(s);
                    return s;
                }
            }
        }
    }

    fn reset(&mut self, _status: Status) {
        let count = self.current.clamp(0, self.nodes.len());

        self.nodes[..count]
            .iter_mut()
            .for_each(|n| n.reset(Policy::STATUS));

        self.current = 0;
    }
}

impl<Ctx> Bhv for Sel<Ctx> {
    type Context = Ctx;

    #[inline]
    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        self.0.update(ctx)
    }

    #[inline]
    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<Ctx> Bhv for Seq<Ctx> {
    type Context = Ctx;

    #[inline]
    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        self.0.update(ctx)
    }

    #[inline]
    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
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
/// let v_other = action(|v| println!("v has more than one digit"));
///
/// let tree = sel! { v_1digit, v_other };
///
/// assert!(tree.execute(&mut 9) == true); // v only has one digit
/// // assert!(tree.execute(&mut 11) == true); // v has more than one digit
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
///     action(|_v| println!("v is greater than 10")),
/// };
///
/// assert!(tree.execute(&mut 11) == true);
/// // assert!(tree.execute(&mut 9) == false);
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