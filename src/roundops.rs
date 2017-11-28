use core::ops::{Add, Sub, Mul, Div};

pub trait RoundAdd {
    type Num: Add;
    fn add_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn add_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSub {
    type Num: Sub;
    fn sub_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn sub_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundMul {
    type Num: Mul;
    fn mul_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn mul_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundDiv {
    type Num: Div;
    fn div_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn div_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSqrt: RoundMul {
    fn sqrt_up(n: <Self as RoundMul>::Num) -> <Self as RoundMul>::Num;
    fn sqrt_down(n: <Self as RoundMul>::Num) -> <Self as RoundMul>::Num;
}


pub trait RoundOps<T: Add + Sub + Mul + Div>
    : RoundAdd<Num = T> + RoundSub<Num = T> + RoundMul<Num = T> + RoundDiv<Num = T>
    {
}

impl<S, T> RoundOps<T> for S
    where S: RoundAdd<Num = T> + RoundSub<Num = T> + RoundMul<Num = T> + RoundDiv<Num = T>,
          T: Add + Sub + Mul + Div
{
}
