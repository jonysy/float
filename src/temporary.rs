use nalgebra::{Absolute, ApproxEq, Axpy, BaseNum};
use super::{Finite, FloatGuard, WithKind};

impl<N: Absolute<N>> Absolute<FloatGuard<N, Finite>> for FloatGuard<N, Finite> 
    where N: WithKind<Finite> {

    fn abs(f: &Self) -> Self {
        unsafe { FloatGuard::from_unchecked(f.as_ref().abs()) }
    }
}

impl<N: Axpy<N>> Axpy<FloatGuard<N, Finite>> for FloatGuard<N, Finite> 
    where N: WithKind<Finite> {

    #[allow(unused_variables)]
    fn axpy(&mut self, a: &Self, x: &Self) {
        unimplemented!()
    }
}

impl<N: BaseNum> BaseNum for FloatGuard<N, Finite> where N: WithKind<Finite> { }

impl<N: ApproxEq<N>> ApproxEq<FloatGuard<N, Finite>> for FloatGuard<N, Finite> 
    where FloatGuard<N, Finite>: Into<N>, N: WithKind<Finite> {

    fn approx_epsilon(unused_mut: Option<Self>) -> Self {
        match unused_mut {
            Some(f) => FloatGuard::try_from(ApproxEq::approx_epsilon(Some(f.into()))).unwrap(),
            None => FloatGuard::try_from(ApproxEq::approx_epsilon(None::<N>)).unwrap()
        }
    }

    fn approx_eq_eps(&self, other: &Self, epsilon: &Self) -> bool {
        self.as_ref().approx_eq_eps(other.as_ref(), epsilon.as_ref())
    }

    fn approx_ulps(unused_mut: Option<Self>) -> u32 {
        match unused_mut {
            Some(f) => ApproxEq::approx_ulps(Some(f.into())),
            None => ApproxEq::approx_ulps(None::<N>)
        }
    }

    fn approx_eq_ulps(&self, other: &Self, ulps: u32) -> bool {
        self.as_ref().approx_eq_ulps(other.as_ref(), ulps)
    }

    fn approx_eq(&self, other: &Self) -> bool {
        self.as_ref().approx_eq(other.as_ref())
    }
}