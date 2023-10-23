
//! # bhv
//! 
//! `bhv` is a library that implements a common interface for Behavior Trees and provides a set
//! of utility nodes divided into:
//! * adaptor nodes, that adapt an object into a behavior tree node,
//! * decorator nodes, that alter the behavior and result of a node,
//! * composite nodes, that several nodes at the same time.


mod adapt;
mod bhv_ext;
mod core;

mod composite;
mod decor;

pub use self::core::*;
pub use adapt::*;
pub use composite::*;
pub use decor::*;

pub use bhv_ext::BhvExt as _;