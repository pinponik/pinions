//! Pinions
//!

#![cfg_attr(feature = "no_std", no_std)]
#[cfg(feature = "no_std")]
use heapless;

mod app_trait;
mod ctx;
pub mod prelude;
mod widget;
mod window;

pub use app_trait::*;
pub use ctx::*;
pub use widget::*;
pub use window::*;

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
