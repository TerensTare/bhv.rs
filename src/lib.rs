#![doc = include_str!("../README.md")]

#[cfg(feature = "events")]
mod events_impl;

#[cfg(feature = "events")]
pub use events_impl::*;

#[cfg(not(feature = "events"))]
mod old_impl;

#[cfg(not(feature = "events"))]
pub use old_impl::*;