use std::marker::PhantomData;

use crate::bhv::{Bhv, Status};

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
    nodes: Vec<Box<dyn Bhv<Context = Ctx>>>,
    current: usize,
    _tag: PhantomData<Policy>,
}

impl<Ctx, Policy> List<Ctx, Policy>
where
    Policy: StatusPolicy,
{
    #[inline]
    pub(crate) fn with_nodes(nodes: Vec<Box<dyn Bhv<Context = Ctx>>>) -> Self {
        Self {
            nodes,
            current: 0,
            _tag: PhantomData,
        }
    }
}

/// A selector is a behavior node composed of a list of nodes that are run until one of them succeeds,
/// in which case the node also succeeds. If none of the nodes succeeds, this node fails.

pub struct Sel<Ctx>(pub(crate) List<Ctx, SelPolicy>);

/// A sequence is a behavior node composed of a list of nodes that are run until one of them fails,
/// in which case the node also fails. If none of the nodes fails, this node succeeds.

pub struct Seq<Ctx>(pub(crate) List<Ctx, SeqPolicy>);

impl<Ctx, Policy> Bhv for List<Ctx, Policy>
where
    Policy: StatusPolicy,
{
    type Context = Ctx;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        loop {
            if self.current >= self.nodes.len() {
                return Policy::STATUS;
            } else {
                let s = self.nodes[self.current].update(ctx);

                if s != Policy::STATUS {
                    return s;
                } else {
                    self.current += 1;
                    continue;
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

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        self.0.update(ctx)
    }

    fn reset(&mut self, _status: Status) {
        self.0.reset(_status)
    }
}

impl<Ctx> Bhv for Seq<Ctx> {
    type Context = Ctx;

    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        self.0.update(ctx)
    }

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
macro_rules! sel {
    () => {
        compile_error!("`sel` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::Sel(
            $crate::List::with_nodes(
                vec![$(Box::new($x)),+],
            ),)
    };
}

/// A macro used to create a sequence from a list of behaviors.
/// Sequences run every behavior until one of them fails.
macro_rules! seq {
    () => {
        compile_error!("`seq` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::Seq(
            $crate::List::with_nodes(
                vec![$(Box::new($x)),+],
            ),)
    };
}
