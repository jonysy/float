use super::Float;

/// Type alias
pub type Wrapped<T, K> = <T as Float<K>>::Type;