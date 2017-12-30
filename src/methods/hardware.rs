use core::marker::PhantomData;

use num_traits::Num;
use float_traits::Sqrt;

use roundops::*;

#[derive(Clone)]
pub struct Hardware<T: Num + rmode::EditRoundingMode>(PhantomData<fn(T)>);

impl<T: Num + rmode::EditRoundingMode> RoundingMethod for Hardware<T> {
    type HostMethod = rmode::Switchable;
    type Num = T;
}

impl<T: Num + rmode::EditRoundingMode> RoundAdd for Hardware<T> {
    #[inline(never)]
    fn add_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::upward();
            let r = lhs + rhs;
            T::to_nearest();
            r
        }
    }
    #[inline(never)]
    fn add_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::downward();
            let r = lhs + rhs;
            T::to_nearest();
            r
        }
    }
}

impl<T: Num + rmode::EditRoundingMode> RoundSub for Hardware<T> {
    #[inline(never)]
    fn sub_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::upward();
            let r = lhs - rhs;
            T::to_nearest();
            r
        }
    }
    #[inline(never)]
    fn sub_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::downward();
            let r = lhs - rhs;
            T::to_nearest();
            r
        }
    }
}

impl<T: Num + rmode::EditRoundingMode> RoundMul for Hardware<T> {
    #[inline(never)]
    fn mul_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::upward();
            let r = lhs * rhs;
            T::to_nearest();
            r
        }
    }
    #[inline(never)]
    fn mul_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::downward();
            let r = lhs * rhs;
            T::to_nearest();
            r
        }
    }
}

impl<T: Num + rmode::EditRoundingMode> RoundDiv for Hardware<T> {
    #[inline(never)]
    fn div_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::upward();
            let r = lhs / rhs;
            T::to_nearest();
            r
        }
    }
    #[inline(never)]
    fn div_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num {
        unsafe {
            T::downward();
            let r = lhs / rhs;
            T::to_nearest();
            r
        }
    }
}

impl<T: Num + Sqrt<Output = T> + rmode::EditRoundingMode> RoundSqrt for Hardware<T> {
    #[inline(never)]
    fn sqrt_up(lhs: Self::Num) -> Self::Num {
        unsafe {
            T::upward();
            let r = lhs.sqrt();
            T::to_nearest();
            r
        }
    }
    #[inline(never)]
    fn sqrt_down(lhs: Self::Num) -> Self::Num {
        unsafe {
            T::downward();
            let r = lhs.sqrt();
            T::to_nearest();
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::Hardware;

    type Hwrf64 = Hardware<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Hwrf64::add_up(a, b), Hwrf64::add_down(a, b));
            if !(a != a || b != b || a + b != a + b) {
                assert!(y <= a + b && a + b <= x);
                assert!(x == y.succ() || x == y);
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
            let (x, y) = (Hwrf64::sub_up(a, b), Hwrf64::sub_down(a, b));
            if !(a != a || b != b || a - b != a - b) {
                assert!(y <= a - b && a - b <= x);
                assert!(x == y.succ() || x == y);
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
            let (x, y) = (Hwrf64::mul_up(a, b), Hwrf64::mul_down(a, b));
            if !(a != a || b != b || a * b != a * b) {
                assert!(y <= a * b && a * b <= x);
                assert!(x == y.succ() || x == y);
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
            let (x, y) = (Hwrf64::div_up(a, b), Hwrf64::div_down(a, b));
            if !(a != a || b != b || a / b != a / b) {
                assert!(y <= a / b && a / b <= x);
                assert!(x == y.succ() || x == y);
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
            let (x, y) = (Hwrf64::sqrt_up(a), Hwrf64::sqrt_down(a));
            if !(a != a || a.sqrt() != a.sqrt()) {
                assert!(y <= a.sqrt() && a.sqrt() <= x);
                assert!(x == y.succ() || x == y);
            } else {
                assert!(x != x && y != y);
            }
        }
    }
}
