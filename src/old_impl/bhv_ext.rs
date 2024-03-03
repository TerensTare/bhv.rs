#[allow(unused_imports)]
use crate::old_impl::{
    core::{Bhv, Status},
    decor::*,
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
    /// assert_eq!(always_true.clone().execute(&mut 100), true);
    ///
    /// let inverse = always_true.inv();
    /// assert_eq!(inverse.execute(&mut 100), false);
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
    /// assert_eq!(always_false.clone().execute(&mut 100), false);
    ///
    /// let always_true = always_false.pass();
    /// assert_eq!(always_true.execute(&mut 100), true);
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
    /// assert_eq!(always_true.clone().execute(&mut 100), true);
    ///
    /// let always_false = always_true.fail();
    /// assert_eq!(always_false.execute(&mut 100), false);
    /// ```
    #[inline]
    fn fail(self) -> Fail<Self> {
        Fail(self)
    }

    /// Return a node that runs this node as long as the specified condition holds true.
    ///
    /// # Example
    ///
    /// ```
    /// use bhv::*;
    ///
    /// let mut v = 0;
    ///
    /// let code = seq! {
    ///     action(|v| println!("v = {}", v)),
    ///     action(|v| *v += 1),
    /// }.run_if(|v| *v < 5)
    /// .repeat_until_fail();
    ///
    /// code.execute(&mut v);
    ///
    /// assert_eq!(v, 5);
    /// ```
    #[inline]
    fn run_if<C>(self, cond: C) -> RunIf<Self, C>
        where
            C: Fn(&Self::Context) -> bool,
    {
        RunIf {
            bhv: self,
            cond,
        }
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
    /// tree.execute(&mut v);
    ///
    /// assert_eq!(v, 7);
    /// ```
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
    /// tree.execute(&mut ctx);
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
    /// tree.execute(&mut ctx);
    ///
    /// assert_eq!(ctx, 5);
    /// ```
    #[inline]
    fn repeat_until_fail(self) -> RepeatUntilFail<Self> {
        RepeatUntilFail(self)
    }
}

impl<B> BhvExt for B where B: Bhv + Sized {}
