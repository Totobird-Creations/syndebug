use crate::*;
use core::fmt::{ self, Debug, Formatter };


pub struct DebugAsSyn<T>(pub T)
where
    T : SynDebug;

impl<T> Debug for DebugAsSyn<T>
where
    T : SynDebug
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, false)
    }
}


pub struct DebugAsConstSyn<T>(pub T)
where
    T : SynDebug;

impl<T> Debug for DebugAsConstSyn<T>
where
    T : SynDebug
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f, true)
    }
}
