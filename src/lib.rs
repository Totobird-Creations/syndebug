#![feature(
    decl_macro,
    formatting_options
)]


use core::fmt::{ self, Formatter, FormattingOptions, Write };


mod wrap;
pub use wrap::*;

mod impls;

#[cfg(test)]
mod tests;


pub use syndebug_macros::SynDebug;
pub trait SynDebug {
    fn fmt(&self, f : &mut Formatter<'_>) -> fmt::Result;
}


#[inline]
pub fn to_writer<W, T>(mut writer : W, value : &T) -> fmt::Result
where
    W : Write,
    T : SynDebug
{ value.fmt(&mut Formatter::new(&mut writer, FormattingOptions::new())) }


#[inline]
pub fn to_string<T>(value : &T) -> String
where
    T : SynDebug
{
    let mut s = String::new();
    to_writer(&mut s, value).unwrap();
    s
}
