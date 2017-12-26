use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use float_traits::*;

use super::roughwrap::{roughsucc_add, roughpred_add, roughsucc_mul, roughpred_mul};

#[derive(Clone)]
pub struct RoughWrappingUnchecked<T: Abs<Output = T> + BinaryFloat + Clone>(PhantomData<fn(T)>);

impl<T: Abs<Output = T> + BinaryFloat + Clone> RoundAdd for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn add_up(a: T, b: T) -> T {
        roughsucc_add(a + b)
    }
    #[inline]
    fn add_down(a: T, b: T) -> T {
        roughpred_add(a + b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundSub for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn sub_up(a: T, b: T) -> T {
        roughsucc_add(a - b)
    }
    #[inline]
    fn sub_down(a: T, b: T) -> T {
        roughpred_add(a - b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundMul for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn mul_up(a: T, b: T) -> T {
        roughsucc_mul(a * b)
    }
    #[inline]
    fn mul_down(a: T, b: T) -> T {
        roughpred_mul(a * b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundDiv for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn div_up(a: T, b: T) -> T {
        roughsucc_mul(a / b)
    }
    #[inline]
    fn div_down(a: T, b: T) -> T {
        roughpred_mul(a / b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Sqrt<Output = T> + Clone> RoundSqrt
    for RoughWrappingUnchecked<T> {
    #[inline]
    fn sqrt_up(a: T) -> T {
        roughsucc_mul(a.sqrt())
    }
    #[inline]
    fn sqrt_down(a: T) -> T {
        roughpred_mul(a.sqrt())
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundedSession
    for RoughWrappingUnchecked<T> {
    type Num = T;
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::RoughWrappingUnchecked;

    type RWf64 = RoughWrappingUnchecked<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::add_up(a, b), RWf64::add_down(a, b));
            if !((a + b).is_infinite() || a != a || b != b || a + b != a + b) {
                assert!((a + b).pred() <= y && x <= (a + b).succ());
            }
        }
    }

    #[test]
    fn subtraction() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::sub_up(a, b), RWf64::sub_down(a, b));
            if !((a - b).is_infinite() || a != a || b != b || a - b != a - b) {
                assert!((a - b).pred() <= y && x <= (a - b).succ());
            }
        }
    }

    #[test]
    fn multiplication() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::mul_up(a, b), RWf64::mul_down(a, b));
            if !((a * b).is_infinite() || a != a || b != b || a * b != a * b) {
                assert!((a * b).pred() <= y && x <= (a * b).succ());
            }
        }
    }

    #[test]
    fn division() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (RWf64::div_up(a, b), RWf64::div_down(a, b));
            if !((a / b).is_infinite() || a != a || b != b || a / b != a / b) {
                assert!((a / b).pred() <= y && x <= (a / b).succ());
            }
        }
    }

    #[test]
    fn sqrt() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let a = rng.gen();
            let (x, y) = (RWf64::sqrt_up(a), RWf64::sqrt_down(a));
            if !(a.is_infinite() || a != a || a.sqrt() != a.sqrt()) {
                assert!(x <= a.sqrt().succ() && a.sqrt().pred() <= y);
            }
        }
    }
}
