#![feature(plugin)]
#![plugin(speculate)]

extern crate float;
extern crate num;

use float::{Finite, FloatGuard};

speculate! {

    describe "A `FloatGuard` on `f64`" {

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

        describe "`num` implementations" {
            use num::{One, Zero};

            it "has an additive identity" {
                let zero: FloatGuard = FloatGuard::zero();
                let one: FloatGuard = FloatGuard::one();
                assert!(zero.is_zero());
                assert_eq!(one.is_zero(), false);
                assert_eq!(one + zero, one);
                assert_eq!(zero + one, one);
            }

            it "has a multiplicative identity" {
                let one = FloatGuard::one();
                let two = one + one;
                assert_eq!(one, 1.0);
                assert_eq!(two * one, 2.0);
                assert_eq!(one * two, 2.0);
            }
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

            it "can be compared to a regular float" {
                assert_eq!(x < 0.0, true);
                assert_eq!(x.lt(&0.0), true);
                assert_eq!(y == -1.0, false);
                assert_eq!(x.eq(&0.0), false);
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