use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use float_traits::*;

#[inline]
pub fn roughsucc<T: Abs<Output = T> + BinaryFloat + Underflow + Clone>(f: T) -> T {
    f.clone() + (T::unit_underflow() + ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs()))
}

#[inline]
pub fn roughpred<T: Abs<Output = T> + BinaryFloat + Underflow + Clone>(f: T) -> T {
    f.clone() - (T::unit_underflow() + ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs()))
}

#[derive(Clone)]
pub struct RoughWrapping<
    T: Abs<Output = T> + BinaryFloat + Infinite + 
       Underflow + BoundedFloat + Clone>(PhantomData<fn(T)>);

impl<T: Abs<Output = T> + BinaryFloat + Infinite + 
       Underflow + BoundedFloat + Clone> RoundingMethod for RoughWrapping<T> {
    type HostMethod = rmode::DefaultRounding;
    type Num = T;
}

impl<T: Abs<Output = T> + BinaryFloat + Infinite + Underflow + BoundedFloat + Clone> RoundAdd
    for RoughWrapping<T> {
    fn add_up(a: T, b: T) -> T {
        let x = a.clone() + b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc(x)
        }
    }
    fn add_down(a: T, b: T) -> T {
        let x = a.clone() + b.clone();
        if x == T::infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundSub
    for RoughWrapping<T> {
    fn sub_up(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc(x)
        }
    }
    fn sub_down(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
        if x == T::infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundMul
    for RoughWrapping<T> {
    fn mul_up(a: T, b: T) -> T {
        let x = a.clone() * b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc(x)
        }
    }
    fn mul_down(a: T, b: T) -> T {
        let x = a.clone() * b.clone();
        if x == T::infinity() {
            if a == T::infinity() || b == T::infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundDiv
    for RoughWrapping<T> {
    fn div_up(a: T, b: T) -> T {
        let x = a.clone() / b.clone();
        if x == T::neg_infinity() {
            if b == T::zero() || a.abs() == T::infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc(x)
        }
    }
    fn div_down(a: T, b: T) -> T {
        let x = a.clone() / b.clone();
        if x == T::infinity() {
            if b == T::zero() || a.abs() == T::infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Sqrt<Output = T> + Clone> RoundSqrt
    for RoughWrapping<T> {
    fn sqrt_up(a: T) -> T {
        let r = a.sqrt();
/*
 *  may fail with ridicuous floating point format
 *  example:
 *  T::min_exponent() == -1 && T::bits() == 11
 *  roughsucc(T::unit_underflow().sqrt()) = sqrt(2^{-11}) is not representable
 *  as exact T and less than T::min_positive().
 *  roughsucc(r)
 */
        roughsucc(r)
    }
    fn sqrt_down(a: T) -> T {
        let r = a.sqrt();
        if r == T::infinity() {
            T::max_value()
        } else {
            roughpred(r)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone>
    RoundedSession for RoughWrapping<T> {
    type Num = T;
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::RoughWrapping;

    type RWf64 = RoughWrapping<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::add_up(a, b), RWf64::add_down(a, b));
            if !(a != a || b != b || a + b != a + b) {
                assert!((a + b).pred() <= y && x <= (a + b).succ());
            } else {
                assert!(x != x && y != y);
            }
        }
    }

    #[test]
    fn subtraction() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::sub_up(a, b), RWf64::sub_down(a, b));
            if !(a != a || b != b || a - b != a - b) {
                assert!((a - b).pred() <= y && x <= (a - b).succ());
            } else {
                assert!(x != x && y != y);
            }
        }
    }

    #[test]
    fn multiplication() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::mul_up(a, b), RWf64::mul_down(a, b));
            if !(a != a || b != b || a * b != a * b) {
                assert!((a * b).pred() <= y && x <= (a * b).succ());
            } else {
                assert!(x != x && y != y);
            }
        }
    }

    #[test]
    fn division() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::div_up(a, b), RWf64::div_down(a, b));
            if !(a != a || b != b || a / b != a / b) {
                assert!((a / b).pred() <= y && x <= (a / b).succ());
            } else {
                assert!(x != x && y != y);
            }
        }
    }

    #[test]
    fn sqrt() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let a = rng.gen();
            let (x, y) = (RWf64::sqrt_up(a), RWf64::sqrt_down(a));
            if !(a != a || a.sqrt() != a.sqrt()) {
                assert!(x <= a.sqrt().succ() && a.sqrt().pred() <= y);
            } else {
                assert!(x != x && y != y);
            }
        }
    }
}
