use num;

pub trait Float<M: super::FpMarker>: num::Float {
    /// Simply used to get around Rust's "unused type parameter" error without using `PhantomData`.
    type Type;
}

/// Trait alias: [issue#8634](https://github.com/rust-lang/rust/issues/8634)
pub trait WithKind<M: super::FpMarker>: Float<M, Type = Self> { }
impl<F, M> WithKind<M> for F where F: Float<M, Type = F>, M: super::FpMarker { }

impl Float<super::Deflated> for f64 { type Type = f64; }
impl Float<super::Finite> for f64 { type Type = f64; }

impl Float<super::Deflated> for f32 { type Type = f32; }
impl Float<super::Finite> for f32 { type Type = f32; }