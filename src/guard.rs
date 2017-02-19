use core::{cmp, convert, fmt, iter, ops};
use num;
use super::{Error, Finite, Float, FpMarker, WithKind, Wrapped};

///  `FloatGuard` is a newtype with a floating-point type (such as `f32` and `f64`) as its 
/// internal type.
///
/// > A "newtype" is a tuple or struct with a single field. The terminology is borrowed 
/// > from Haskell.
/// >
/// > Newtypes are a zero-cost abstraction: they introduce a new, distinct name for an existing 
/// > type, with no runtime overhead when converting between the two types.
///
/// FYI: Float + Floaty + Pool lifeguard = FloatGuard.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FloatGuard<I = f64, M = Finite>(Wrapped<I, M>) where I: Float<M, Type = I>, M: FpMarker;

impl<I, M> FloatGuard<I, M> where I: Float<M, Type = I>, M: FpMarker {

    // pub fn cast<T>(self) -> FloatGuard<T, M> where T: WithKind<M> {
    //     unimplemented!()
    // }

    /// Returns the largest integer less than or equal to a number.
    #[inline]
    pub fn floor(self) -> Self {
        FloatGuard(self.0.floor())
    }
}

impl<V> FloatGuard<V, Finite> where V: WithKind<Finite> {

    /// Try to construct `FloatGuard` from `value`.
    pub fn try_from(value: V) -> Result<Self, Error> {
        if value.is_finite() {
            Ok(FloatGuard(value))
        } else {
            Err(Error::NotFinite)
        }
    }

    /// Construct a finite `FloatGuard` from an unchecked value.
    pub unsafe fn from_unchecked(value: V) -> Self {
        debug_assert!(value.is_finite());
        FloatGuard(value)
    }

    /// Safely add
    pub fn try_add(self, other: Self) -> Result<Self, Error> where V: ops::Add<Output = V> {
        Self::try_from(self.0 + other.0)
    }

    pub fn try_add_float(self, other: V) -> Result<Self, Error> where V: ops::Add<Output = V> {
        Self::try_from(self.0 + other)
    }

    /// Safely divide
    pub fn try_div(self, other: Self) -> Result<Self, Error> where V: ops::Div<Output = V> {
        Self::try_from(self.0 / other.0)
    }

    pub fn try_div_float(self, other: V) -> Result<Self, Error> where V: ops::Div<Output = V> {
        Self::try_from(self.0 / other)
    }

    /// Safely multiply
    pub fn try_mul(self, other: Self) -> Result<Self, Error> where V: ops::Mul<Output = V> {
        Self::try_from(self.0 * other.0)
    }

    pub fn try_mul_float(self, other: V) -> Result<Self, Error> where V: ops::Mul<Output = V> {
        Self::try_from(self.0 * other)
    }

    /// Safely subtract
    pub fn try_sub(self, other: Self) -> Result<Self, Error> where V: ops::Sub<Output = V> {
        Self::try_from(self.0 - other.0)
    }

    pub fn try_sub_float(self, other: V) -> Result<Self, Error> where V: ops::Sub<Output = V> {
        Self::try_from(self.0 - other)
    }
}

impl<V> cmp::Eq for FloatGuard<V, Finite> where V: WithKind<Finite> { }

impl<I> cmp::PartialEq<I> for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn eq(&self, other: &I) -> bool {
        // TODO
        let unchecked = unsafe { FloatGuard::from_unchecked(*other) };
        self.eq(&unchecked)
    }
}

impl<I> cmp::PartialOrd<I> for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn partial_cmp(&self, other: &I) -> Option<cmp::Ordering> {
        // TODO
        let unchecked = unsafe { FloatGuard::from_unchecked(*other) };
        self.partial_cmp(&unchecked)
    }
}

impl<I> cmp::Ord for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl<V> convert::From<V> for FloatGuard<V, Finite> where V: WithKind<Finite> {

    fn from(value: V) -> Self {
        Self::try_from(value).unwrap()
    }
}

impl<M> convert::Into<f32> for FloatGuard<f32, M> where f32: WithKind<M>, M: FpMarker {

    fn into(self) -> f32 {
        let FloatGuard(value) = self;
        value
    }
}

impl<M> convert::Into<f64> for FloatGuard<f64, M> where f64: WithKind<M>, M: FpMarker {

    fn into(self) -> f64 {
        let FloatGuard(value) = self;
        value
    }
}

impl<I, M> fmt::Display for FloatGuard<I, M> 
    where I: Float<M, Type = I> + fmt::Display, 
          M: FpMarker {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(unused_variables)]
impl<I> iter::Step for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn step(&self, by: &Self) -> Option<Self> {
        self.try_add(*by).ok()
    }

    fn steps_between(start: &Self, end: &Self, by: &Self) -> Option<usize> {
        unimplemented!()
    }

    fn steps_between_by_one(start: &Self, end: &Self) -> Option<usize> {
        unimplemented!()
    }

    fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    fn replace_one(&mut self) -> Self {
        unimplemented!()
    }

    fn replace_zero(&mut self) -> Self {
        unimplemented!()
    }

    fn add_one(&self) -> Self {
        self.try_add_float(I::one()).unwrap()
    }

    fn sub_one(&self) -> Self {
        self.try_sub_float(I::one()).unwrap()
    }
}

impl<I> num::One for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn one() -> Self {
        FloatGuard(I::one())
    }
}

impl<I> num::Zero for FloatGuard<I, Finite> where I: WithKind<Finite> {

    fn zero() -> Self {
        FloatGuard(I::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<I> ops::Add for FloatGuard<I, Finite> where I: WithKind<Finite> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.try_add(rhs).expect("`add` resulted in a non-finite value")
    }
}

impl<'a, I> ops::Add for &'a FloatGuard<I, Finite> where I: WithKind<Finite> {

    type Output = FloatGuard<I, Finite>;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(*rhs).unwrap()
    }
}

impl<I> ops::Div for FloatGuard<I, Finite> where I: WithKind<Finite> {

    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.try_div(rhs).unwrap()
    }
}

impl<I> ops::Mul for FloatGuard<I, Finite> where I: WithKind<Finite> {

    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.try_mul(rhs).unwrap()
    }
}

impl<I> ops::Sub for FloatGuard<I, Finite> where I: WithKind<Finite> {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.try_sub(rhs).unwrap()
    }
}