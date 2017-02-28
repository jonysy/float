#![feature(step_trait)]
#![no_std]

extern crate num;

// temporary
extern crate nalgebra;
mod temporary;

pub use self::error::Error;
pub use self::float::{Float, WithKind};
pub use self::guard::FloatGuard;
pub use self::marker::{FpMarker, Deflated, Finite};
pub use self::wrapped::Wrapped;

mod error;
mod float;
mod guard;
mod marker;
mod wrapped;