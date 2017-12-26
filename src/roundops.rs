use core::marker::PhantomData;
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

pub mod direction {
    pub trait Direction: Clone {
        type Inversed: Direction;
    }

    #[derive(Clone)]
    pub enum Upward {}

    impl Direction for Upward {
        type Inversed = Downward;
    }

    #[derive(Clone)]
    pub enum Downward {}

    impl Direction for Downward {
        type Inversed = Upward;
    }
}

#[derive(Clone)]
pub struct RoundedNum<Dir: direction::Direction, Num, Method>(Num, PhantomData<(Dir, Method)>);

macro_rules! impl_RNum_op {
    ($dir:ty, $op:ident, $rop:ident, $fn:ident, $rfn:ident) => (
        impl<N: $op, M: $rop<Num = N>> $op<RoundedNum<$dir, N, M>>
            for RoundedNum<$dir, N, M> {
            type Output = RoundedNum<$dir, N, M>;
            #[inline(always)]
            fn $fn(self, rhs: RoundedNum<$dir, N, M>) -> RoundedNum<$dir, N, M> {
                RoundedNum(M::$rfn(self.0, rhs.0), PhantomData)
            }
        }
    )
}

macro_rules! impl_RNum_sqrt {
    ($dir:ty, $rfn:ident) => (
        impl<N: Mul, M: RoundSqrt + RoundMul<Num = N>> RoundedNum<$dir, N, M> {
            #[inline(always)]
            pub fn sqrt(self) -> RoundedNum<$dir, N, M> {
                RoundedNum(M::$rfn(self.0), PhantomData)
            }
        }
    )
}

impl_RNum_op!(direction::Upward, Add, RoundAdd, add, add_up);
impl_RNum_op!(direction::Upward, Sub, RoundSub, sub, sub_up);
impl_RNum_op!(direction::Upward, Mul, RoundMul, mul, mul_up);
impl_RNum_op!(direction::Upward, Div, RoundDiv, div, div_up);
impl_RNum_op!(direction::Downward, Add, RoundAdd, add, add_down);
impl_RNum_op!(direction::Downward, Sub, RoundSub, sub, sub_down);
impl_RNum_op!(direction::Downward, Mul, RoundMul, mul, mul_down);
impl_RNum_op!(direction::Downward, Div, RoundDiv, div, div_down);
impl_RNum_sqrt!(direction::Upward, sqrt_up);
impl_RNum_sqrt!(direction::Downward, sqrt_down);

pub trait RoundedSession: Clone + Sized {
    type Num: Clone;
    #[inline]
    fn calc_with<Dir: direction::Direction>(input: Vec<Self::Num>,
                                            func: fn(Vec<RoundedNum<Dir, Self::Num, Self>>)
                                                     -> Vec<RoundedNum<Dir, Self::Num, Self>>)
                                            -> Vec<Self::Num> {
        func(input
                 .into_iter()
                 .map(|num| RoundedNum(num, PhantomData))
                 .collect::<Vec<_>>())
                .into_iter()
                .map(|e| e.0)
                .collect::<Vec<_>>()
    }
}
