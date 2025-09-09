#![feature(
    decl_macro,
    formatting_options
)]


use core::fmt::{ self, Formatter, FormattingOptions, Write };


mod wrap;
pub use wrap::*;

mod impls;


pub use syndebug_macros::SynDebug;
pub trait SynDebug {
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result;
}


#[inline]
pub fn to_writer<W, T>(mut writer : W, value : &T, const_like : bool) -> fmt::Result
where
    W : Write,
    T : SynDebug
{ value.fmt(&mut Formatter::new(&mut writer, FormattingOptions::new()), const_like) }


#[inline]
pub fn to_string<T>(value : &T, const_like : bool) -> String
where
    T : SynDebug
{
    let mut s = String::new();
    to_writer(&mut s, value, const_like).unwrap();
    s
}
