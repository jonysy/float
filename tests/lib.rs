#![feature(plugin)]
#![plugin(speculate)]

extern crate float_guard;

use float_guard::{Finite, FloatGuard};

speculate! {
    describe "A `FloatGuard on `f64`" {
        it "can construct (finite) `Self` via a conversion" {
            let f = FloatGuard::<f64, Finite>::try_from(1.0);
            assert!(f.is_ok());
        }

        it "should fail to construct a finite float if the provided value is not finite" {
            let f1 = FloatGuard::try_from(::std::f64::INFINITY);
            assert!(f1.is_err());

            let f2 = FloatGuard::try_from(::std::f64::NAN);
            assert!(f2.is_err());

            let f3 = FloatGuard::try_from(::std::f64::NEG_INFINITY);
            assert!(f3.is_err());
        }

        describe "ordering" {
            before {
                let x = FloatGuard::from(-1.0);
                let y = FloatGuard::from(0.0);
            }

            it "can be compared to another float" {
                assert_eq!(x < y, true);
                assert_eq!(x.lt(&y), true);
                assert_eq!(x == y, false);
                assert_eq!(x.eq(&y), false);
            }

            describe "min/max usage" {
                use std::cmp;

                it "can be compared to another float using the `min` function" {

                    assert_eq!(cmp::min(x, y), x);
                }

                it "can be compared to another float using the `max` function" {

                    assert_eq!(cmp::max(x, y), y);
                }
            }
        }

        describe "stepping" {

            it "1) can iterate over a range" {
                let three = FloatGuard::try_from(3.0).unwrap();
                let five_p_size = FloatGuard::try_from(5.6).unwrap();

                let mut it = three..five_p_size;

                for x in 0..3 {
                    let x = FloatGuard::try_from(x as f64 + 3.0).unwrap();
                    assert_eq!(x, it.next().unwrap());
                }

                assert!(it.next().is_none());
            }

            it "2) can iterate over a range" {
                let three = FloatGuard::try_from(3.6).unwrap();
                let five_p_size = FloatGuard::try_from(5.6).unwrap();

                let mut it = three..five_p_size;

                for x in 0..2 {
                    let x = FloatGuard::try_from(x as f64 + 3.6).unwrap();
                    assert_eq!(x, it.next().unwrap());
                }

                assert!(it.next().is_none());
            }
        }
    }
}