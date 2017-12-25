use core::marker::PhantomData;

use float_traits::IEEE754Float;

use roundops::*;
use utils::safeeft::{safetwosum_branch as safetwosum, safetwoproduct_branch};
#[cfg(any(feature = "use-fma", feature = "doc"))]
use utils::safeeft::safetwoproduct_fma;
#[cfg(any(feature = "use-fma", feature = "doc"))]
use utils::fma::{fma, Fma};

use utils::FloatSuccPred;

pub struct EmulationRegular<T>(PhantomData<fn(T)>);
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub struct EmulationFma<T>(PhantomData<fn(T)>);

macro_rules! impl_rops {
    ($bound:ident $(+$bound1:ident)+, $method:ident, $twoproduct:ident) => (
        impl<T: $($bound1+)+$bound> RoundAdd for $method<T> {
            type Num = T;
            fn add_up(a: T, b: T) -> T {
                let (x, y) = safetwosum(a.clone(), b.clone());
                if x == T::infinity() {
                    x
                } else if x == T::neg_infinity() {
                    if a == T::neg_infinity() || b == T::neg_infinity() {
                        x
                    } else {
                        T::min_value()
                    }
                } else {
                    if y > T::zero() { x.succ() } else { x }
                }
            }
            fn add_down(a: T, b: T) -> T {
                let (x, y) = safetwosum(a.clone(), b.clone());
                if x == T::infinity() {
                    if a == T::neg_infinity() || b == T::neg_infinity() {
                        x
                    } else {
                        T::max_value()
                    }
                } else if x == T::neg_infinity() {
                    x
                } else {
                    if y < T::zero() { x.pred() } else { x }
                }
            }
        }

        impl<T: $($bound1+)+$bound> RoundSub for $method<T> {
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

        impl<T: $($bound1+)+$bound> RoundMul for $method<T> {
            type Num = T;
            fn mul_up(a: T, b: T) -> T {
                let (x, y) = $twoproduct(a.clone(), b.clone());
                if x == T::infinity() {
                    x
                } else if x == T::neg_infinity() {
                    if a == T::neg_infinity() || b == T::neg_infinity() {
                        x
                    } else {
                        T::min_value()
                    }
                } else {
                    if x.clone().abs() > T::min_positive() / T::eps() * T::radix() {
                        if y > T::zero() { x.succ() } else { x }
                    } else {
                        use num_traits::One;
                        let (s_h, s_l) =
                            $twoproduct(a *
                                        T::radix_powi(T::bit_size() -
                                                        (T::min_exponent() /
                                                        (T::Exponent::one() + T::Exponent::one()))),
                                        b *
                                        T::radix_powi(T::bit_size() -
                                                        (T::min_exponent() /
                                                        (T::Exponent::one() + T::Exponent::one()))));
                        let t = (x.clone() *
                                T::radix_powi(T::bit_size() -
                                            (T::min_exponent() /
                                                (T::Exponent::one() + T::Exponent::one())))) *
                                T::radix_powi(T::bit_size() -
                                            (T::min_exponent() /
                                            (T::Exponent::one() + T::Exponent::one())));
                        if t < s_h || (t == s_h && s_l > T::zero()) {
                            x.succ()
                        } else {
                            x
                        }
                    }
                }
            }
            fn mul_down(a: T, b: T) -> T {
                let (x, y) = $twoproduct(a.clone(), b.clone());
                if x == T::infinity() {
                    if a == T::infinity() || b == T::infinity() {
                        // TODO: check
                        x
                    } else {
                        T::max_value()
                    }
                } else if x == T::neg_infinity() {
                    x
                } else {
                    if x.clone().abs() > T::min_positive() / T::eps() * T::radix() {
                        if y < T::zero() { x.pred() } else { x }
                    } else {
                        use num_traits::One;
                        let (s_h, s_l) =
                            $twoproduct(a *
                                        T::radix_powi(T::bit_size() -
                                                        (T::min_exponent() /
                                                        (T::Exponent::one() + T::Exponent::one()))),
                                        b *
                                        T::radix_powi(T::bit_size() -
                                                        (T::min_exponent() /
                                                        (T::Exponent::one() + T::Exponent::one()))));
                        let t = (x.clone() *
                                T::radix_powi(T::bit_size() -
                                            (T::min_exponent() /
                                                (T::Exponent::one() + T::Exponent::one())))) *
                                T::radix_powi(T::bit_size() -
                                            (T::min_exponent() /
                                            (T::Exponent::one() + T::Exponent::one())));
                        if t > s_h || (t == s_h && s_l < T::zero()) {
                            x.pred()
                        } else {
                            x
                        }
                    }
                }
            }
        }

        impl<T: $($bound1+)+$bound> RoundDiv for $method<T> {
            type Num = T;
            fn div_up(a: T, b: T) -> T {
                if a == T::zero() || b == T::zero() || a.clone().abs() == T::infinity() ||
                b.clone().abs() == T::infinity() || a != a || b != b {
                    a / b
                } else {
                    let (mut ss, mut bb) = (a, b);
                    if bb < T::zero() {
                        ss = ss * -T::one();
                        bb = bb * -T::one();
                    }
                    if ss.clone().abs() < T::min_positive() / T::eps() * T::radix() {
                        if bb.clone().abs() < T::min_positive() / T::eps() / T::eps() {
                            ss = ss * (T::radix() * T::eps() * T::eps());
                            bb = bb * (T::radix() * T::eps() * T::eps());
                        } else {
                            if ss < T::zero() {
                                return T::zero();
                            } else {
                                return T::unit_underflow();
                            }
                        }
                    }
                    let d = ss.clone() / bb.clone();
                    if d.is_infinite() {
                        if d > T::zero() { d } else { T::min_value() }
                    } else {
                        let (x, y) = $twoproduct(d.clone(), bb.clone());
                        if x < ss || (x == ss && y > T::zero()) {
                            d.succ()
                        } else {
                            d
                        }
                    }
                }
            }
            fn div_down(a: T, b: T) -> T {
                if a == T::zero() || b == T::zero() || a.clone().abs() == T::infinity() ||
                b.clone().abs() == T::infinity() || a != a || b != b {
                    a / b
                } else {
                    let (mut ss, mut bb) = (a, b);

                    if bb < T::zero() {
                        ss = ss * -T::one();
                        bb = bb * -T::one();
                    }
                    if ss.clone().abs() < T::min_positive() / T::eps() * T::radix() {
                        if bb.clone().abs() < T::min_positive() / T::eps() / T::eps() {
                            ss = ss * (T::radix() * T::eps() * T::eps());
                            bb = bb * (T::radix() * T::eps() * T::eps());
                        } else {
                            if ss < T::zero() {
                                return -T::unit_underflow();
                            } else {
                                return T::zero();
                            }
                        }
                    }
                    let d = ss.clone() / bb.clone();
                    if d.is_infinite() {
                        if d > T::zero() { T::max_value() } else { d }
                    } else {
                        let (x, y) = $twoproduct(d.clone(), bb.clone());
                        if x > ss || (x == ss && y < T::zero()) {
                            d.pred()
                        } else {
                            d
                        }
                    }
                }
            }
        }

        impl<T: $($bound1+)+$bound> RoundSqrt for $method<T> {
            fn sqrt_up(a: T) -> T {
                let r = a.clone().sqrt();
                if a < T::min_positive() / T::eps() * T::radix() {
                    let (ss, rr) = (a * (T::radix() / T::eps() * T::radix() / T::eps()),
                                    r.clone() * (T::radix() / T::eps()));
                    let (x, y) = $twoproduct(ss.clone(), rr.clone());
                    if x < ss || (x == ss && y < T::zero()) {
                        r.succ()
                    } else {
                        r
                    }
                } else {
                    let (x, y) = $twoproduct(r.clone(), r.clone());
                    if x < a || (x == a && y < T::zero()) {
                        r.succ()
                    } else {
                        r
                    }
                }
            }
            fn sqrt_down(a: T) -> T {
                let r = a.clone().sqrt();
                if a < T::min_positive() / T::eps() * T::radix() {
                    let (ss, rr) = (a * (T::radix() / T::eps() * T::radix() / T::eps()),
                                    r.clone() * T::radix() / T::eps());
                    let (x, y) = $twoproduct(ss.clone(), rr.clone());
                    if x > ss || (x == ss && y > T::zero()) {
                        r.pred()
                    } else {
                        r
                    }
                } else {
                    let (x, y) = $twoproduct(r.clone(), r.clone());
                    if x > a || (x == a && y > T::zero()) {
                        r.pred()
                    } else {
                        r
                    }
                }
            }
        }
    )
}

impl_rops!(IEEE754Float + Clone,
           EmulationRegular,
           safetwoproduct_branch);
#[cfg(any(feature = "use-fma", feature = "doc"))]
impl_rops!(IEEE754Float + Fma + Clone, EmulationFma, safetwoproduct_fma);

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::EmulationRegular;

    type Emuf64 = EmulationRegular<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (Emuf64::add_up(a, b), Emuf64::add_down(a, b));
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
            let (x, y) = (Emuf64::sub_up(a, b), Emuf64::sub_down(a, b));
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
            let (x, y) = (Emuf64::mul_up(a, b), Emuf64::mul_down(a, b));
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
            let (x, y) = (Emuf64::div_up(a, b), Emuf64::div_down(a, b));
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
            let (x, y) = (Emuf64::sqrt_up(a), Emuf64::sqrt_down(a));
            if !(a != a || a.sqrt() != a.sqrt()) {
                assert!(y <= a.sqrt() && a.sqrt() <= x);
                assert!(x == y.succ() || x == y);
            } else {
                assert!(x != x && y != y);
            }
        }
    }
}
