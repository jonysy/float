#![feature(plugin, test)]
#![plugin(speculate)]

extern crate float;
extern crate ordered_float;
extern crate test;

use float::FloatGuard;

speculate! {

    bench "construct an unwrapped float w/ guard 100x" |b| {
        b.iter(|| {
            for _ in 0..100 {
                // *`unwrap` has a performance cost..
                let _ = FloatGuard::try_from(2.0).unwrap();
            }
        })
    }

    bench "construct a float w/ guard using `if let` 100x" |b| {
        b.iter(|| {
            for _ in 0..100 {
                // Using `if let` (ignoring the result) instead of `unwrap` removes the cost. 
                if let Ok(f) = FloatGuard::try_from(2.0) {
                    let _ = f;
                }
            }
        })
    }

    bench "construct an unwrapped float using ordered-float 100x" |b| {
        use ordered_float::NotNaN;

        b.iter(|| {
            for _ in 0..100 {
                let _ = NotNaN::new(2.0).unwrap();
            }
        })
    }

    bench "regular float 100x" |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = 2.0;
            }
        })
    }
}