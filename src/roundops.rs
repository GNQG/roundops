use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div};

#[cfg(target_env = "msvc")]
pub mod rmode {
    extern "C" {
        fn _controlfp_s(current: *mut u32, new: u32, mask: u32) -> u32;
    }

    pub trait NativeRoundingMode {}

    pub enum DefaultRounding {}

    impl NativeRoundingMode for DefaultRounding {}

    pub enum Switchable {}

    impl NativeRoundingMode for Switchable {}

    pub trait EditRoundingMode: Sized {
        type RoundingState: Clone;

        fn rmode_controler() -> Result<RoundingModeControler<Self>, ()>;
        fn current_rounding_state() -> Self::RoundingState;
        unsafe fn set_rounding_state(Self::RoundingState) -> Self::RoundingState;
        unsafe fn upward() -> Self::RoundingState;
        unsafe fn downward() -> Self::RoundingState;
        unsafe fn to_nearest() -> Self::RoundingState;
        unsafe fn toward_zero() -> Self::RoundingState;
    }

    macro_rules! impl_rmode {
        ($type:ty) => (
            impl EditRoundingMode for $type {
                type RoundingState = u32;

                #[inline]
                fn rmode_controler() -> Result<RoundingModeControler<Self>, ()> {
                    Ok(RoundingModeControler { initial_state: Self::current_rounding_state() })
                }
                #[inline]
                fn current_rounding_state() -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    unsafe {
                        _controlfp_s(raw_mut, 0, 0);
                        *raw_mut
                    }
                }
                #[inline]
                unsafe fn set_rounding_state(state: u32)
                                            -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    _controlfp_s(raw_mut, state, 0x300);
                    *raw_mut
                }
                #[inline]
                unsafe fn upward() -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    _controlfp_s(raw_mut, 0x200, 0x300);
                    *raw_mut
                }
                #[inline]
                unsafe fn downward() -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    _controlfp_s(raw_mut, 0x100, 0x300);
                    *raw_mut
                }
                #[inline]
                unsafe fn to_nearest() -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    _controlfp_s(raw_mut, 0x000, 0x300);
                    *raw_mut
                }
                #[inline]
                unsafe fn toward_zero() -> Self::RoundingState {
                    let raw_mut = &mut 0u32 as *mut u32;
                    _controlfp_s(raw_mut, 0x300, 0x300);
                    *raw_mut
                }
            }
        )
    }

    impl_rmode!(f64);
    impl_rmode!(f32);

    #[derive(Debug)]
    pub struct RoundingModeControler<S: EditRoundingMode> {
        initial_state: S::RoundingState,
    }

    impl<S: EditRoundingMode> RoundingModeControler<S> {
        #[inline]
        pub fn rollback(&self) -> S::RoundingState {
            unsafe { S::set_rounding_state(self.initial_state.clone()) }
        }
        #[inline(never)]
        pub unsafe fn upward_session<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            let state = S::current_rounding_state();
            S::upward();
            let r = func();
            S::set_rounding_state(state);
            Ok(r)
        }
        #[inline(never)]
        pub unsafe fn downward_session<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            let state = S::current_rounding_state();
            S::downward();
            let r = func();
            S::set_rounding_state(state);
            Ok(r)
        }
        #[inline(never)]
        pub unsafe fn to_nearest_session<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            let state = S::current_rounding_state();
            S::to_nearest();
            let r = func();
            S::set_rounding_state(state);
            Ok(r)
        }
        #[inline(never)]
        pub unsafe fn toward_zero_session<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            let state = S::current_rounding_state();
            S::toward_zero();
            let r = func();
            S::set_rounding_state(state);
            Ok(r)
        }
        #[inline(never)]
        pub unsafe fn upward_then<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            S::upward();
            Ok(func())
        }
        #[inline(never)]
        pub unsafe fn downward_then<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            S::downward();
            Ok(func())
        }
        #[inline(never)]
        pub unsafe fn to_nearest_then<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            S::to_nearest();
            Ok(func())
        }
        #[inline(never)]
        pub unsafe fn toward_zero_then<O, F>(&mut self, func: F) -> Result<O, S::RoundingState>
            where F: Fn() -> O
        {
            S::toward_zero();
            Ok(func())
        }
    }

    impl<S: EditRoundingMode> Drop for RoundingModeControler<S> {
        fn drop(&mut self) {
            unsafe {
                let _ = S::set_rounding_state(self.initial_state.clone());
            };
        }
    }

    #[test]
    fn rf64() {
        use roundops::rmode::*;
        let mut c = f64::rmode_controler().unwrap();
        let x = 0.1;
        let y = 10.7;
        let z = 0.1;
        let w = 10.7;

        let v = vec![1., 10., 3146136.314, 6136.1346, 5367.67467, -134562.4537];
        assert!(unsafe { c.upward_session(|| v.iter().sum::<f64>()).unwrap() } >
                unsafe { c.downward_session(|| v.iter().sum::<f64>()).unwrap() });
        println!("{} > {}",
                 unsafe { c.upward_session(|| v.iter().sum::<f64>()).unwrap() },
                 unsafe { c.downward_session(|| v.iter().sum::<f64>()).unwrap() });
        assert!(unsafe { c.upward_session(|| x + y).unwrap() } >
                unsafe { c.downward_session(|| z + w).unwrap() });
    }
}

