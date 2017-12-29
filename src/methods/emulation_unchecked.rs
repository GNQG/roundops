use core::marker::PhantomData;

use roundops::*;
use float_traits::IEEE754Float;
use utils::safeeft::{safetwosum_branch as safetwosum, safetwoproduct_branch};
#[cfg(any(feature = "use-fma", feature = "doc"))]
use utils::safeeft::safetwoproduct_fma;
#[cfg(any(feature = "use-fma", feature = "doc"))]
use utils::fma::Fma;
use utils::FloatSuccPred;

#[derive(Clone)]
pub struct EmulationRegularUnchecked<T: IEEE754Float + Clone>(PhantomData<fn(T)>);
#[cfg(any(feature = "use-fma", feature = "doc"))]
#[derive(Clone)]
pub struct EmulationFmaUnchecked<T: IEEE754Float + Fma + Clone>(PhantomData<fn(T)>);

impl<T: IEEE754Float + Clone> RoundingMethod for EmulationRegularUnchecked<T> {
    type HostMethod = rmode::DefaultRounding;
    type Num = T;
}

#[cfg(any(feature = "use-fma", feature = "doc"))]
impl<T: IEEE754Float + Fma + Clone> RoundingMethod for EmulationFmaUnchecked<T> {
    type HostMethod = rmode::DefaultRounding;
    type Num = T;
}

macro_rules! impl_rops {
    ($bound:ident $( + $bound1:ident)+, $method:ident, $twoproduct:ident) => (
        impl<T: $($bound1+)+$bound> RoundAdd for $method<T> {
            fn add_up(a: T, b: T) -> T {
                let (x, y) = safetwosum(a, b);
                if y > T::zero() { x.succ() } else { x }
            }
            fn add_down(a: T, b: T) -> T {
                let (x, y) = safetwosum(a, b);
                if y < T::zero() { x.pred() } else { x }
            }
        }

        impl<T: $($bound1+)+$bound> RoundSub for $method<T> {
            #[inline]
            fn sub_up(a: T, b: T) -> T {
                Self::add_up(a, -b)
            }
            #[inline]
            fn sub_down(a: T, b: T) -> T {
                Self::add_down(a, -b)
            }
        }

        impl<T: $($bound1+)+$bound> RoundMul for $method<T> {
            fn mul_up(a: T, b: T) -> T {
                let (x, y) = $twoproduct(a, b);
                if y > T::zero() { x.succ() } else { x }
            }
            fn mul_down(a: T, b: T) -> T {
                let (x, y) = $twoproduct(a, b);
                if y < T::zero() { x.pred() } else { x }
            }
        }

        impl<T: $($bound1+)+$bound> RoundDiv for $method<T> {
            fn div_up(a: T, b: T) -> T {
                let (a, b) = if b < T::zero() { (-a, -b) } else { (a, b) };
                let d = a.clone() / b.clone();
                let (x, y) = $twoproduct(d.clone(), b);
                if x < a || (x == a && y > T::zero()) {
                    d.succ()
                } else {
                    d
                }
            }
            fn div_down(a: T, b: T) -> T {
                let (a, b) = if b < T::zero() { (-a, -b) } else { (a, b) };
                let d = a.clone() / b.clone();
                let (x, y) = $twoproduct(d.clone(), b);
                if x > a || (x == a && y < T::zero()) {
                    d.pred()
                } else {
                    d
                }
            }
        }

        impl<T: $($bound1+)+$bound> RoundSqrt for $method<T> {
            fn sqrt_up(a: T) -> T {
                let r = a.clone().sqrt();
                let (x, y) = $twoproduct(r.clone(), r.clone());
                if x < a || (x == a && y < T::zero()) {
                    r.succ()
                } else {
                    r
                }
            }
            fn sqrt_down(a: T) -> T {
                let r = a.clone().sqrt();
                let (x, y) = $twoproduct(r.clone(), r.clone());
                if x > a || (x == a && y > T::zero()) {
                    r.pred()
                } else {
                    r
                }
            }
        }
    )
}

impl_rops!(IEEE754Float + Clone,
           EmulationRegularUnchecked,
           safetwoproduct_branch);
#[cfg(any(feature = "use-fma", feature = "doc"))]
impl_rops!(IEEE754Float + Fma + Clone,
           EmulationFmaUnchecked,
           safetwoproduct_fma);

impl<T: IEEE754Float + Clone> RoundedSession for EmulationRegularUnchecked<T> {
    type Num = T;
}
#[cfg(any(feature = "use-fma", feature = "doc"))]
impl<T: IEEE754Float + Fma + Clone> RoundedSession for EmulationFmaUnchecked<T> {
    type Num = T;
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::EmulationRegularUnchecked;
    type Emuf64 = EmulationRegularUnchecked<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Emuf64::add_up(a, b), Emuf64::add_down(a, b));
            if !((a + b).is_infinite() || a != a || b != b || a + b != a + b) {
                assert!(y <= a + b && a + b <= x);
                assert!(x == y.succ() || x == y);
            }
        }
    }

    #[test]
    fn subtraction() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Emuf64::sub_up(a, b), Emuf64::sub_down(a, b));
            if !((a - b).is_infinite() || a != a || b != b || a - b != a - b) {
                assert!(y <= a - b && a - b <= x);
                assert!(x == y.succ() || x == y);
            }
        }
    }

    #[test]
    fn multiplication() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Emuf64::mul_up(a, b), Emuf64::mul_down(a, b));
            if !((a * b).is_infinite() || a != a || b != b || a * b != a * b) {
                assert!(y <= a * b && a * b <= x);
                assert!(x == y.succ() || x == y);
            }
        }
    }

    #[test]
    fn division() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Emuf64::div_up(a, b), Emuf64::div_down(a, b));
            if !((a / b).is_infinite() || a != a || b != b || a / b != a / b) {
                assert!(y <= a / b && a / b <= x);
                assert!(x == y.succ() || x == y);
            }
        }
    }

    #[test]
    fn sqrt() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let a = rng.gen();
            let (x, y) = (Emuf64::sqrt_up(a), Emuf64::sqrt_down(a));
            if !(a.is_infinite() || a != a || a.sqrt() != a.sqrt()) {
                assert!(y <= a.sqrt() && a.sqrt() <= x);
                assert!(x == y.succ() || x == y);
            }
        }
    }
}
