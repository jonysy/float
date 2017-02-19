use core::fmt::Debug;
use core::hash::Hash;

/// A marker-trait.
pub trait FpMarker: Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd { }

/// Hope you can swim 'cause you're on your own, buddy.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Deflated;

impl FpMarker for Deflated { }

/// A finite floating point number.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Finite;

impl FpMarker for Finite { }