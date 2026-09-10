//! Pinions
//!

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!(
    "Feature conflict: You cannot enable both 'std' and 'no_std' features at the same time.\
                Choose exactly one."
);

#[cfg(not(any(feature = "std", feature = "no_std")))]
compile_error!(
    "Feature missing: You must enable either 'std' or 'no_std' feature.\
                Leaving both disabled is not allowed."
);

#[cfg(feature = "no_std")]
use heapless;

mod app_trait;
mod ctx;
pub mod prelude;
mod shape;
#[cfg(feature = "std")]
mod std_run;
mod windows;

pub use app_trait::*;
pub use ctx::*;
pub use shape::*;
pub use std_run::*;
pub use windows::*;

static MAX: usize = core::usize::MAX;

#[cfg(not(feature = "no_std"))]
pub type Vect<T, const N: usize> = Vec<T>;

#[cfg(feature = "no_std")]
pub type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
pub type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
pub type Str<const N: usize> = heapless::String<N>;

pub trait ToPStr<const N: usize> {
    fn to_pstr(&self) -> Str<N>;
}

impl<const N: usize> ToPStr<N> for str {
    fn to_pstr(&self) -> Str<N> {
        let mut s = Str::<N>::new();
        s.push_str(self);
        s
    }
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub enum Flow {
    Wait,
    Poll,
}
