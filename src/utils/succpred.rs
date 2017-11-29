use num_traits::{Float, One};

#[inline]
pub fn succ<T: FloatSuccPred>(a: T) -> T{
    FloatSuccPred::succ(a)
}

#[inline]
pub fn pred<T: FloatSuccPred>(a: T) -> T{
    FloatSuccPred::pred(a)
}

pub trait FloatSuccPred: Float {
    fn succ(self) -> Self;
    fn pred(self) -> Self;

    fn eps() -> Self;
    fn inveps() -> Self;
    fn two_invepsp2() -> Self;
    fn min_pos_inveps() -> Self;
    fn min_pos() -> Self;
    fn min_pos_subnormal() -> Self;
}

impl<T: Float> FloatSuccPred for T {
    fn succ(self) -> Self {
        let abs = self.abs();
        if abs >= Self::min_pos_inveps() {
            self + abs * (Self::inveps() + Self::two_invepsp2())
        } else if abs < Self::min_pos() {
            self + Self::min_pos_subnormal()
        } else {
            let c = self * Self::inveps();
            let e = (Self::inveps() + Self::two_invepsp2()) * c.abs();
            (c + e) * Self::eps()
        }
    }
    fn pred(self) -> Self {
        let abs = self.abs();
        if abs >= Self::min_pos_inveps() {
            self - abs * (Self::inveps() + Self::two_invepsp2())
        } else if abs < Self::min_pos() {
            self - Self::min_pos_subnormal()
        } else {
            let c = self * Self::inveps();
            let e = (Self::inveps() + Self::two_invepsp2()) * c.abs();
            (c - e) * Self::eps()
        }
    }

    #[inline]
    fn eps() -> Self {
        <Self as Float>::epsilon()
    }

    #[inline]
    fn inveps() -> Self {
        <Self as One>::one() / Self::eps()
    }

    #[inline]
    fn two_invepsp2() -> Self {
        (<Self as One>::one() + <Self as One>::one()) * Self::inveps() * Self::inveps()
    }

    #[inline]
    fn min_pos_inveps() -> Self {
        <Self as Float>::min_positive_value() * Self::inveps()
    }

    #[inline]
    fn min_pos() -> Self {
        <Self as Float>::min_positive_value()
    }

    #[inline]
    fn min_pos_subnormal() -> Self {
        (<Self as Float>::min_positive_value() * (<Self as One>::one() + <Self as One>::one())) *
        Self::eps()
    }
}
