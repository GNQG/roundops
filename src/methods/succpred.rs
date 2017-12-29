use core::marker::PhantomData;

use roundops::*;
use utils::FloatSuccPred;
use float_traits::*;

#[derive(Clone)]
pub struct SuccPred<T: FloatSuccPred + Infinite + BoundedFloat>(PhantomData<fn(T)>);

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundingMethod for SuccPred<T> {
    type HostMethod = rmode::DefaultRounding;
    type Num = T;
}

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundAdd for SuccPred<T> {
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

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundSub for SuccPred<T> {
    fn sub_up(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
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
    fn sub_down(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
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

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundMul for SuccPred<T> {
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

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundDiv for SuccPred<T> {
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

impl<T: FloatSuccPred + Infinite + BoundedFloat + Sqrt<Output = T>> RoundSqrt
    for SuccPred<T> {
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

impl<T: FloatSuccPred + Infinite + BoundedFloat> RoundedSession for SuccPred<T> {
    type Num = T;
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::SuccPred;

    type SPf64 = SuccPred<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (SPf64::add_up(a, b), SPf64::add_down(a, b));
            if !(a != a || b != b || a + b != a + b) {
                assert!(y <= a + b && a + b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
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
            let (x, y) = (SPf64::sub_up(a, b), SPf64::sub_down(a, b));
            if !(a != a || b != b || a - b != a - b) {
                assert!(y <= a - b && a - b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
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
            let (x, y) = (SPf64::mul_up(a, b), SPf64::mul_down(a, b));
            if !(a != a || b != b || a * b != a * b) {
                assert!(y <= a * b && a * b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
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
            let (x, y) = (SPf64::div_up(a, b), SPf64::div_down(a, b));
            if !(a != a || b != b || a / b != a / b) {
                assert!(y <= a / b && a / b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
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
            let (x, y) = (SPf64::sqrt_up(a), SPf64::sqrt_down(a));
            if !(a != a || a.sqrt() != a.sqrt()) {
                assert!(y <= a.sqrt() && a.sqrt() <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            } else {
                assert!(x != x && y != y);
            }
        }
    }
}
