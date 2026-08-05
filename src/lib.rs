//! Pinions
//!

#![cfg_attr(feature = "no_std", no_std)]
#[cfg(feature = "no_std")]
use heapless;

mod app_trait;
mod ctx;
mod window;

use app_trait::*;
use ctx::*;
use window::*;

static MAX: usize = core::usize::MAX;

#[cfg(not(feature = "no_std"))]
pub type Vect<T, const N: usize> = Vec<T>;

#[cfg(feature = "no_std")]
pub type Vect<T, const N: usize> = heapless::Vec<T, N>;

#[cfg(not(feature = "no_std"))]
pub type Str<const N: usize> = String;

#[cfg(feature = "no_std")]
pub type Str<const N: usize> = heapless::String<N>;
