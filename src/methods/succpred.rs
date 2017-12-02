use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use utils::FloatSuccPred;
use float_traits::IEEE754Float;

pub struct SuccPred<T>(PhantomData<fn(T)>);

impl<T: IEEE754Float + Clone> RoundAdd for SuccPred<T> {
    type Num = T;
    fn add_up(a: T, b: T) -> T {
        let x = a.clone() + b.clone();
        if x == T::infinity() {
            x
        } else if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            (x).succ()
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
        } else if x == T::neg_infinity() {
            x
        } else {
            (x).pred()
        }
    }
}

impl<T: IEEE754Float + Clone> RoundSub for SuccPred<T> {
    type Num = T;
    #[inline]
    fn sub_up(a: T, b: T) -> T {
        Self::add_up(a, -b)
    }
    #[inline]
    fn sub_down(a: T, b: T) -> T {
        Self::add_down(a, -b)
    }
}

impl<T: IEEE754Float + Clone> RoundMul for SuccPred<T> {
    type Num = T;
    fn mul_up(a: T, b: T) -> T {
        let x = a.clone() * b.clone();
        if x == T::infinity() {
            x
        } else if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            (x).succ()
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
        } else if x == T::neg_infinity() {
            x
        } else {
            (x).pred()
        }
    }
}

impl<T: IEEE754Float + Clone> RoundDiv for SuccPred<T> {
    type Num = T;
    fn div_up(a: T, b: T) -> T {
        let x = a.clone() / b.clone();
        if x == T::infinity() {
            x
        } else if x == T::neg_infinity() {
            if b == T::zero() || a.abs() == T::infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            (x).succ()
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
        } else if x == T::neg_infinity() {
            x
        } else {
            (x).pred()
        }
    }
}

impl<T: IEEE754Float + Clone> RoundSqrt for SuccPred<T> {
    fn sqrt_up(a: T) -> T {
        (a.sqrt().succ())
    }
    fn sqrt_down(a: T) -> T {
        let r = a.sqrt();
        if r == T::infinity() {
            T::max_value()
        } else {
            (r).pred()
        }
    }
}

mod tests {
    #[test]
    fn addition() {
        use super::SuccPred;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPred<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::add_up(a, b), SPf64::add_down(a, b));
        assert!(y == (a + b).pred() && (a + b).succ() == x);
    }

    #[test]
    fn subtraction() {
        use super::SuccPred;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPred<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::sub_up(a, b), SPf64::sub_down(a, b));
        assert!(y == (a - b).pred() && (a - b).succ() == x);
    }

    #[test]
    fn multiplication() {
        use super::SuccPred;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPred<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::mul_up(a, b), SPf64::mul_down(a, b));
        assert!(y == (a * b).pred() && (a * b).succ() == x);
    }

    #[test]
    fn division() {
        use super::SuccPred;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPred<f64>;

        for &(a, b) in [
            (3., 123.),
            (2345.56, -74.12),
            (254634.13590234, 245.4556),
            (32.1, 123.122),
        ].iter()
        {
            let (x, y) = (SPf64::div_up(a, b), SPf64::div_down(a, b));
            assert!(y == (a / b).pred() && (a / b).succ() == x);
        }
    }

    #[test]
    fn sqrt() {
        use super::SuccPred;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPred<f64>;

        for &a in [
            3.,
            123.,
            2345.56,
            74.12,
            254634.13590234,
            245.4556,
            32.1,
            123.122,
        ].iter()
        {
            let (x, y) = (SPf64::sqrt_up(a), SPf64::sqrt_down(a));
            assert!(y == (a.sqrt().pred()) && (a.sqrt().succ()) == x);
        }
    }
}
