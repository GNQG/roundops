use float_traits::*;

#[inline]
pub fn succ<T: FloatSuccPred>(a: T) -> T {
    a.succ()
}

#[inline]
pub fn pred<T: FloatSuccPred>(a: T) -> T {
    a.pred()
}

pub trait FloatSuccPred: Abs<Output = Self> + BinaryFloat + Underflow + Clone {
    fn succ(&self) -> Self {
        let abs = self.clone().abs();
        let two = Self::radix();
        let phi = Self::eps() / Self::radix() * (Self::one() + Self::eps());
        let min_pos_two_inveps = Self::radix() * Self::min_positive() / Self::eps();

        if abs >= min_pos_two_inveps {
            self.clone() + abs * phi
        } else if abs < Self::min_positive() {
            self.clone() + Self::min_positive() * Self::eps()
        } else {
            let c = Self::radix() / Self::eps() * self.clone();
            let e = phi * c.clone().abs();
            (c + e) / two * Self::eps()
        }
    }

    fn pred(&self) -> Self {
        let abs = self.clone().abs();
        let two = Self::radix();
        let phi = Self::eps() / Self::radix() * (Self::one() + Self::eps());
        let min_pos_two_inveps = Self::radix() * Self::min_positive() / Self::eps();

        if abs >= min_pos_two_inveps {
            self.clone() - abs * phi
        } else if abs < Self::min_positive() {
            self.clone() - Self::min_positive() * Self::eps()
        } else {
            let c = Self::radix() / Self::eps() * self.clone();
            let e = phi * c.clone().abs();
            (c - e) / two * Self::eps()
        }
    }
}

impl<T: Abs<Output = Self> + BinaryFloat + Underflow + Clone> FloatSuccPred for T {}

#[cfg(test)]
mod tests {
    use core::f64;
    use core::mem::transmute;
    use rand::{Rng, thread_rng};
    use super::FloatSuccPred;

    #[test]
    fn succpred() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let f = rng.gen::<f64>();
            let (succ, pred) = (f.succ(), f.pred());
            assert!(succ - f == f64::MIN_POSITIVE * f64::EPSILON ||
                    unsafe { transmute::<f64, u64>(succ) - transmute::<f64, u64>(f) == 1 } ||
                    (f >= f64::MAX && succ == f64::INFINITY) ||
                    f == f64::NEG_INFINITY || f != f);
            assert!(f - pred == f64::MIN_POSITIVE * f64::EPSILON ||
                    unsafe { transmute::<f64, u64>(f) - transmute::<f64, u64>(pred) == 1 } ||
                    (f <= f64::MIN && succ == f64::INFINITY) ||
                    f == f64::INFINITY || f != f);
        }
    }
}