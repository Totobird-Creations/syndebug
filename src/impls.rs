use crate::*;
use core::fmt::{ self, Formatter };
use core::num::{
    NonZero,
    ZeroablePrimitive
};
use std::borrow::Cow;


macro impl_syndebug_for_tuples {
    ( $first:ident $( , $next:ident )* $(,)? ) => {
        impl_syndebug_for_tuple!( $first $( , $next )* , );
        impl_syndebug_for_tuples!( $( $next , )* );
    },
    ( $(,)? ) => { }
}
macro impl_syndebug_for_tuple( $( $generics:ident ),* $(,)? ) {
    impl< $( $generics , )* > SynDebug for ( $( $generics , )* )
    where $( $generics : SynDebug , )*
    {
        fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
            write!(f, "( ")?;
            #[allow(non_snake_case)]
            let ( $( $generics , )* ) = self;
            $(
                SynDebug::fmt($generics, f, const_like)?;
                write!(f, ", ")?;
            )*
            write!(f, ")")?;
            Ok(())
        }
    }
}
impl_syndebug_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);


macro impl_syndebug_for_debug( $ty:ty $(,)? ) {
    impl SynDebug for $ty {
        #[inline]
        fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }
}
impl_syndebug_for_debug!(());


macro impl_syndebug_for_debug_suffixed( $ty:ty $(,)? ) {
    impl SynDebug for $ty {
        #[inline]
        fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
            write!(f, concat!("{:?}", stringify!($ty)), self)
        }
    }
}
impl_syndebug_for_debug_suffixed!(u8);
impl_syndebug_for_debug_suffixed!(i8);
impl_syndebug_for_debug_suffixed!(u16);
impl_syndebug_for_debug_suffixed!(i16);
impl_syndebug_for_debug_suffixed!(u32);
impl_syndebug_for_debug_suffixed!(i32);
impl_syndebug_for_debug_suffixed!(u64);
impl_syndebug_for_debug_suffixed!(i64);
impl_syndebug_for_debug_suffixed!(u128);
impl_syndebug_for_debug_suffixed!(i128);
impl_syndebug_for_debug_suffixed!(f32);
impl_syndebug_for_debug_suffixed!(f64);

impl SynDebug for bool {
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        match (self) {
            true  => write!(f, "true"),
            false => write!(f, "false")
        }
    }
}


impl<T> SynDebug for [T]
where
    T : SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        write!(f, "[ ")?;
        for item in self {
            <T as SynDebug>::fmt(item, f, const_like)?;
            write!(f, ", ")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T, const N : usize> SynDebug for [T; N]
where
    T : SynDebug
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        <[T] as SynDebug>::fmt(self, f, const_like)
    }
}

impl<T> SynDebug for Vec<T>
where
    T : SynDebug
{
    #[inline(always)]
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        if (const_like) { panic!("Vec<_> does not support const_like SynDebug"); }
        write!(f, "vec!")?;
        <[T] as SynDebug>::fmt(self, f, false)
    }
}


impl SynDebug for &str {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl SynDebug for String {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        if (const_like) { panic!("String does not support const_like SynDebug"); }
        write!(f, "String::from( {self:?}, )")
    }
}


impl<T> SynDebug for &T
where
    T : SynDebug + ?Sized
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        write!(f, "&")?;
        <T as SynDebug>::fmt(*self, f, const_like)
    }
}

impl<T> SynDebug for &mut T
where
    T : SynDebug + ?Sized
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        write!(f, "&mut ")?;
        <T as SynDebug>::fmt(*self, f, const_like)
    }
}

impl<T> SynDebug for Box<T>
where
    T : SynDebug + ?Sized
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        if (const_like) { panic!("Box<_> does not support const_like SynDebug"); }
        write!(f, "Box::new( ")?;
        <T as SynDebug>::fmt(self, f, false)?;
        write!(f, ", )")?;
        Ok(())
    }
}

impl<T> SynDebug for Cow<'_, T>
where
            T                     : ToOwned + ?Sized,
    for<'l> &'l T                 : SynDebug,
            <T as ToOwned>::Owned : SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        if (const_like) {
            write!(f, "Cow::Borrowed( ")?;
            <&T as SynDebug>::fmt(&&**self, f, true)?;
            write!(f, ", )")?;
            Ok(())
        } else {
            match (self) {
                Cow::Borrowed(inner) => {
                    write!(f, "Cow::Borrowed( ")?;
                    <&T as SynDebug>::fmt(inner, f, false)?;
                    write!(f, ", )")?;
                    Ok(())
                },
                Cow::Owned(inner) => {
                    write!(f, "Cow::Owned( ")?;
                    <<T as ToOwned>::Owned as SynDebug>::fmt(inner, f, false)?;
                    write!(f, ", )")?;
                    Ok(())
                }
            }
        }
    }
}


impl<T> SynDebug for Option<T>
where
    T : SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        match (self) {
            Some(inner) => {
                write!(f, "Option::Some( ")?;
                <T as SynDebug>::fmt(inner, f, const_like)?;
                write!(f, ", )")?;
                Ok(())
            },
            None => write!(f, "Option::None")
        }
    }
}

impl<T, E> SynDebug for Result<T, E>
where
    T : SynDebug,
    E : SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        match (self) {
            Ok(inner) => {
                write!(f, "Result::Ok( ")?;
                <T as SynDebug>::fmt(inner, f, const_like)?;
                write!(f, ", )")?;
                Ok(())
            },
            Err(inner) => {
                write!(f, "Result::Err( ")?;
                <E as SynDebug>::fmt(inner, f, const_like)?;
                write!(f, ", )")?;
                Ok(())
            }
        }
    }
}

impl<T> SynDebug for NonZero<T>
where
    T : ZeroablePrimitive + SynDebug
{
    fn fmt(&self, f : &mut Formatter<'_>, const_like : bool) -> fmt::Result {
        write!(f, "NonZero::new( ")?;
        <T as SynDebug>::fmt(&self.get(), f, const_like)?;
        write!(f, ", ).unwrap()")?;
        Ok(())
    }
}

#[cfg(feature = "serde")]
impl SynDebug for serde::de::IgnoredAny {
    #[inline]
    fn fmt(&self, f : &mut Formatter<'_>, _const_like : bool) -> fmt::Result {
        write!(f, "IgnoredAny")
    }
}
