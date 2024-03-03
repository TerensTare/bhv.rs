use crate::old_impl::core::{Bhv, Status};

/// A node that runs its child nodes in turn until one of them completes successfully (if any), or
/// all of the children complete otherwise. If all the children return [`Status::Failure`],
/// then this node also returns [`Status::Failure`], otherwise it returns [`Status::Success`] after
/// the first child node that succeeds. If there are no failures but there are still nodes that are
/// running, this returns [`Status::Running`]. This node is similar to [`Sel`], with the difference
/// being that this node runs its children even after one of them returns [`Status::Running`],
/// instead of waiting for the next call of [`Bhv::update`].
pub struct WhenAny<C>(pub(crate) Box<[Box<dyn Bhv<Context=C>>]>);

/// A node that runs its child nodes in turn until all of them complete. If all the children return
/// [`Status::Success`], then this node also returns [`Status::Success`], otherwise it returns
/// [`Status::Failure`] after the first child node that fails. If there are no failures but there
/// is at least one node that returns [`Status::Running`], all the nodes are run and
/// [`Status::Running`] is returned. This node is similar to [`Seq`], with the difference
/// being that this node runs its children even after one of them returns [`Status::Running`],
/// instead of waiting for the next call of [`Bhv::update`].
pub struct WhenAll<C>(pub(crate) Box<[Box<dyn Bhv<Context=C>>]>);

impl<C> WhenAny<C> {
    #[inline]
    pub fn new(bhvs: Box<[Box<dyn Bhv<Context=C>>]>) -> Self {
        Self(bhvs)
    }
}

impl<C> WhenAll<C> {
    #[inline]
    pub fn new(bhvs: Box<[Box<dyn Bhv<Context=C>>]>) -> Self {
        Self(bhvs)
    }
}

impl<C> Bhv for WhenAny<C> {
    type Context = C;
    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        let mut any_running = false;

        for node in self.0.iter_mut() {
            match node.update(ctx) {
                Status::Running => any_running = true,
                Status::Failure => continue,
                Status::Success => return Status::Success,
            }
        }

        if any_running {
            Status::Running
        } else {
            Status::Failure
        }
    }
}

impl<C> Bhv for WhenAll<C> {
    type Context = C;
    fn update(&mut self, ctx: &mut Self::Context) -> Status {
        let mut any_running = false;

        for node in self.0.iter_mut() {
            match node.update(ctx) {
                Status::Running => any_running = true,
                Status::Success => continue,
                Status::Failure => return Status::Failure,
            }
        }

        if any_running {
            Status::Running
        } else {
            Status::Success
        }
    }
}

/// A macro used to create an [`WhenAny`] from a list of behaviors.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let tree = when_any! {
///     async_action(|v| Status::Running), // show still goes on
///     action(|v| *v = 20),
///     cond(|v| false), // but not from here
/// };
///
/// let mut ctx = 10;
/// tree.execute(&mut ctx);
///
/// assert_eq!(ctx, 20);
/// ```
#[macro_export]
macro_rules! when_any {
    () => {
        compile_error!("`when_any` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::WhenAny::new(
            Box::new([$(Box::new($x)),+]),
        )
    };
}

/// A macro used to create an [`WhenAll`] from a list of behaviors.
///
/// # Example
///
/// ```
/// use bhv::*;
///
/// let tree = when_all! {
///     async_action(|v| Status::Running), // show still goes on
///     action(|v| *v = 20),
///     cond(|v| false), // but not from here
///     action(|v| *v = 40),
/// };
///
/// let mut ctx = 10;
/// tree.execute(&mut ctx);
///
/// assert_eq!(ctx, 20); // value is still modified on the first time
/// ```
#[macro_export]
macro_rules! when_all {
    () => {
        compile_error!("`when_any` should have at least one argument!")
    };
    ($($x:expr),+$(,)?) => {
        $crate::WhenAll::new(
            Box::new([$(Box::new($x)),+]),
        )
    };
}