pub trait RoundingMethod {
    type HostMethod: rmode::NativeRoundingMode;
    type Num;
}

pub trait RoundAdd: RoundingMethod {
    fn add_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn add_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSub: RoundingMethod {
    fn sub_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn sub_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundMul: RoundingMethod {
    fn mul_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn mul_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundDiv: RoundingMethod {
    fn div_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn div_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSqrt: RoundMul {
    fn sqrt_up(n: Self::Num) -> Self::Num;
    fn sqrt_down(n: Self::Num) -> Self::Num;
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
impl<Dir: direction::Direction, Num, Method> RoundedNum<Dir, Num, Method> {
    #[inline(always)]
    pub fn new(num: Num) -> Self {
        RoundedNum(num, PhantomData)
    }
    #[inline(always)]
    pub fn extract(self) -> Num {
        self.0
    }
}

impl<D: direction::Direction, N: Neg<Output = N>, M> Neg for RoundedNum<D, N, M> {
    type Output = RoundedNum<D, N, M>;
    fn neg(self) -> RoundedNum<D, N, M> {
        RoundedNum(-self.0, PhantomData)
    }
}

macro_rules! impl_rnum_op {
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

macro_rules! impl_rnum_sqrt {
    ($dir:ty, $rfn:ident) => (
        impl<N: Mul, M: RoundSqrt + RoundMul<Num = N>> RoundedNum<$dir, N, M> {
            #[inline(always)]
            pub fn sqrt(self) -> RoundedNum<$dir, N, M> {
                RoundedNum(M::$rfn(self.0), PhantomData)
            }
        }
    )
}

impl_rnum_op!(direction::Upward, Add, RoundAdd, add, add_up);
impl_rnum_op!(direction::Upward, Sub, RoundSub, sub, sub_up);
impl_rnum_op!(direction::Upward, Mul, RoundMul, mul, mul_up);
impl_rnum_op!(direction::Upward, Div, RoundDiv, div, div_up);
impl_rnum_op!(direction::Downward, Add, RoundAdd, add, add_down);
impl_rnum_op!(direction::Downward, Sub, RoundSub, sub, sub_down);
impl_rnum_op!(direction::Downward, Mul, RoundMul, mul, mul_down);
impl_rnum_op!(direction::Downward, Div, RoundDiv, div, div_down);
impl_rnum_sqrt!(direction::Upward, sqrt_up);
impl_rnum_sqrt!(direction::Downward, sqrt_down);

#[macro_export]
macro_rules! rnum_init {
    (<$dir:path,$n:ty,$m:ty>,($numf:expr $(, $num:expr)+)) => (
        (RoundedNum::<$dir, $n, $m>::new($numf) $(, RoundedNum::<$dir, $n, $m>::new($num))+)
    )
}

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
