use crate::{
    bhv::Bhv,
    decor::{Fail, Inv, Pass, Repeat, RepeatUntil},
};

pub trait BhvExt: Bhv + Sized {
    #[inline]
    fn inv(self) -> Inv<Self> {
        Inv(self)
    }

    #[inline]
    fn pass(self) -> Pass<Self> {
        Pass(self)
    }

    #[inline]
    fn fail(self) -> Fail<Self> {
        Fail(self)
    }

    #[inline]
    fn repeat(self, count: u32) -> Repeat<Self> {
        Repeat {
            bhv: self,
            count,
            current: 1,
        }
    }

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
